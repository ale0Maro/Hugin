#![no_std]
#![no_main]
#![feature(asm_experimental_arch)]

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

// TODO: Move this logic
#[inline(always)]
pub fn delay(cycles: u32) {
    for _ in 0..cycles {
        unsafe {
            core::arch::asm!("nop", options(nomem, nostack, preserves_flags));
        }
    }
}

#[unsafe(no_mangle)]
#[unsafe(link_section = ".text._start")]
#[cfg(all(target_arch = "xtensa", target_os = "none"))]
pub extern "C" fn _start() -> ! {
    unsafe {
        core::ptr::write_volatile(0x3FF4_9040 as *mut u32, 2 << 12);
        core::ptr::write_volatile((0x3FF4_4000 + 0x0024) as *mut u32, 1 << 4);
    }

    loop {
        unsafe {
            // LED HIGH
            core::ptr::write_volatile((0x3FF4_4000 + 0x0008) as *mut u32, 1 << 4);
        }
        delay(500_000);

        unsafe {
            // LED LOW
            core::ptr::write_volatile((0x3FF4_4000 + 0x000C) as *mut u32, 1 << 4);
        }
        delay(500_000);
    }

}