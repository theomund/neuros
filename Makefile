SHELL := /bin/sh

ISO := NeurOS.iso
OVMF := /usr/share/edk2/ovmf/OVMF_CODE.fd

$(ISO): limine kernel
	mkdir -p iso_root/EFI/BOOT
	cp -v bootloader/limine.cfg kernel/kernel.elf limine/limine-bios.sys limine/limine-bios-cd.bin limine/limine-uefi-cd.bin iso_root/
	cp -v limine/BOOTX64.EFI limine/BOOTIA32.EFI iso_root/EFI/BOOT/
	xorriso -as mkisofs -b limine-bios-cd.bin \
		-no-emul-boot -boot-load-size 4 -boot-info-table \
		--efi-boot limine-uefi-cd.bin \
		-efi-boot-part --efi-boot-image --protective-msdos-label \
		iso_root -o $(ISO)
	limine/limine bios-install $(ISO)
	rm -rf iso_root

limine:
	git clone https://github.com/limine-bootloader/limine.git --branch=v6.x-branch-binary --depth=1
	$(MAKE) -C limine

.PHONY: all
all: $(ISO)

.PHONY: clean
clean:
	$(MAKE) -C kernel clean
	rm -f $(ISO)

.PHONY: distclean
distclean: clean
	rm -rf limine

.PHONY: format
format:
	$(MAKE) -C kernel format

.PHONY: kernel
kernel:
	$(MAKE) -C kernel

.PHONY: lint
lint:
	$(MAKE) -C kernel lint

.PHONY: run
run: $(ISO)
	qemu-system-x86_64 -M q35 -m 2G -cdrom $(ISO) -boot d

.PHONY: run-uefi
run-uefi: $(ISO) $(OVMF)
	qemu-system-x86_64 -M q35 -m 2G -bios $(OVMF) -cdrom $(ISO) -boot d
