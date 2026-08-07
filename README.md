# Hugin Kernel Project

## Overview
**Hugin Kernel Project** aims to simplify the orchestration and management of large networks of PCs and embedded chips by implementing a **Multi-Architecture Kernel** across all devices.

## Key Features & Goals
* **Streamline Programming:** Simplify firmware development and deployment for microcontrollers and edge devices.
* **Unified Control:** Enable effortless device management via wired connections (UART/USB) as well as wireless protocols (Wi-Fi, Bluetooth).
* **Architecture Agnostic:** Designed to run seamlessly across both x86_64 hosts and Xtensa/RISC-V embedded microcontrollers (ESP32 family).
* **Isolated Execution & Protection:** Secure kernel boundary preventing user software from corrupting system memory or flashing over core routines.

## How to test

### Setup
* **x86_64 Setup**
```bash
# Install the x86_64 targets
rustup target add x86_64-unknown-uefi x86_64-unknown-none

# Install cargo-binutils for rust-objcopy
cargo install cargo-binutils
rustup component add llvm-tools-preview

# Qemu
# Fedora
sudo dnf install qemu-system-x86 edk2-ovmf make
# Debian
sudo apt install qemu-system-x86 ovmf make
```

* **xtensa esp32 Setup**
```bash
# Install Espressif Rust toolchain (for ESP32 Xtensa target)
cargo install espup
espup install
# Install esptool and espflash
cargo install espflash
pip install esptool
```

### Compile & Run
To compile and run the project, you can use the commands defined in the `Makefile`,
**execute this in the root of the project**.

> **Note:** These commands may change over time as the project evolves.
* **x86_64 qemu**
```bash
# Compile
make build-x86_64

# Run
make run-x86_64
```

* **esp32**
```bash
# Compile
make build-esp32

# Flash
make run-esp32
```