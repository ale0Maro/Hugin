.PHONY: fmt build run

all: build

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