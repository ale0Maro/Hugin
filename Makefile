.PHONY: all fmt boot kernel disk-image x86_64-qemu clean

all: x86_64-qemu

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

boot:
	cargo build -p boot --target x86_64-unknown-uefi

kernel:
	cargo build -p kernel --target x86_64-unknown-none
	rust-objcopy -O binary target/x86_64-unknown-none/debug/kernel target/x86_64-unknown-none/debug/kernel.bin

disk-image: boot kernel
	rm -f target/disk.img
	dd if=/dev/zero of=target/disk.img bs=1M count=64
	
	parted -s target/disk.img mklabel gpt
	parted -s target/disk.img mkpart ESP fat32 1MiB 100%
	parted -s target/disk.img set 1 esp on
	
	mformat -i target/disk.img@@1M -F
	
	mmd -i target/disk.img@@1M ::EFI
	mmd -i target/disk.img@@1M ::EFI/BOOT
	mcopy -i target/disk.img@@1M target/x86_64-unknown-uefi/debug/boot.efi ::EFI/BOOT/BOOTX64.EFI
	mcopy -i target/disk.img@@1M target/x86_64-unknown-none/debug/kernel.bin ::kernel.bin

x86_64-qemu: disk-image
	cp /usr/share/edk2/ovmf/OVMF_VARS.fd /tmp/OVMF_VARS.fd

	qemu-system-x86_64 \
    -m 2G \
    -drive if=pflash,format=raw,readonly=on,file=/usr/share/edk2/ovmf/OVMF_CODE.fd \
    -drive if=pflash,format=raw,file=/tmp/OVMF_VARS.fd \
    -drive format=raw,file=target/disk.img \
    -serial stdio \
    -d int,cpu_reset -D qemu.log


clean:
	cargo clean
	rm -rf target/disk.img target/esp