// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::build::BuildArguments;
use crate::iso::IsoArguments;
use crate::run::RunArguments;
use clap::{Parser, ValueEnum};
use std::fmt::{self, Display, Formatter, Write};
use std::str::FromStr;

/// Task runner for the microdragon kernel.
#[derive(Parser)]
pub enum ProgramArguments {
    Build(BuildArguments),
    Run(RunArguments),
    Iso(IsoArguments),

    /// Updates the license header in rust files.
    License,
}

#[derive(ValueEnum, Default, Clone, Copy, PartialEq, Eq)]
pub enum Target {
    /// Specifies the 64-bit Intel / AMD Architecture.
    #[default]
    #[value(name = "x86_64")]
    X86_64,

    /// Specifies the 64-bit Arm Architecture.
    #[value(name = "aarch64")]
    AArch64,

    /// Specifies the 64-bit Risc-V Architecture.
    #[value(name = "riscv64")]
    RiscV64,
}

impl Target {
    pub fn as_rust_target(self) -> &'static str {
        match self {
            Target::X86_64 => "x86_64-unknown-none",
            Target::AArch64 => "aarch64-unknown-none-softfloat",
            Target::RiscV64 => "riscv64imac-unknown-none-elf",
        }
    }
}

impl Display for Target {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Target::X86_64 => f.write_str("x86_64"),
            Target::AArch64 => f.write_str("aarch64"),
            Target::RiscV64 => f.write_str("riscv64"),
        }
    }
}

#[derive(ValueEnum, Default, Clone, Copy, PartialEq, Eq)]
pub enum Bootloader {
    /// Specifies the Limine Bootloader.
    #[default]
    Limine,

    /// Specifies the Rust Bootloader.
    Rust,
}

impl Bootloader {
    pub fn as_bootloader_package(self) -> &'static str {
        match self {
            Bootloader::Limine => "microdragon-limine",
            Bootloader::Rust => "microdragon-rust",
        }
    }

    pub fn supports_target(self, target: Target) -> bool {
        match self {
            Bootloader::Limine => true,
            Bootloader::Rust => matches!(target, Target::X86_64),
        }
    }
}

impl Display for Bootloader {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Bootloader::Limine => f.write_str("Limine"),
            Bootloader::Rust => f.write_str("Rust Bootloader"),
        }
    }
}

#[derive(ValueEnum, Default, Clone, Copy, PartialEq, Eq)]
pub enum Firmware {
    /// Specifies the Bios firmware.
    #[default]
    Bios,

    /// Specifies the Uefi Firmware.
    Uefi,
}

impl Display for Firmware {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Firmware::Bios => f.write_str("bios"),
            Firmware::Uefi => f.write_str("uefi"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ModuleInfo {
    pub name: String,
    pub features: Vec<String>,
    pub default_features: bool,
}

impl ModuleInfo {
    pub fn new(name: &str) -> Self {
        ModuleInfo {
            name: name.to_string(),
            features: Vec::new(),
            default_features: true,
        }
    }
}

impl Display for ModuleInfo {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(&self.name)?;
        if self.features.is_empty() && self.default_features {
            return Ok(());
        }

        f.write_str(if self.default_features { "=" } else { "==" })?;

        let mut first = true;
        for feature in &self.features {
            if first {
                first = false;
            } else {
                f.write_char(',')?;
            }

            f.write_str(feature)?;
        }

        Ok(())
    }
}

impl FromStr for ModuleInfo {
    type Err = color_eyre::Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(idx) = s.find('=') {
            let name = s[..idx].to_string();
            let default_features = idx + 1 > s.len() || s.as_bytes()[idx + 1] != b'=';

            let idx = if default_features { idx } else { idx + 1 };

            let mut features = Vec::new();
            for feature in s[idx..].split(',') {
                if feature.is_empty() {
                    continue;
                }

                features.push(feature.to_string());
            }

            Ok(ModuleInfo {
                name,
                features,
                default_features,
            })
        } else {
            Ok(ModuleInfo {
                name: s.to_string(),
                features: Vec::new(),
                default_features: true,
            })
        }
    }
}
