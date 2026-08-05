#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
	#[cfg(target_arch = "x86_64")]
	uefi::println!("Boot Panic: {:?}", _info);
	loop {}
}
