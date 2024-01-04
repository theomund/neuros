SHELL := /bin/sh

PROFILE := dev
SUBDIR := debug
TARGET := x86_64-unknown-none

ifneq ($(PROFILE),dev)
  SUBDIR := $(PROFILE)
endif

.PHONY: all
all: kernel.elf

kernel.elf:
	cargo build --target $(TARGET) --profile $(PROFILE)
	cp target/$(TARGET)/$(SUBDIR)/kernel kernel.elf

.PHONY: clean
clean:
	cargo clean
	rm -f kernel.elf

.PHONY: format
format:
	cargo fmt

.PHONY: lint
lint:
	cargo clippy --target $(TARGET) --profile $(PROFILE)
