#![no_std]
#![allow(unused_imports)]

extern crate alloc;

pub use alloc::string::String;
pub use alloc::vec::Vec;
pub use alloc::boxed::Box;

pub use alloc::rc::Rc;
pub use alloc::sync::Arc;

pub use alloc::collections::BTreeMap;
pub use alloc::collections::VecDeque;

#[cfg(all(target_arch = "xtensa", target_os = "none"))]
use esp32::pins::{addr, mask};

#[cfg(all(target_arch = "xtensa", target_os = "none"))]
use mask::GpioPin;

#[cfg(all(target_arch = "xtensa", target_os = "none"))]
use addr::{set_pin_direction, set_pin_high, set_pin_low, GpioDirection};