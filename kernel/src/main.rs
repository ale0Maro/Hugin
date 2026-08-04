#![no_std]
#![no_main]

mod panic {
	mod panic;
}

#[unsafe(no_mangle)]
pub unsafe extern "sysv64" fn _start(_boot_info: &'static boot::BootInfo) -> ! {
	let a = 10;
	let b = 10;
	let _x = a + b;
	loop {}
}
