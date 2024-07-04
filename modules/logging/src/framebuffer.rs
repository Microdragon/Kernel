// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::theme;
use core::ptr::NonNull;

pub const LINE_SPACING: usize = 2;

pub struct Framebuffer {
    buffer: NonNull<u32>,
    size: usize,
    red_shift: u8,
    green_shift: u8,
    blue_shift: u8,
    width: usize,
    pitch: usize,
}

impl Framebuffer {
    pub fn new(
        buffer: NonNull<u32>,
        size: usize,
        red_shift: u8,
        green_shift: u8,
        blue_shift: u8,
        width: usize,
        pitch: usize,
    ) -> Self {
        Framebuffer {
            buffer,
            size,
            red_shift,
            green_shift,
            blue_shift,
            width,
            pitch,
        }
    }

    /// Updates the buffer address to the new memory model.
    pub fn rewire(&mut self) {
        todo!()
        // let ptr = PhysAddr::new_truncate(self.buffer.as_ptr());
        // self.buffer = physical_to_virtual(ptr).as_mut_ptr::<u32>();
    }

    /// Encodes a red, green and blue color value into a combined [`u32`].
    pub const fn encode_color(&self, (red, green, blue): (u8, u8, u8)) -> u32 {
        ((red as u32) << self.red_shift)
            | ((green as u32) << self.green_shift)
            | ((blue as u32) << self.blue_shift)
    }

    /// Sets a pixel at position x, y to the given color.
    /// The color needs to be encoded with `encode_color` first.
    pub fn set_pixel(&mut self, x: usize, y: usize, color: u32) {
        let index = (y * self.pitch) + (x * 4);
        debug_assert!(
            index < self.size,
            "Trying to write pixel outside of framebuffer"
        );

        // Safety: The position should be writable and properly aligned.
        unsafe { self.buffer.byte_add(index).write_volatile(color) };
    }

    /// Sets all pixels starting at the given y coordinates until the end of the buffer to the default background color.
    pub fn clear_pixels(&mut self, y: usize) {
        let color = self.encode_color(theme::DEFAULT_BG_COLOR);
        let overhang = self.pitch - (self.width * 4);

        let mut start = y * self.pitch;
        while start < self.size {
            for _ in 0..self.width {
                // Safety: write is valid since start is less than self.size and we move in 4 bytes steps.
                unsafe { self.buffer.byte_add(start).write_volatile(color) };
                start += 4;
            }
            start += overhang;
        }
    }

    /// Copies the given amount of pixels at index `from` to index `to`.
    pub fn copy_pixels(&mut self, from_y: usize, to_y: usize, size: usize) {
        debug_assert_ne!(from_y, to_y, "Starting and ending axis are identical");

        let overhang = self.pitch - (self.width * 4);

        let mut from = from_y * self.pitch;
        let mut to = to_y * self.pitch;

        debug_assert!(
            (from + (size * self.pitch)) < self.size,
            "Trying to read pixel outside of framebuffer"
        );
        debug_assert!(
            (to + (size * self.pitch)) < self.size,
            "Trying to write pixel outside of framebuffer"
        );

        for _ in 0..size {
            for _ in 0..self.width {
                // Safety: read and write are valid since from and two are less than self.size at all times and are moved in 4 bytes steps.
                unsafe {
                    self.buffer
                        .byte_add(to)
                        .write_volatile(self.buffer.byte_add(from).read_volatile())
                };
                from += 4;
                to += 4;
            }

            from += overhang;
            to += overhang;
        }
    }
}

unsafe impl Send for Framebuffer {}
