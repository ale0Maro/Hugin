TARGET_XTENSA := xtensa-esp32-none-elf
TARGET_X86_UEFI := x86_64-unknown-uefi
TARGET_X86_KERNEL := x86_64-unknown-none

OFFSET_ESP32 := 0x10000

XTENSA_OBJCOPY_FOUND := $(firstword $(wildcard $(HOME)/.rustup/toolchains/esp/xtensa-esp-elf/*/xtensa-esp-elf/bin/xtensa-esp-elf-objcopy))
XTENSA_OBJCOPY := $(if $(XTENSA_OBJCOPY_FOUND),$(XTENSA_OBJCOPY_FOUND),rust-objcopy)

OVMF_PATH := $(firstword $(wildcard /usr/share/edk2/ovmf/OVMF_CODE.fd /usr/share/OVMF/OVMF_CODE.fd))

.PHONY: all fmt build-esp32 flash-esp32 monitor-esp32 run-esp32 build-x86_64 run-x86_64 clean

all: build-esp32

fmt:
	@echo "Formatting workspace..."
	@cargo fmt --all
	@if git diff --quiet; then \
		echo "No formatting changes to save."; \
	else \
		git add . ; \
		git commit -m "style: format workspace with rustfmt" ; \
		FMT_HASH=$$(git rev-parse HEAD) ; \
		echo "" >> .git-blame-ignore-revs ; \
		echo "# rustfmt run" >> .git-blame-ignore-revs ; \
		echo "$$FMT_HASH" >> .git-blame-ignore-revs ; \
		git add .git-blame-ignore-revs ; \
		git commit --amend --no-edit ; \
		echo "Formatting and ignore-revs completed in a single commit!"; \
	fi

build-esp32:
	cargo +esp build --package kernel --target $(TARGET_XTENSA) --release
	$(XTENSA_OBJCOPY) -O binary target/$(TARGET_XTENSA)/release/kernel target/$(TARGET_XTENSA)/release/kernel.bin

flash-esp32: build-esp32
	espflash write-bin $(OFFSET_ESP32) target/$(TARGET_XTENSA)/release/kernel.bin

monitor-esp32:
	espflash monitor

run-esp32: flash-esp32 monitor-esp32

build-x86_64:
	cargo build --package boot --target $(TARGET_X86_UEFI)
	cargo build --package kernel --target $(TARGET_X86_KERNEL)
	rust-objcopy -O binary target/$(TARGET_X86_KERNEL)/debug/kernel target/$(TARGET_X86_KERNEL)/debug/kernel.bin
	rm -rf target/esp
	mkdir -p target/esp/EFI/BOOT
	cp target/$(TARGET_X86_UEFI)/debug/boot.efi target/esp/EFI/BOOT/BOOTX64.EFI
	cp target/$(TARGET_X86_KERNEL)/debug/kernel.bin target/esp/kernel.bin

run-x86_64: build-x86_64
	@if [ -z "$(OVMF_PATH)" ]; then \
		echo "Errore: Firmware OVMF non trovato! Installa edk2-ovmf."; \
		exit 1; \
	fi
	qemu-system-x86_64 \
		-m 2G \
		-bios $(OVMF_PATH) \
		-drive format=raw,file=fat:rw:target/esp \
		-serial stdio

clean:
	cargo clean
	rm -rf target/esp