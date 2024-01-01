SHELL := /bin/sh

.PHONY: build
build:
	$(MAKE) -C kernel build

.PHONY: clean
clean:
	$(MAKE) -C kernel clean

.PHONY: lint
lint:
	$(MAKE) -C kernel lint
