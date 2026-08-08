// Copyright (c) 2026 Hugin Kernel Project
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

/// The ESP32-DevKitV1 has 520 KB of SRAM (Static RAM).
pub const TOTAL_SRAM_BYTE: u32 = 532_480;

/// The internal SRAM is split into 3 blocks:
///
/// SRAM0 (192 KB):
/// The first 64 KB can act as a data cache for the MMU (to interface with external memory).
/// If not used as a cache, it can be read/written using BUS instructions.
///
/// Combined with SRAM1, it creates a contiguous IRAM space
/// where code is executed.
pub const INTERNAL_SRAM_0_SIZE_BYTE: u32 = 196_608;

/// SRAM1 (128 KB):
/// A hybrid block that can act as either IRAM or DRAM.
/// It is frequently used as DRAM.
pub const INTERNAL_SRAM_1_SIZE_BYTE: u32 = 131_072;

/// SRAM2 (200 KB):
/// Acts only as DRAM, used for BSS segments and the heap.
///
/// Together with SRAM1, it constitutes the DRAM address space
/// available to applications.
///
/// References:
/// - https://developer.espressif.com/blog/esp32-programmers-memory-model/
/// - https://www.scottyob.com/post/2025-02-27-esp32-memory/
pub const INTERNAL_SRAM_2_SIZE_BYTE: u32 = 204_800;

/// Reference:
/// - https://documentation.espressif.com/esp32_datasheet_en.pdf
/// Start address of the internal SRAM division 0.
///
/// # Note
/// The end address is calculated as the sum of the start address and the region size.
pub const INTERNAL_SRAM_0_ADDR: u32 = 0x4007_0000;

/// Start address of the internal SRAM division 1.
///
/// # Note
/// The end address is calculated as the sum of the start address and the region size.
pub const INTERNAL_SRAM_1_ADDR: u32 = 0x400A_0000;

/// Start address of the internal SRAM division 2.
///
/// # Note
/// The end address is calculated as the sum of the start address and the region size.
pub const INTERNAL_SRAM_2_ADDR: u32 = 0x3FFA_E000;
