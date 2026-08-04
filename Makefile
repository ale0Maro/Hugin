.PHONY: all build run

all: build

build:
	cargo build --package boot --target x86_64-unknown-uefi
	cargo build --package kernel --target x86_64-unknown-none
	rust-objcopy -O binary target/x86_64-unknown-none/debug/kernel target/x86_64-unknown-none/debug/kernel.bin
	rm -rf target/esp
	mkdir -p target/esp/EFI/BOOT
	cp target/x86_64-unknown-uefi/debug/boot.efi target/esp/EFI/BOOT/BOOTX64.EFI
	cp target/x86_64-unknown-none/debug/kernel.bin target/esp/kernel.bin

run: build
	qemu-system-x86_64 \
		-m 2G \
		-bios /usr/share/edk2/ovmf/OVMF_CODE.fd \
		-drive format=raw,file=fat:rw:target/esp \
		-serial stdio