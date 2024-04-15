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
PROFILE := dev

ifeq ($(DEBUG),true)
    DEBUG_FLAGS := -s -S
endif

ifeq ($(PROFILE),dev)
    SUBDIR := debug
else
    SUBDIR := $(PROFILE)
endif

BOOTLOADER := bootloader/src/limine
BOOTLOADER_BIN := $(addprefix bootloader/src/,limine-bios.sys limine-bios-cd.bin limine-uefi-cd.bin)
BOOTLOADER_CFG := bootloader/limine.cfg
BOOTLOADER_EFI := $(addprefix bootloader/src/,BOOTX64.EFI BOOTIA32.EFI)
INIT := initrd/bin/init
INIT_SOURCE := $(shell find userland/init)
INITRD := target/initrd.tar
INITRD_SOURCE := $(shell find initrd)
ISO := target/NeurOS.iso
ISO_ROOT := target/iso_root
KERNEL := target/x86_64-unknown-none/$(SUBDIR)/kernel
KERNEL_SOURCE := $(shell find kernel)
OS := $(shell uname)
OVMF := /usr/share/edk2/ovmf/OVMF_CODE.fd
RUNTIME := $(shell command -v podman 2>/dev/null || command -v docker 2>/dev/null)
STYLE := .github/styles/RedHat
TAG := builder
TARGET := all

$(ISO): $(BOOTLOADER) $(BOOTLOADER_BIN) $(BOOTLOADER_EFI) $(KERNEL) $(INITRD)
	mkdir -p $(ISO_ROOT)/EFI/BOOT
	cp -v $(BOOTLOADER_BIN) $(BOOTLOADER_CFG) $(INITRD) $(KERNEL) $(ISO_ROOT)
	cp -v $(BOOTLOADER_EFI) $(ISO_ROOT)/EFI/BOOT/
	xorriso -as mkisofs -b limine-bios-cd.bin \
		-no-emul-boot -boot-load-size 4 -boot-info-table \
		--efi-boot limine-uefi-cd.bin \
		-efi-boot-part --efi-boot-image --protective-msdos-label \
		$(ISO_ROOT) -o $(ISO)
	$(BOOTLOADER) bios-install $(ISO)
	rm -rf $(ISO_ROOT)

$(BOOTLOADER) $(BOOTLOADER_BIN) $(BOOTLOADER_EFI):
	git submodule update --init
	$(MAKE) -C bootloader/src

$(INIT): $(INIT_SOURCE)
	cargo build --profile $(PROFILE) --package init
	mkdir -p initrd/bin
	cp target/x86_64-unknown-none/$(SUBDIR)/init initrd/bin/

$(INITRD): $(INITRD_SOURCE) $(INIT)
	tar --format ustar -c -f $(INITRD) initrd

$(KERNEL): $(KERNEL_SOURCE)
	cargo build --profile $(PROFILE) --package kernel

$(STYLE):
	vale sync

.PHONY: all
all: $(ISO)

.PHONY: clean
clean:
	cargo clean

.PHONY: container
container: image
	$(RUNTIME) run -it -v $(shell pwd):/usr/src/app:z --rm $(TAG) make $(TARGET)

.PHONY: debug
debug: $(KERNEL)
	rust-gdb -ex "file $(KERNEL)" -ex "target remote localhost:1234"

.PHONY: distclean
distclean: clean
	rm -rf $(STYLE)
	$(MAKE) -C bootloader/src clean

.PHONY: format
format:
	cargo fmt

.PHONY: image
image: setup
	$(RUNTIME) build -t $(TAG) .

.PHONY: lint
lint: $(STYLE)
	cargo clippy --profile $(PROFILE)
	hadolint Dockerfile
	vale README.md

.PHONY: run
run: $(ISO)
	qemu-system-x86_64 $(DEBUG_FLAGS) -M q35 -m 2G -cdrom $(ISO) -boot d

.PHONY: run-uefi
run-uefi: $(ISO) $(OVMF)
	qemu-system-x86_64 $(DEBUG_FLAGS) -M q35 -m 2G -bios $(OVMF) -cdrom $(ISO) -boot d

.PHONY: setup
setup:
	if [ $(OS) = "Darwin" ]; then \
		brew install docker colima
		colima start
	fi
