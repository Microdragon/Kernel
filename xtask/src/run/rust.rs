// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::dependencies::RUST_BOOTLOADER;
use crate::utils::CommandContext;
use color_eyre::Result;
use std::fs;

pub fn copy_files(ctx: &mut CommandContext) -> Result<()> {
    let dep = ctx.resolve_dependency(&RUST_BOOTLOADER)?;

    fs::copy(
        ctx.workspace_at(&["bootloader", "rust", "boot.json"]),
        ctx.sysroot_directory().join("boot.json"),
    )?;

    fs::copy(
        dep.at(&["bin", "bootloader-x86_64-uefi.efi"]),
        ctx.sysroot_at(&["EFI", "BOOT", "BOOTX64.EFI"])?,
    )?;

    Ok(())
}
