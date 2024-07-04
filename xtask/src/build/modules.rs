// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::BuildArguments;
use crate::arguments::ModuleInfo;
use crate::utils::CommandContext;
use color_eyre::Result;
use xshell::cmd;

pub fn build_modules(build: &BuildArguments, ctx: &CommandContext) -> Result<Vec<String>> {
    let mut dir = build.output_directory(ctx);

    let mut args = Vec::new();
    for info in &build.modules {
        let target = build.target.as_rust_target();
        let name = &info.name;
        let default_features = if info.default_features {
            None
        } else {
            Some("--no-default-features")
        };
        let (feature, features) = if info.features.is_empty() {
            (None, None)
        } else {
            (Some("--features"), Some(info.features.join(",")))
        };

        cmd!(ctx.shell(), "cargo build --target {target} --package {name} {default_features...} {feature...} {features...}").run()?;

        dir.push(format!("lib{}.rlib", name));
        args.push("--extern".to_string());
        args.push(format!("{}={}", name, dir.display()));
        dir.pop();
    }

    Ok(args)
}

pub fn default_modules() -> Vec<ModuleInfo> {
    vec![ModuleInfo::new("acpi"), ModuleInfo::new("logging")]
}
