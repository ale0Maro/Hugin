#![no_std]
#![no_main]
#![allow(unused_features)]
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
        core::ptr::write_volatile(0x3FF4_904C as *mut u32, 2 << 12); // GPIO 4
        core::ptr::write_volatile(0x3FF4_9050 as *mut u32, 2 << 12); // GPIO 5
        core::ptr::write_volatile(0x3FF4_9070 as *mut u32, 2 << 12); // GPIO 18 

        let gpio_enable_w1ts = (0x3FF4_4000 + 0x0024) as *mut u32;
        core::ptr::write_volatile(gpio_enable_w1ts, (1 << 4) | (1 << 5) | (1 << 18));
    }

    loop {
        let pins = [4, 5, 18];

        for &pin in &pins {
            unsafe {
                // LED HIGH
                core::ptr::write_volatile((0x3FF4_4000 + 0x0008) as *mut u32, 1 << pin);
            }
            delay(500_000);

            unsafe {
                // LED LOW
                core::ptr::write_volatile((0x3FF4_4000 + 0x000C) as *mut u32, 1 << pin);
            }
            delay(500_000);
        }
    }
}