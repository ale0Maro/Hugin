#![no_std]
#![no_main]

mod panic {
	mod panic;
}

pub fn print_byte_serial(byte: u8) {
    unsafe {
        let mut status: u8;
        loop {
            core::arch::asm!(
                "in al, dx",
                out("al") status,
                in("dx") 0x3FDu16,
                options(nomem, nostack, preserves_flags)
            );
            if (status & 0x20) != 0 {
                break;
            }
        }

        core::arch::asm!(
            "out dx, al",
            in("dx") 0x3F8u16,
            in("al") byte,
            options(nomem, nostack, preserves_flags)
        );
    }
}


#[unsafe(no_mangle)]
pub unsafe extern "sysv64" fn _start(_boot_info: &'static boot::BootInfo) -> ! {
	let a = 10;
	let b = 10;
	let _x = a + b;
	loop {}
}
