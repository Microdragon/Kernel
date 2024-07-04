// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

mod modules;
mod runner;

use crate::arguments::{Bootloader, ModuleInfo, Target};
use crate::utils::CommandContext;
use clap::Args;
use color_eyre::eyre::anyhow;
use color_eyre::Result;
use log::info;
use std::fs;
use std::path::PathBuf;
use xshell::{cmd, Shell};

/// Builds the microdragon kernel.
#[derive(Args)]
pub struct BuildArguments {
    /// Specifies the target CPU architecture to build for.
    #[arg(short, long, value_enum, default_value_t)]
    pub target: Target,

    /// Specifies the bootloader to build for.
    #[arg(short, long, value_enum, default_value_t)]
    pub bootloader: Bootloader,

    /// Specifies if a release build should be done.
    #[arg(short, long, default_value_t)]
    pub release: bool,

    /// List of built-in modules to include.
    #[arg(short, long, default_values_t = modules::default_modules())]
    pub modules: Vec<ModuleInfo>,
}

impl BuildArguments {
    pub fn run(&self, ctx: &CommandContext) -> Result<()> {
        if !self.bootloader.supports_target(self.target) {
            return Err(anyhow!(
                "The selected bootloader ({}) does not support the selected target ({})",
                self.bootloader,
                self.target
            ));
        }

        install_target_if_needed(ctx.shell(), self.target)?;

        let args = modules::build_modules(self, ctx)?;

        let runner = runner::generate_runner(self, ctx)?;

        let target = self.target.as_rust_target();
        let bootloader = self.bootloader.as_bootloader_package();
        let release = if self.release {
            Some("--release")
        } else {
            None
        };

        cmd!(
            ctx.shell(),
            "cargo rustc --target {target} --package {bootloader} {release...} -- {args...}"
        )
        .env("MICRODRAGON_RUNNER", runner)
        .run()?;

        Ok(())
    }

    pub fn output_directory(&self, ctx: &CommandContext) -> PathBuf {
        let mut result = ctx.target_directory().to_path_buf();
        result.push(self.target.as_rust_target());
        result.push(if self.release { "release" } else { "debug" });
        result
    }

    pub fn copy_kernel_binary(&self, ctx: &CommandContext) -> Result<()> {
        let mut source: PathBuf = self.output_directory(ctx);
        source.push(self.bootloader.as_bootloader_package());

        if matches!(self.bootloader, Bootloader::Rust) {
            fs::copy(source, ctx.sysroot_directory().join("kernel-x86_64"))?;
        } else {
            fs::copy(source, ctx.sysroot_at(&["system", "kernel"])?)?;
        }

        Ok(())
    }
}

fn install_target_if_needed(sh: &Shell, target: Target) -> Result<()> {
    let installed = cmd!(sh, "rustup target list --installed").read()?;
    let targets: Vec<&str> = installed.lines().collect();
    let target = target.as_rust_target();

    if !targets.contains(&target) {
        info!("Rust target {} not installed. Installing...", target);
        cmd!(sh, "rustup target add {target}").run()?;
    }

    Ok(())
}
