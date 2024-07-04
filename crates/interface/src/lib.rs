// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! # Module Interface
//!
//! This create is depended on by all other kernel modules to interface with information supplied by the bootloader and also other modules.
//! It is passed to the module's entrypoint, which in turn gets wired up using module runner.
//!
#![no_std]
#![deny(improper_ctypes, improper_ctypes_definitions)]

pub extern crate microdragon_macros as macros;

pub mod framebuffer;
pub mod link;
pub mod memory;
pub mod stack;

/// Interface to be used bt the different kernel modules.
#[repr(C)]
pub struct ModuleInterface {
    /// Provides info about the kernel's stacks.
    pub stack_info: stack::StackInfo,

    /// Pointer to the Root System Description Pointer (RSDP) or `0` if this system doesn't have ACPI.
    pub rsdp_address: u64,

    /// Provides a framebuffer to draw into or `None` if no framebuffer could be acquired.
    pub framebuffer_info: framebuffer::FramebufferInfo,

    /// Provides a memory map.
    pub memory_map_info: memory::MemoryMapInfo,

    /// Provides info about the MMU.
    pub memory_info: memory::MemoryInfo,
}

#[cfg(debug_assertions)]
extern "C" fn __assert_export(_: ModuleInterface) {}
