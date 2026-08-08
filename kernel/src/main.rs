#![no_std]
#![no_main]
#![allow(unused_features)]
#![feature(asm_experimental_arch)]

#[cfg(target_arch = "x86_64")]
use kernel::*;

mod panic {
	mod panic;
}

#[unsafe(no_mangle)]
#[unsafe(link_section = ".text._start")]
pub extern "C" fn _start(_boot_info: &'static boot::BootInfo) -> ! {
    #[cfg(target_arch = "x86_64")]{
        mm::bump::bump::ALLOCATOR.init(_boot_info);

        let mut my_vec: Vec<u64> = Vec::new();
        my_vec.push(100 as u64);
    }
    
    #[cfg(all(target_arch = "xtensa", target_os = "none"))]{
        unsafe {
            core::ptr::write_volatile(0x3FF4_904C as *mut u32, 2 << 12); // GPIO 4
            core::ptr::write_volatile(0x3FF4_9050 as *mut u32, 2 << 12); // GPIO 5
            core::ptr::write_volatile(0x3FF4_9070 as *mut u32, 2 << 12); // GPIO 18 

            let gpio_enable_w1ts = (0x3FF4_4000 + 0x0024) as *mut u32;
            core::ptr::write_volatile(gpio_enable_w1ts, (1 << 4) | (1 << 5) | (1 << 18));
        }
    }

	loop {
        #[cfg(all(target_arch = "xtensa", target_os = "none"))]{

            let pins = [4, 5,18];

            for &pin in &pins {
                unsafe {
                    core::ptr::write_volatile((0x3FF4_4000 + 0x0008) as *mut u32, 1 << pin);
                }
                time::delay(500_000);

                unsafe {
                    core::ptr::write_volatile((0x3FF4_4000 + 0x000C) as *mut u32, 1 << pin);
                }
                time::delay(350_000);
            }
        }

		unsafe {
            #[cfg(target_arch = "x86_64")]{
			    core::arch::asm!("hlt");
            }
		}
	}
}