# NeurOS - Hobbyist operating system written in Rust.
# Copyright (C) 2024 Theomund
#
# This program is free software: you can redistribute it and/or modify
# it under the terms of the GNU General Public License as published by
# the Free Software Foundation, either version 3 of the License, or
# (at your option) any later version.
#
# This program is distributed in the hope that it will be useful,
# but WITHOUT ANY WARRANTY; without even the implied warranty of
# MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
# GNU General Public License for more details.
#
# You should have received a copy of the GNU General Public License
# along with this program. If not, see <https://www.gnu.org/licenses/>.

SHELL := /bin/sh

DEBUG := false
ISO := target/NeurOS.iso
ISO_ROOT := target/iso_root
KERNEL := target/kernel.elf
LIMINE := vendor/limine/limine
LIMINE_BIN := $(addprefix vendor/limine/,limine-bios.sys limine-bios-cd.bin limine-uefi-cd.bin)
LIMINE_EFI := $(addprefix vendor/limine/,BOOTX64.EFI BOOTIA32.EFI)
OVMF := /usr/share/edk2/ovmf/OVMF_CODE.fd
PROFILE := dev
STYLE := .vale/styles/RedHat
TAG := builder
TARGET := x86_64-unknown-none

ifeq ($(PROFILE),dev)
  SUBDIR := debug
else
  SUBDIR := $(PROFILE)
endif

ifeq ($(DEBUG),true)
  DEBUG_FLAGS := -s -S
endif

$(ISO): $(LIMINE) $(LIMINE_BIN) $(LIMINE_EFI) $(KERNEL)
	mkdir -p $(ISO_ROOT)/EFI/BOOT
	cp -v bootloader/limine.cfg $(KERNEL) $(LIMINE_BIN) $(ISO_ROOT)
	cp -v $(LIMINE_EFI) $(ISO_ROOT)/EFI/BOOT/
	xorriso -as mkisofs -b limine-bios-cd.bin \
		-no-emul-boot -boot-load-size 4 -boot-info-table \
		--efi-boot limine-uefi-cd.bin \
		-efi-boot-part --efi-boot-image --protective-msdos-label \
		$(ISO_ROOT) -o $(ISO)
	$(LIMINE) bios-install $(ISO)
	rm -rf $(ISO_ROOT)

$(LIMINE) $(LIMINE_BIN) $(LIMINE_EFI):
	git submodule update --init
	$(MAKE) -C vendor/limine

$(KERNEL):
	cargo build --target $(TARGET) --profile $(PROFILE)
	cp target/$(TARGET)/$(SUBDIR)/kernel $(KERNEL)

$(STYLE):
	vale sync

.PHONY: all
all: $(ISO)

.PHONY: clean
clean:
	cargo clean

.PHONY: container
container: image
	podman run -it -v $(shell pwd):/usr/src/app:z --rm $(TAG)

.PHONY: debug
debug: $(KERNEL)
	rust-gdb -ex "file $(KERNEL)" -ex "target remote localhost:1234"

.PHONY: distclean
distclean: clean
	rm -rf .vale

.PHONY: format
format:
	cargo fmt

.PHONY: image
image:
	podman build --format docker -t $(TAG) .

.PHONY: lint
lint: $(STYLE)
	cargo clippy --target $(TARGET) --profile $(PROFILE)
	hadolint Dockerfile
	vale README.md

.PHONY: run
run: $(ISO)
	qemu-system-x86_64 $(DEBUG_FLAGS) -M q35 -m 2G -cdrom $(ISO) -boot d

.PHONY: run-uefi
run-uefi: $(ISO) $(OVMF)
	qemu-system-x86_64 $(DEBUG_FLAGS) -M q35 -m 2G -bios $(OVMF) -cdrom $(ISO) -boot d
