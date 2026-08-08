// Copyright (c) 2026 Hugin Kernel Project
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
//
//
// This file holds the definitions of GPIO pins
// and helpers to convert hex, dec, and bin values
// into this mask.

/// NOTE: 
/// On many boards, GPIO pin 2 is connected to an integrated component 
/// (such as an onboard LED). Setting this pin to output and 
/// driving it high may trigger that integrated component.
pub const GPIO_PIN_2_MASK:  u64 = 1 <<  2;

pub const GPIO_PIN_4_MASK:  u64 = 1 <<  4;

pub const GPIO_PIN_5_MASK:  u64 = 1 <<  5;

pub const GPIO_PIN_12_MASK: u64 = 1 << 12;

pub const GPIO_PIN_13_MASK: u64 = 1 << 13;

pub const GPIO_PIN_14_MASK: u64 = 1 << 14;

pub const GPIO_PIN_18_MASK: u64 = 1 << 18;

pub const GPIO_PIN_19_MASK: u64 = 1 << 19;

pub const GPIO_PIN_21_MASK: u64 = 1 << 21;

pub const GPIO_PIN_25_MASK: u64 = 1 << 25;

pub const GPIO_PIN_26_MASK: u64 = 1 << 26;

pub const GPIO_PIN_27_MASK: u64 = 1 << 27;

pub const GPIO_PIN_32_MASK: u64 = 1 << 32;

pub const GPIO_PIN_33_MASK: u64 = 1 << 33;

pub const GPIO_PIN_34_MASK: u64 = 1 << 34;

pub const GPIO_PIN_35_MASK: u64 = 1 << 35;
// TODO: Consider adding more GPIO pins


/// This function is used to match a value to its corresponding mask.
/// It takes a u32 number as input to prevent passing negative numbers, 
/// as GPIO pins cannot be negative.
/// 
/// NOTE: If the passed value is not in the mask definitions,
/// it will not be matched, and the program will fail at run-time
///     
///     _ => panic!(
///         "Error: Invalid GPIO pin. Allowed pins are: 2, 4, 5, 12, 13, 14, 18, 19, 21, 25, 26, 27, 32, 33, 34, 35."
///     ),
///
/// It is designed to be used in functions that configure GPIO pins.
/// HOW TO USE:
///     (This example use the function 'set_pin_up(pin: u32)')
/// 
///     pub fn set_pin_up(pin: u32) {
///         let pin_mask = match_mask(pin);
/// 
///         unsafe {
///             core::ptr::write_volatile((0x3FF4_4000 + 0x0008) as *mut u32, pin_mask);
///         }
///     }
/// 
/// This is only and example, the function described is different.
///
pub fn match_mask(gpio_num: u32) -> u32 {
    match gpio_num {
        2 => GPIO_PIN_2_MASK as u32,
        4 => GPIO_PIN_4_MASK as u32,
        5 => GPIO_PIN_5_MASK as u32,
        12 => GPIO_PIN_12_MASK as u32,
        13 => GPIO_PIN_13_MASK as u32,
        14 => GPIO_PIN_14_MASK as u32,
        18 => GPIO_PIN_18_MASK as u32,
        19 => GPIO_PIN_19_MASK as u32,
        21 => GPIO_PIN_21_MASK as u32,
        25 => GPIO_PIN_25_MASK as u32,
        26 => GPIO_PIN_26_MASK as u32,
        27 => GPIO_PIN_27_MASK as u32,
        32 => GPIO_PIN_32_MASK as u32,
        33 => GPIO_PIN_33_MASK as u32,
        34 => GPIO_PIN_34_MASK as u32,
        35 => GPIO_PIN_35_MASK as u32,
        _ => panic!(
            "Error: Invalid GPIO pin. Allowed pins are: 2, 4, 5, 12, 13, 14, 18, 19, 21, 25, 26, 27, 32, 33, 34, 35."
        ),
    }
}