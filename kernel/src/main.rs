#![no_std]
#![no_main]

mod panic {
    mod panic;
}

#[unsafe(no_mangle)]
#[unsafe(link_section = ".text._start")]
pub unsafe extern "sysv64" fn _start(_boot_info: &'static boot::BootInfo) -> ! {

    loop {
        unsafe {
            core::arch::asm!("hlt");
        }
    }
}