#![no_std]
#![allow(unused_features)]
#![feature(asm_experimental_arch)]

// TODO: Document this function
#[inline(always)]
pub fn delay(cycles: u32) {
    for _ in 0..cycles {
        unsafe {
            core::arch::asm!("nop", options(nomem, nostack, preserves_flags));
        }
    }
}