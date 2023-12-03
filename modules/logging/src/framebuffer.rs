// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
use common::addr::{PhysAddr, VirtAddr};
use common::memory::physical_to_virtual;

pub const LINE_SPACING: usize = 2;

pub struct Framebuffer {
    buffer: VirtAddr,
    size: usize,
    red_shift: u8,
    green_shift: u8,
    blue_shift: u8,
    reserved_mask: u32,
    pitch: u64,
}

impl Framebuffer {
    pub fn new(
        buffer: VirtAddr,
        size: usize,
        red_shift: u8,
        green_shift: u8,
        blue_shift: u8,
        reserved_mask: u32,
        pitch: u64,
    ) -> Self {
        Framebuffer {
            buffer,
            size,
            red_shift,
            green_shift,
            blue_shift,
            reserved_mask,
            pitch,
        }
    }

    /// Updates the buffer address to the new memory model.
    pub fn rewire(&mut self) {
        let ptr = PhysAddr::new_truncate(self.buffer.as_u64());
        self.buffer = physical_to_virtual(ptr);
    }

    /// Encodes a red, green and blue color value into a combined [`u32`].
    pub const fn encode_color(&self, (red, green, blue): (u8, u8, u8)) -> u32 {
        ((red as u32) << self.red_shift)
            | ((green as u32) << self.green_shift)
            | ((blue as u32) << self.blue_shift)
            | self.reserved_mask
    }

    /// Sets a pixel at position x, y to the given color.
    /// The color needs to be encoded with `encode_color` first.
    pub fn set_pixel(&mut self, x: usize, y: usize, color: u32) {
        let index = (y * self.pitch as usize) + (x * 4);
        debug_assert!(index < self.size, "Trying to set pixel out of bounds");

        unsafe {
            (self.buffer + index)
                .as_mut_ptr::<u32>()
                .write_volatile(color)
        };
    }

    /// Sets all pixels starting at the given coordinates until the end of the buffer to the given color.
    pub fn set_pixels(&mut self, x: usize, y: usize, color: u32) {
        let start = (y * self.pitch as usize) + (x * 4);
        debug_assert!(start < self.size, "Trying to set pixel out of bounds");
        debug_assert!(
            ((self.size - start) % 4) == 0,
            "pixel range must be u32 aligned"
        );

        for offset in (start..self.size).step_by(4) {
            unsafe {
                (self.buffer + offset)
                    .as_mut_ptr::<u32>()
                    .write_volatile(color)
            };
        }
    }

    /// Copies the given amount of pixels at index `from` to index `to`.
    pub fn copy_pixels(
        &mut self,
        from_x: usize,
        from_y: usize,
        to_x: usize,
        to_y: usize,
        size: usize,
    ) {
        let from = (from_y * self.pitch as usize) + (from_x * 4);
        let to = (to_y * self.pitch as usize) + (to_x * 4);
        let size = size * self.pitch as usize;

        debug_assert!(
            (from + size) < self.size,
            "Trying to set pixel out of bounds"
        );
        debug_assert!((to + size) < self.size, "Trying to set pixel out of bounds");
        debug_assert!((size % 4) == 0, "pixel range must be u32 aligned");

        for offset in (0..size).step_by(4) {
            let color = unsafe {
                (self.buffer + from + offset)
                    .as_mut_ptr::<u32>()
                    .read_volatile()
            };

            unsafe {
                (self.buffer + to + offset)
                    .as_mut_ptr::<u32>()
                    .write_volatile(color)
            };
        }
    }
}
