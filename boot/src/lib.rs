#![no_std]

#[repr(C)]
#[derive(Debug)]
pub struct BootInfo {
	pub ram_start: u64,
	pub ram_end: u64,
	pub total_conventional_bytes: u64,
	pub heap_start: u64,
	pub heap_end: u64,
	pub kernel_file_size: u64,
	pub kernel_size_ram: u64,
}
