TARGET_XTENSA := xtensa-esp32-none-elf
TARGET_X86_UEFI := x86_64-unknown-uefi
TARGET_X86_KERNEL := x86_64-unknown-none

OFFSET_ESP32 := 0x1000

OVMF_PATH := $(firstword $(wildcard /usr/share/edk2/ovmf/OVMF_CODE.fd /usr/share/OVMF/OVMF_CODE.fd))

.PHONY: all fmt build-esp32 flash-esp32 monitor-esp32 run-esp32 build-x86_64 run-x86_64 clean

build-esp32:
	cargo +esp build --package kernel --target $(TARGET_XTENSA) --release
	esptool --chip esp32 elf2image --flash-mode dio --flash-freq 40m -o target/$(TARGET_XTENSA)/release/kernel.bin target/$(TARGET_XTENSA)/release/kernel

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