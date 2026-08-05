#![no_std]
#![no_main]

mod panic {
    mod panic;
}

use uefi::boot::{self, AllocateType, MemoryType};
use uefi::mem::memory_map::MemoryMap;
use uefi::prelude::*;
use uefi::proto::media::file::{File, FileAttribute, FileInfo, FileMode, FileType};
use uefi::proto::media::fs::SimpleFileSystem;
use uefi::Status;

extern crate boot as hugin_boot;

const KERNEL_PHYS_BASE: u64 = 0x200000; // 2 MB as linker

#[entry]
fn main() -> Status {
    {
        let init_result = uefi::helpers::init();
        init_result.unwrap();
    }
    uefi::println!("Welgome to Huging Kernel Project's Bootloader");

    let memory_map =
        boot::memory_map(MemoryType::LOADER_DATA).expect("Failed to get memory map");

    let mut min_phys_addr = u64::MAX;
    let mut max_phys_addr = 0u64;
    let mut total_conventional_bytes = 0u64;

    for entry in memory_map.entries() {
        let start = entry.phys_start;
        let end = start + (entry.page_count * 4096);

        if start < min_phys_addr {
            min_phys_addr = start;
        }

        if entry.ty == MemoryType::CONVENTIONAL {
            if end > max_phys_addr {
                max_phys_addr = end;
            }
            total_conventional_bytes += entry.page_count * 4096;
        }
    }

    let sfs_handle = boot::get_handle_for_protocol::<SimpleFileSystem>()
        .expect("Failed to find SimpleFileSystem");

    let mut sfs = boot::open_protocol_exclusive::<SimpleFileSystem>(sfs_handle)
        .expect("Failed to open SimpleFileSystem");

    let mut root = sfs.open_volume().expect("Failed to open root");

    let file_handle = root
        .open(cstr16!("kernel.bin"), FileMode::Read, FileAttribute::empty())
        .expect("Failed to find kernel.bin");

    let mut regular_file = match file_handle.into_type().unwrap() {
        FileType::Regular(f) => f,
        _ => panic!("The kernel is not a regular file"),
    };

    let mut info_buffer = [0u8; 256];
    let file_info = regular_file
        .get_info::<FileInfo>(&mut info_buffer)
        .expect("Failed to get file info");

    let kernel_file_size = file_info.file_size();

    let pages_to_allocate = ((kernel_file_size + 4095) / 4096) as usize;
    let kernel_size_ram = (pages_to_allocate * 4096) as u64;

    let kernel_mem_ptr = boot::allocate_pages(
        AllocateType::Address(KERNEL_PHYS_BASE),
        MemoryType::LOADER_DATA,
        pages_to_allocate,
    )
    .expect("Page allocation at 2MB failed");

    let kernel_raw_ptr = kernel_mem_ptr.as_ptr() as *mut u8;

    let kernel_buffer = unsafe {
        core::slice::from_raw_parts_mut(kernel_raw_ptr, kernel_size_ram as usize)
    };

    let bytes_read = regular_file.read(kernel_buffer).expect("Kernel read error");

    let heap_start = KERNEL_PHYS_BASE + kernel_size_ram;
    let heap_end = max_phys_addr;

    let boot_info = hugin_boot::BootInfo {
        ram_start: min_phys_addr,
        ram_end: max_phys_addr,
        total_conventional_bytes,
        heap_start,
        heap_end,
        kernel_file_size: bytes_read as u64,
        kernel_size_ram,
    };

    uefi::println!("--- BootInfo Summary ---");
    uefi::println!("kernel_base: {:#x}", KERNEL_PHYS_BASE);
    uefi::println!("ram_start: {:#x}", boot_info.ram_start);
    uefi::println!("ram_end: {:#x}", boot_info.ram_end);
    uefi::println!("total_conventional: {} MB", boot_info.total_conventional_bytes / (1024 * 1024));
    uefi::println!("heap_start: {:#x}", boot_info.heap_start);
    uefi::println!("heap_end: {:#x}", boot_info.heap_end);
    uefi::println!("kernel_file_size: {} bytes", boot_info.kernel_file_size);
    uefi::println!("kernel_size_ram: {} bytes", boot_info.kernel_size_ram);
    uefi::println!("------------------------");

    boot::stall(core::time::Duration::from_secs(2));

    uefi::println!("Jumping...");
    let _final_memory_map = unsafe { boot::exit_boot_services(Some(MemoryType::LOADER_DATA)) };

    type KernelEntry = unsafe extern "sysv64" fn(&'static hugin_boot::BootInfo) -> !;
    let entry_point: KernelEntry = unsafe { core::mem::transmute(KERNEL_PHYS_BASE) };

    let boot_info_ptr = (KERNEL_PHYS_BASE + bytes_read as u64) as *mut hugin_boot::BootInfo;
    unsafe {
        core::ptr::write(boot_info_ptr, boot_info);
    }
    let static_boot_info = unsafe { &*boot_info_ptr };

    unsafe {
        entry_point(static_boot_info);
    }
}