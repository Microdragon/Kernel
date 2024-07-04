// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::arguments::Target;
use crate::dependencies::LIMINE_DEPENDENCY;
use crate::utils::CommandContext;
use color_eyre::Result;
use std::fs;
use std::path::Path;

pub const XORRISO_ARGUMENTS: &[&str] = &[
    "-as",
    "mkisofs",
    "-b",
    "limine/limine-bios-cd.bin",
    "-no-emul-boot",
    "-boot-load-size",
    "4",
    "-boot-info-table",
    "--efi-boot",
    "limine/limine-uefi-cd.bin",
    "-efi-boot-part",
    "--efi-boot-image",
    "--protective-msdos-label",
];

pub fn copy_files(ctx: &mut CommandContext, target: Target) -> Result<()> {
    let dep = ctx.resolve_dependency(&LIMINE_DEPENDENCY)?;

    fs::copy(
        ctx.workspace_at(&["bootloader", "limine", "limine.cfg"]),
        ctx.sysroot_at(&["limine", "limine.cfg"])?,
    )?;

    fs::copy(
        dep.path().join("limine-bios.sys"),
        ctx.sysroot_at(&["limine", "limine-bios.sys"])?,
    )?;

    fs::copy(
        dep.path().join("limine-bios-cd.bin"),
        ctx.sysroot_at(&["limine", "limine-bios-cd.bin"])?,
    )?;

    fs::copy(
        dep.path().join("limine-uefi-cd.bin"),
        ctx.sysroot_at(&["limine", "limine-uefi-cd.bin"])?,
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

pub fn post_process_iso(ctx: &mut CommandContext, iso: &Path) -> Result<()> {
    let dep = ctx.resolve_dependency(&LIMINE_DEPENDENCY)?;
    let limine = dep.path().join(if cfg!(windows) {
        "limine.exe"
    } else {
        "limine"
    });

    ctx.shell().cmd(limine).arg("bios-install").arg(iso).run()?;

    Ok(())
}
