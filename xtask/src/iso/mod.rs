// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::arguments::Bootloader;
use crate::build::BuildArguments;
use crate::utils::CommandContext;
use clap::Args;
use color_eyre::eyre::anyhow;
use color_eyre::Result;
use log::info;

mod limine;

/// Builds the microdragon kernel and packs it into an iso
///
/// The build iso file is a 'hybrid' iso that can be booted from both UEFI and BIOS systems.
#[derive(Args)]
pub struct IsoArguments {
    #[command(flatten)]
    build: BuildArguments,
}

impl IsoArguments {
    pub fn run(self, mut ctx: CommandContext) -> Result<()> {
        self.build.run(&ctx)?;

        info!("Collecting files...");
        match self.build.bootloader {
            Bootloader::Limine => limine::copy_files(&mut ctx, self.build.target)?,
            Bootloader::Rust => {
                return Err(anyhow!(
                    "Rust Bootloader does not support booting from disk."
                ))
            }
        }

        self.build.copy_kernel_binary(&ctx)?;

        info!("Creating iso...");
        let iso = ctx.target_directory().join("microdragon.iso");
        ctx.shell()
            .cmd("xorriso")
            .args(match self.build.bootloader {
                Bootloader::Limine => limine::XORRISO_ARGUMENTS,
                Bootloader::Rust => unreachable!(),
            })
            .arg(ctx.sysroot_directory())
            .arg("-o")
            .arg(&iso)
            .run()?;

        match self.build.bootloader {
            Bootloader::Limine => limine::post_process_iso(&mut ctx, &iso)?,
            Bootloader::Rust => unreachable!(),
        }

        Ok(())
    }
}
