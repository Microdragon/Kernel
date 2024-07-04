// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

/// Provides info about a framebuffer.
/// A color is always 32 bits and each color component is 8 bits.
#[repr(C)]
#[derive(Debug, Default)]
pub struct FramebufferInfo {
    /// The start of the memory-mapped framebuffer.
    pub address: u64,

    /// The size of the frambuffer in bytes.
    pub size: usize,

    /// The width of the frmebuffer screen.
    pub width: u64,

    /// The height of the frmebuffer screen.
    pub height: u64,

    /// The amount of bytes that make up one row.
    pub pitch: u64,

    /// Amount to shift the 8-bit red color part by.
    pub red_mask_shift: u8,

    /// Amount to shift the 8-bit green color part by.
    pub green_mask_shift: u8,

    /// Amount to shift the 8-bit blue color part by.
    pub blue_mask_shift: u8,
}
