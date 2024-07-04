// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

extern "Rust" {
    static __md_link_init_text_start: u8;
    static __md_link_init_text_end: u8;

    static __md_link_init_rodata_start: u8;
    static __md_link_init_rodata_end: u8;

    static __md_link_init_data_start: u8;
    static __md_link_init_data_end: u8;

    static __md_link_init_cell_start: u8;
    static __md_link_init_cell_end: u8;
}
