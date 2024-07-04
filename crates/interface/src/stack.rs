// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

/// The size of the kernel's primary stacks in bytes.
pub const PRIMARY_STACK_SIZE: usize = 64 * 1024;

/// The size of the kernel's secondary stacks in bytes.
pub const SECONDARY_STACK_SIZE: usize = 16 * 1024;

/// Provides info about the kernel's stack.
#[repr(C)]
pub struct StackInfo {
    /// Pointer to the kernel's primary stack.
    pub primary_stack: u64,

    /// Pointer to the kernel's secondary stack.
    pub secondary_stack: u64,
}
