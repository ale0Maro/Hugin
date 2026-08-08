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

const GPIO_2_MUX_REG: usize = 0x3FF4_9040;  // IO_MUX register for GPIO 2
const GPIO_4_MUX_REG: usize = 0x3FF4_904C;  // IO_MUX register for GPIO 4
const GPIO_5_MUX_REG: usize = 0x3FF4_9050;  // IO_MUX register for GPIO 5
const GPIO_12_MUX_REG: usize = 0x3FF4_9034; // IO_MUX register for GPIO 12
const GPIO_13_MUX_REG: usize = 0x3FF4_9038; // IO_MUX register for GPIO 13
const GPIO_14_MUX_REG: usize = 0x3FF4_903C; // IO_MUX register for GPIO 14
const GPIO_18_MUX_REG: usize = 0x3FF4_9070; // IO_MUX register for GPIO 18
const GPIO_19_MUX_REG: usize = 0x3FF4_9074; // IO_MUX register for GPIO 19
const GPIO_21_MUX_REG: usize = 0x3FF4_901C; // IO_MUX register for GPIO 21
const GPIO_25_MUX_REG: usize = 0x3FF4_9024; // IO_MUX register for GPIO 25
const GPIO_26_MUX_REG: usize = 0x3FF4_9028; // IO_MUX register for GPIO 26
const GPIO_27_MUX_REG: usize = 0x3FF4_902C; // IO_MUX register for GPIO 27
const GPIO_32_MUX_REG: usize = 0x3FF4_9054; // IO_MUX register for GPIO 32
const GPIO_33_MUX_REG: usize = 0x3FF4_9058; // IO_MUX register for GPIO 33
const GPIO_34_MUX_REG: usize = 0x3FF4_905C; // IO_MUX register for GPIO 34
const GPIO_35_MUX_REG: usize = 0x3FF4_9060; // IO_MUX register for GPIO 35

/// Maps a `GpioPin` variant to its corresponding IO_MUX register address.
///
/// The ESP32's IO_MUX registers are not linearly mapped to GPIO numbers.
/// This helper function provides the correct hardware memory address for a 
/// given pin so its multiplexer properties can be configured.
///
/// # Examples
///
/// ```rust
/// use crate::pins::mask::GpioPin;
///
/// // Retrieve the IO_MUX address for GPIO 4
/// let mux_addr = match_gpio(GpioPin::Pin4);
/// assert_eq!(mux_addr, 0x3FF4_904C);
/// ```
fn match_gpio(pin: GpioPin) -> usize {
    match pin {
        GpioPin::Pin2 => GPIO_2_MUX_REG,
        GpioPin::Pin4 => GPIO_4_MUX_REG,
        GpioPin::Pin5 => GPIO_5_MUX_REG,
        GpioPin::Pin12 => GPIO_12_MUX_REG,
        GpioPin::Pin13 => GPIO_13_MUX_REG,
        GpioPin::Pin14 => GPIO_14_MUX_REG,
        GpioPin::Pin18 => GPIO_18_MUX_REG,
        GpioPin::Pin19 => GPIO_19_MUX_REG,
        GpioPin::Pin21 => GPIO_21_MUX_REG,
        GpioPin::Pin25 => GPIO_25_MUX_REG,
        GpioPin::Pin26 => GPIO_26_MUX_REG,
        GpioPin::Pin27 => GPIO_27_MUX_REG,
        GpioPin::Pin32 => GPIO_32_MUX_REG,
        GpioPin::Pin33 => GPIO_33_MUX_REG,
        GpioPin::Pin34 => GPIO_34_MUX_REG,
        GpioPin::Pin35 => GPIO_35_MUX_REG,
    }
}

/// Direction configuration for a GPIO pin.
///
/// Used to specify whether a GPIO pin should function as an input or an output.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum GpioDirection {
    Input,
    Output,
}

/// Initializes the hardware multiplexer (IO_MUX) for a specific GPIO pin.
///
/// This function prepares the pin for standard digital I/O by writing the
/// appropriate configuration value (`2 << 12`) to its IO_MUX register. 
/// It ensures the pin is properly routed internally before its direction 
/// or level is set.
///
/// # Examples
///
/// ```rust
/// use crate::pins::mask::GpioPin;
///
/// // Initialize the IO_MUX for pin 4
/// init_gpio(GpioPin::Pin4);
/// ```
fn init_gpio(pin: GpioPin) {
    let addr = match_gpio(pin);

    unsafe {
        core::ptr::write_volatile(addr as *mut u32, 2 << 12);
    }
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
    init_gpio(pin);
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