// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::arguments::{Bootloader, Firmware, Target};
use crate::build::BuildArguments;
use crate::dependencies::OVMF_DEPENDENCY;
use crate::utils::CommandContext;
use clap::Args;
use color_eyre::eyre::bail;
use color_eyre::Result;
use log::info;
use xshell::cmd;

mod limine;
mod rust;

/// Builds the microdragon kernel and runs it in a VM.
#[derive(Args)]
pub struct RunArguments {
    #[command(flatten)]
    build: BuildArguments,

    /// Firmware to run in QEMU.
    #[arg(short, long, default_value_t)]
    firmware: Firmware,

    /// Does not launch the debugger.
    #[arg(long)]
    no_debug: bool,

    /// Additional QEMU arguments.
    args: Vec<String>,
}

impl RunArguments {
    pub fn run(self, mut ctx: CommandContext) -> Result<()> {
        if self.build.bootloader == Bootloader::Rust && self.firmware == Firmware::Bios {
            bail!("Cannot PXE boot the rust bootloader using bios firmware.");
        }

        self.build.run(&ctx)?;

        info!("Collecting files...");
        self.copy_bootloader_files(&mut ctx)?;
        self.build.copy_kernel_binary(&ctx)?;
        let ovmf = ctx.resolve_dependency(&OVMF_DEPENDENCY)?;

        info!("Starting QEMU...");
        let (qemu, mut default_args) = match self.build.target {
            Target::X86_64 => ("qemu-system-x86_64", vec!["-cpu", "qemu64"]),
            Target::AArch64 => ("qemu-system-aarch64", vec!["-M", "virt"]),
            Target::RiscV64 => todo!(),
        };
        let extra = &self.args;
        let sysroot = ctx.sysroot_directory();

        if !self.no_debug {
            default_args.push("-gdb");
            default_args.push("tcp:localhost:1234");
            default_args.push("-S");

            open::that_detached("vscode://vadimcn.vscode-lldb/launch?name=Remote attach")?;
        }

        match self.firmware {
            Firmware::Bios => {
                cmd!(
                    ctx.shell(),
                    "{qemu} {default_args...} -netdev user,id=net0,tftp={sysroot},bootfile=/limine-bios-pxe.bin -device virtio-net-pci,netdev=net0 {extra...}"
                )
                .run()?;
            }
            Firmware::Uefi => {
                let code = match self.build.target {
                    Target::X86_64 => ovmf.at(&["x64", "code.fd"]),
                    Target::AArch64 => ovmf.at(&["aarch64", "code.fd"]),
                    Target::RiscV64 => todo!(),
                };
                cmd!(
                    ctx.shell(),
                    "{qemu} {default_args...} -drive if=pflash,format=raw,unit=0,file={code},readonly=on -netdev user,id=net0,tftp={sysroot},bootfile=/EFI/BOOT/BOOTX64.EFI -device virtio-net-pci,netdev=net0 {extra...}"
                )
                .run()?;
            }
        }

        Ok(())
    }

    fn copy_bootloader_files(&self, ctx: &mut CommandContext) -> Result<()> {
        match self.build.bootloader {
            Bootloader::Limine => limine::copy_files(ctx, self.build.target),
            Bootloader::Rust => rust::copy_files(ctx),
        }
    }
}
