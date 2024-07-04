// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

/// Provides a memory map.
#[repr(C)]
pub struct MemoryMapInfo {
    /// Pointer to the memory map buffer.
    pub memory_map: u64,

    /// The number of entries in the memory map.
    pub memory_map_count: usize,

    /// Type of the memory map.
    pub memory_map_type: MemoryMapType,
}

/// The type of the memory map.
/// Describes it's entries layout and values.
#[repr(u8)]
pub enum MemoryMapType {
    /// The memory map is an array of Limine MemmapEntry structs.
    Limine,

    /// The memory map is an array of Rust Bootloader MemoryRegion structs.
    Rust,
}

/// Information about the MMU of this system.
#[repr(C)]
pub struct MemoryInfo {
    /// How many bits a virtual address can have.
    pub virtual_address_bits: u64,

    /// How many bits a physical address can have.
    pub physical_address_bits: u64,

    /// Mask to extract the address from a page table entry.
    pub page_table_entry_address_mask: u64,

    /// The highest level of page table supported.
    pub highest_page_table_level: u8,
}
