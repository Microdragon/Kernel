// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::escape::EscapeSequence;
use crate::framebuffer::{Framebuffer, LINE_SPACING};
use crate::position::Position;
use common::sync::{Spinlock, SyncLazy};
use core::fmt::Write;
use core::ptr::NonNull;
use microdragon_interface::framebuffer::FramebufferInfo;
use noto_sans_mono_bitmap::{
    get_raster, get_raster_width, FontWeight, RasterHeight, RasterizedChar,
};

pub static TERMINAL_OUTPUT: SyncLazy<Spinlock<TerminalOutput>> =
    SyncLazy::new(|| Spinlock::new(TerminalOutput::new()));

const BORDER_PADDING: usize = 1;
const RASTER_HEIGHT: RasterHeight = RasterHeight::Size16;

/// Logger output creating a write-only terminal based on a framebuffer.
pub struct TerminalOutput {
    framebuffer: Option<Framebuffer>,
    position: Position,
    sequence: EscapeSequence,
}

impl TerminalOutput {
    /// Creates an uninitialized terminal output.
    fn new() -> Self {
        TerminalOutput {
            framebuffer: None,
            position: Position::new(),
            sequence: EscapeSequence::new(),
        }
    }

    /// Initializes the terminal output.
    /// This can only be called onces, subsequent calls do nothing.
    pub fn init(&mut self, info: &FramebufferInfo, address: NonNull<u32>) {
        // Check if buffer is already set and do nothing if so.
        if self.framebuffer.is_some() {
            return;
        }

        let size = info.height * info.pitch;
        debug_assert!(
            info.size as u64 >= size,
            "Provided buffer size is too small"
        );

        // Create framebuffer struct,
        let mut fb = Framebuffer::new(
            address,
            size as usize,
            info.red_mask_shift,
            info.green_mask_shift,
            info.blue_mask_shift,
            info.width as usize,
            info.pitch as usize,
        );

        // clear the framebuffer
        fb.clear_pixels(0);

        // and set it in the terminal output struct.
        self.framebuffer = Some(fb);

        // Calculate the max rows and columns we have.
        self.position.set_limits(
            (info.width as usize - BORDER_PADDING * 2)
                / get_raster_width(FontWeight::Regular, RASTER_HEIGHT),
            (info.height as usize - BORDER_PADDING * 2) / (RASTER_HEIGHT.val() + LINE_SPACING),
        );
    }

    /// Calls rewire on the framebuffer, if one is available.
    pub fn rewire(&mut self) {
        if let Some(fb) = &mut self.framebuffer {
            fb.rewire();
        }
    }

    /// Writes the given [`RasterizedChar`] into the framebuffer.
    fn write_rasterized_char(&mut self, c: RasterizedChar) {
        const WIDTH: usize = get_raster_width(FontWeight::Regular, RASTER_HEIGHT);

        let Some(fb) = &mut self.framebuffer else {
            return;
        };

        for (y, row) in c.raster().iter().enumerate() {
            for (x, intensity) in row.iter().enumerate() {
                let color = fb.encode_color(self.sequence.apply_intensity(*intensity));

                fb.set_pixel(
                    BORDER_PADDING + (self.position.row() * WIDTH) + x,
                    BORDER_PADDING
                        + (self.position.column() * (RASTER_HEIGHT.val() + LINE_SPACING))
                        + y,
                    color,
                );
            }
        }

        // Update the row and make a newline if needed.
        if self.position.next() {
            self.newline();
        }
    }

    /// Makes a newline in the framebuffer.
    fn newline(&mut self) {
        if self.position.newline() {
            if let Some(fb) = &mut self.framebuffer {
                fb.copy_pixels(
                    BORDER_PADDING + RASTER_HEIGHT.val() + LINE_SPACING,
                    BORDER_PADDING,
                    self.position.max_columns() * (RASTER_HEIGHT.val() + LINE_SPACING),
                );
                fb.clear_pixels(
                    BORDER_PADDING
                        + (self.position.max_columns() * (RASTER_HEIGHT.val() + LINE_SPACING)),
                );
            }
        }
    }
}

impl Write for TerminalOutput {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        if self.framebuffer.is_none() {
            return Ok(());
        }

        for c in s.chars() {
            self.write_char(c)?;
        }

        Ok(())
    }

    fn write_char(&mut self, c: char) -> core::fmt::Result {
        if self.framebuffer.is_none() {
            return Ok(());
        }

        if self.sequence.try_process(c) {
            return Ok(());
        }

        match c {
            '\r' => {}
            '\n' => self.newline(),
            '\x1b' => self.sequence.start(),
            _ if (c as u32) < 32 => {}
            _ => {
                if let Some(c) = get_raster(c, FontWeight::Regular, RASTER_HEIGHT) {
                    self.write_rasterized_char(c)
                } else if let Some(c) = get_raster('?', FontWeight::Regular, RASTER_HEIGHT) {
                    self.write_rasterized_char(c)
                }
            }
        }
        Ok(())
    }
}
