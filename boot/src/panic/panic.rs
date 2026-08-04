#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
	uefi::println!("Boot Panic: {:?}", _info);
	loop {}
}
