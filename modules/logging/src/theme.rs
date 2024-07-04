// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! # Theme for the Terminal Output
//!
//! The theme is compose out of 8 normal or dark colors and 8 bright colors.
//! Each being able to be set as either the foreground or background color.

use microdragon_interface::macros::config;

/// The default foreground color used.
pub const DEFAULT_FG_COLOR: (u8, u8, u8) = split_color(config!("theme.default_fg", 0xCCCCCC));

/// The default background color used.
pub const DEFAULT_BG_COLOR: (u8, u8, u8) = split_color(config!("theme.default_bg", 0x0C0C0C));

/// Get a normal or dark color based on the given char.
/// The char is the last digit of it's ansi escape sequence.
pub const fn get_color(c: char) -> (u8, u8, u8) {
    match c {
        '0' => split_color(config!("theme.black", 0x0C0C0C)),
        '1' => split_color(config!("theme.red", 0xC50F1E)),
        '2' => split_color(config!("theme.green", 0x13A10E)),
        '3' => split_color(config!("theme.yellow", 0xC19A00)),
        '4' => split_color(config!("theme.blue", 0x0037DA)),
        '5' => split_color(config!("theme.magenta", 0x891798)),
        '6' => split_color(config!("theme.cyan", 0x3A96DD)),
        '7' => split_color(config!("theme.white", 0xCCCCCC)),
        _ => DEFAULT_BG_COLOR,
    }
}

/// Get a bright color based on the given char.
/// The char is the last digit of it's ansi escape sequence.
pub const fn get_bright_color(c: char) -> (u8, u8, u8) {
    match c {
        '0' => split_color(config!("theme.bright_black", 0x767676)),
        '1' => split_color(config!("theme.bright_red", 0xE74855)),
        '2' => split_color(config!("theme.bright_green", 0x15C60C)),
        '3' => split_color(config!("theme.bright_yellow", 0xF9F1A5)),
        '4' => split_color(config!("theme.bright_blue", 0x3B79FF)),
        '5' => split_color(config!("theme.bright_magenta", 0xB4009F)),
        '6' => split_color(config!("theme.bright_cyan", 0x61D6D6)),
        '7' => split_color(config!("theme.bright_white", 0xF2F2F2)),
        _ => DEFAULT_FG_COLOR,
    }
}

/// Slips a 32-bit hex color value into three u8 color bytes for RGB respectively.
const fn split_color(color: u32) -> (u8, u8, u8) {
    ((color >> 16) as u8, (color >> 8) as u8, color as u8)
}
