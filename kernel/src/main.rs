#![no_std]
#![no_main]

mod panic {
    mod panic;
}

#[unsafe(no_mangle)]
#[unsafe(link_section = ".text._start")]
#[cfg(target_arch = "x86_64")]
pub unsafe extern "sysv64" fn _start(_boot_info: &'static boot::BootInfo) -> ! {

    loop {
        unsafe {
            core::arch::asm!("hlt");
        }
    }
}

#[unsafe(no_mangle)]
#[cfg(all(target_arch = "xtensa", target_os = "none"))]
pub extern "C" fn main() -> ! {

    loop {
    }
}