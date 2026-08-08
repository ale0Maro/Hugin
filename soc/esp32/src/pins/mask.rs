// Copyright (c) 2026 Hugin Kernel Project
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
//
//
// This file holds the definitions of ESP32 GPIO pins
// and their associated bitmasks for hardware manipulation.

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

/// Represents only the valid and supported GPIO pins.
/// It is designed to be used in functions that configure GPIO pins.
/// 
/// # Compile-Time Safety
/// Passing an invalid pin is prevented at compile-time because 
/// only valid pins are defined as variants of this enum.
/// 
/// # Examples
/// 
/// ```rust
/// use crate::pins::mask::GpioPin;
/// 
/// pub fn set_pin_up(pin: GpioPin) {
///     let pin_mask = pin.mask();
/// 
///     unsafe {
///         core::ptr::write_volatile((0x3FF4_4000 + 0x0008) as *mut u32, pin_mask as u32);
///     }
/// }
/// ```
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum GpioPin {
    Pin2 = 2,
    Pin4 = 4,
    Pin5 = 5,
    Pin12 = 12,
    Pin13 = 13,
    Pin14 = 14,
    Pin18 = 18,
    Pin19 = 19,
    Pin21 = 21,
    Pin25 = 25,
    Pin26 = 26,
    Pin27 = 27,
    Pin32 = 32,
    Pin33 = 33,
    Pin34 = 34,
    Pin35 = 35,
}

impl GpioPin {
    /// Returns the bitmask associated with the pin as a `u64`.
    /// 
    /// This is evaluated at compile time to provide zero-cost abstractions
    /// when mapping pins to register operations.
    pub const fn mask(self) -> u64 {
        match self {
            Self::Pin2 => GPIO_PIN_2_MASK,
            Self::Pin4 => GPIO_PIN_4_MASK,
            Self::Pin5 => GPIO_PIN_5_MASK,
            Self::Pin12 => GPIO_PIN_12_MASK,
            Self::Pin13 => GPIO_PIN_13_MASK,
            Self::Pin14 => GPIO_PIN_14_MASK,
            Self::Pin18 => GPIO_PIN_18_MASK,
            Self::Pin19 => GPIO_PIN_19_MASK,
            Self::Pin21 => GPIO_PIN_21_MASK,
            Self::Pin25 => GPIO_PIN_25_MASK,
            Self::Pin26 => GPIO_PIN_26_MASK,
            Self::Pin27 => GPIO_PIN_27_MASK,
            Self::Pin32 => GPIO_PIN_32_MASK,
            Self::Pin33 => GPIO_PIN_33_MASK,
            Self::Pin34 => GPIO_PIN_34_MASK,
            Self::Pin35 => GPIO_PIN_35_MASK,
        }
    }
}