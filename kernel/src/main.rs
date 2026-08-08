// Copyright (c) 2026 Hugin Kernel Project
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![no_std]
#![no_main]
#![allow(unused_features)]
#![feature(asm_experimental_arch)]

#[allow(unused_imports)]
use kernel::*;

#[cfg(all(target_arch = "xtensa", target_os = "none"))]
use esp32::pins::{addr, mask};

#[cfg(all(target_arch = "xtensa", target_os = "none"))]
use mask::GpioPin;

#[cfg(all(target_arch = "xtensa", target_os = "none"))]
use addr::{GpioDirection, set_pin_direction, set_pin_high, set_pin_low};

mod panic {
	mod panic;
}

#[unsafe(no_mangle)]
#[unsafe(link_section = ".text._start")]
pub extern "C" fn _start(_boot_info: &'static boot::BootInfo) -> ! {
	#[cfg(target_arch = "x86_64")]
	mm::bump::bump::ALLOCATOR.init(_boot_info);
	#[cfg(all(target_arch = "xtensa", target_os = "none"))]
	mm::bump::bump::ALLOCATOR.init();

	#[cfg(all(target_arch = "xtensa", target_os = "none"))]
	{
		set_pin_direction(GpioPin::Pin4, GpioDirection::Output);
		set_pin_direction(GpioPin::Pin5, GpioDirection::Output);
		set_pin_direction(GpioPin::Pin18, GpioDirection::Output);
	}

	loop {
		#[cfg(all(target_arch = "xtensa", target_os = "none"))]
		{
			let mut pins: Vec<GpioPin> = Vec::new();
			pins.push(GpioPin::Pin4);

			for &pin in &pins {
				set_pin_high(pin);
				time::delay(500_000);

				set_pin_low(pin);
				time::delay(350_000);
			}
		}
	}
}
