#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
	log::info!("Boot Panic: {:?}", _info);
	loop {}
}
