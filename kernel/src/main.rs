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

use kernel::*;
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

    let mut my_vec: Vec<u32> = Vec::new();
    my_vec.push(100 as u32);
    
    loop {

    }
}