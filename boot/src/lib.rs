// Copyright (c) 2026 Hugin Kernel Project
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![no_std]

/// Contains critical hardware, memory layout, and environment data passed 
/// from the UEFI bootloader to the kernel during the handoff process.
/// 
/// Because the kernel assumes control after UEFI boot services are terminated, 
/// it cannot query the firmware for this information itself. This structure 
/// serves as the kernel's definitive map of physical memory and its own 
/// placement within it.
#[repr(C)]
#[derive(Debug)]
pub struct BootInfo {
    // The lowest physical memory address available on the system.
    // 
    // **Calculation:** Derived by iterating through the UEFI memory map and 
    // finding the absolute minimum `phys_start` across all memory entries.
    // **Usage:** Used by the kernel to understand the absolute floor of physical RAM.
    pub ram_start: u64,

    // The highest physical memory address of usable (conventional) memory.
    // 
    // **Calculation:** Derived by finding the maximum `phys_start + (page_count * 4096)` 
    // specifically among memory regions marked as `MemoryType::CONVENTIONAL`.
    // **Usage:** Used by the kernel's physical memory manager to establish the 
    // upper boundary of usable RAM space.
    pub ram_end: u64,

    // The total amount of usable RAM available to the operating system, in bytes.
    //
    // **Calculation:** The sum of `page_count * 4096` for all UEFI memory map 
    // entries marked as `MemoryType::CONVENTIONAL`.
    // **Usage:** Used for system statistics (e.g., displaying total RAM) and 
    // sizing internal kernel data structures like page frame arrays.
    pub total_conventional_bytes: u64,

    // The safe starting physical address for the kernel's dynamic memory allocator.
    // 
    // **Calculation:** Computed as `KERNEL_PHYS_BASE + kernel_size_ram`. It points 
    // to the first byte immediately following the loaded kernel binary.
    // **Usage:** Crucial for the initial bump allocator. It guarantees the kernel 
    // will not accidentally overwrite its own code or data when allocating memory.
    pub heap_start: u64,

    // The maximum physical address up to which the initial kernel allocator can grow.
    // 
    // **Calculation:** Set to `max_phys_addr` (same as `ram_end`).
    // **Usage:** Acts as the out-of-memory boundary for early boot allocations 
    // before a full virtual memory manager and page allocator are initialized.
    pub heap_end: u64,

    // The exact size of the kernel binary file as it was stored on the disk.
    // 
    // **Calculation:** Extracted directly from the FAT32 EFI partition using 
    // the UEFI `FileInfo` protocol before reading the file.
    // **Usage:** Mostly informational, useful for debugging and verifying that 
    // the entire file was read successfully.
    pub kernel_file_size: u64,

    // The total footprint of the kernel in physical RAM, aligned to 4KB page boundaries.
    // 
    // **Calculation:** `((kernel_file_size + 4095) / 4096) * 4096`. This rounds up 
    // the raw file size to the nearest multiple of a UEFI memory page (4KB).
    // **Usage:** Ensures that the `heap_start` begins on a clean page boundary, 
    // preventing unaligned memory access and protecting the trailing bytes of the kernel page.
    pub kernel_size_ram: u64,
}