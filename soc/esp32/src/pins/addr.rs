// Copyright (c) 2026 Hugin Kernel Project
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
//
//
// This file holds the definitions of the GPIO addresses and
// control helper functions to configure and drive GPIO pins.

use crate::pins::mask::GpioPin;

/// ESP32 GPIO base register address
const GPIO_BASE: usize = 0x3FF4_4000;

/// GPIO registers offset
const GPIO_OUT_W1TS_OFFSET: usize = 0x0008; // Set output high (Write 1 to Set)
const GPIO_OUT_W1TC_OFFSET: usize = 0x000C; // Set output low (Write 1 to Clear)
const GPIO_ENABLE_W1TS_OFFSET: usize = 0x0024; // Enable output
const GPIO_ENABLE_W1TC_OFFSET: usize = 0x0028; // Disable output (input mode)

/// Direction configuration for a GPIO pin.
///
/// Used to specify whether a GPIO pin should function as an input or an output.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum GpioDirection {
	Input,
	Output,
}

/// Sets the direction of a given GPIO pin (Input or Output).
///
/// This function interacts directly with the ESP32 GPIO enable registers
/// using volatile memory writes to configure the pin's operational mode.
///
/// # Examples
///
/// ```rust
/// use crate::pins::mask::GpioPin;
/// use crate::gpio::{set_pin_direction, GpioDirection};
///
/// // Configure pin 4 as an output
/// set_pin_direction(GpioPin::Pin4, GpioDirection::Output);
/// ```
pub fn set_pin_direction(pin: GpioPin, direction: GpioDirection) {
	let pin_mask = pin.mask() as u32;

	unsafe {
		match direction {
			GpioDirection::Output => {
				let reg = (GPIO_BASE + GPIO_ENABLE_W1TS_OFFSET) as *mut u32;
				core::ptr::write_volatile(reg, pin_mask);
			}
			GpioDirection::Input => {
				let reg = (GPIO_BASE + GPIO_ENABLE_W1TC_OFFSET) as *mut u32;
				core::ptr::write_volatile(reg, pin_mask);
			}
		}
	}
}

/// Sets the output level of a GPIO pin (`true` = High/On, `false` = Low/Off).
///
/// This function uses the W1TS (Write 1 to Set) and W1TC (Write 1 to Clear)
/// hardware features of the ESP32 to safely modify the output state.
///
/// # Examples
///
/// ```rust
/// use crate::pins::mask::GpioPin;
/// use crate::gpio::set_pin_level;
///
/// // Drive pin 4 high
/// set_pin_level(GpioPin::Pin4, true);
/// ```
pub fn set_pin_level(pin: GpioPin, high: bool) {
	let pin_mask = pin.mask() as u32;

	unsafe {
		if high {
			let reg = (GPIO_BASE + GPIO_OUT_W1TS_OFFSET) as *mut u32;
			core::ptr::write_volatile(reg, pin_mask);
		} else {
			let reg = (GPIO_BASE + GPIO_OUT_W1TC_OFFSET) as *mut u32;
			core::ptr::write_volatile(reg, pin_mask);
		}
	}
}

/// Convenience helper to turn a pin high (`true`).
///
/// # Examples
///
/// ```rust
/// use crate::pins::mask::GpioPin;
/// use crate::gpio::set_pin_high;
///
/// set_pin_high(GpioPin::Pin18);
/// ```
pub fn set_pin_high(pin: GpioPin) {
	set_pin_level(pin, true);
}

/// Convenience helper to turn a pin low (`false`).
///
/// # Examples
///
/// ```rust
/// use crate::pins::mask::GpioPin;
/// use crate::gpio::set_pin_low;
///
/// set_pin_low(GpioPin::Pin18);
/// ```
pub fn set_pin_low(pin: GpioPin) {
	set_pin_level(pin, false);
}
