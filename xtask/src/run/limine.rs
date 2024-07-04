// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::arguments::Target;
use crate::dependencies::LIMINE_DEPENDENCY;
use crate::utils::CommandContext;
use color_eyre::Result;
use std::fs;

pub fn copy_files(ctx: &mut CommandContext, target: Target) -> Result<()> {
    let dep = ctx.resolve_dependency(&LIMINE_DEPENDENCY)?;

    fs::write(
        ctx.sysroot_at(&["limine", "limine.cfg"])?,
        "TIMEOUT=0
        :Microdragon Debug
        PROTOCOL=limine
        KASLR=no
        KERNEL_PATH=boot:///system/kernel",
    )?;

    fs::copy(
        dep.path().join("limine-bios.sys"),
        ctx.sysroot_at(&["limine", "limine-bios.sys"])?,
    )?;

    fs::copy(
        dep.path().join("limine-bios-pxe.bin"),
        ctx.sysroot_directory().join("limine-bios-pxe.bin"),
    )?;

    match target {
        Target::X86_64 => fs::copy(
            dep.path().join("BOOTX64.EFI"),
            ctx.sysroot_at(&["EFI", "BOOT", "BOOTX64.EFI"])?,
        )?,
        Target::AArch64 => fs::copy(
            dep.path().join("BOOTAA64.EFI"),
            ctx.sysroot_at(&["EFI", "BOOT", "BOOTAA64.EFI"])?,
        )?,
        Target::RiscV64 => fs::copy(
            dep.path().join("BOOTRISCV64.EFI"),
            ctx.sysroot_at(&["EFI", "BOOT", "BOOTRISCV64.EFI"])?,
        )?,
    };

    Ok(())
}
