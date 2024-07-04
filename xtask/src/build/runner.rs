// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::BuildArguments;
use crate::utils::CommandContext;
use cargo_metadata::MetadataCommand;
use color_eyre::Result;
use log::warn;
use std::collections::HashSet;
use std::fmt::Write;
use std::fs;
use std::path::PathBuf;

const PREAMBLE: &str = "// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

fn run_modules(interface: &ModuleInterface) {
";

const APPENDIX: &str = "}
";

pub fn generate_runner(build: &BuildArguments, ctx: &CommandContext) -> Result<PathBuf> {
    let metadata = MetadataCommand::new()
        .current_dir(ctx.workspace_directory())
        .no_deps()
        .exec()?;

    let modules = build
        .modules
        .iter()
        .map(|x| x.name.to_string())
        .collect::<HashSet<String>>();

    let mut infos = Vec::new();
    for package in metadata
        .packages
        .iter()
        .filter(|x| modules.contains(&x.name))
    {
        let Some(constructors) = package
            .metadata
            .as_object()
            .and_then(|x| x.get("microdragon"))
            .and_then(|x| x.as_object())
            .and_then(|x| x.get("constructors"))
            .and_then(|x| x.as_array())
        else {
            continue;
        };

        for constructor in constructors.iter().filter_map(|x| x.as_object()) {
            let Some(path) = constructor.get("path").and_then(|x| x.as_str()) else {
                warn!(
                    "Constructor in package {} is missing it's path",
                    package.name
                );
                continue;
            };

            let Some(order) = constructor.get("order").and_then(|x| x.as_u64()) else {
                warn!(
                    "Constructor {} in package {} is missing it's order",
                    path, package.name
                );
                continue;
            };

            let cfg = constructor
                .get("cfg")
                .and_then(|x| x.as_str())
                .map(|x| x.to_string());

            infos.push(ConstructorInfo {
                function: format!("::{}::{}", package.name, path),
                order,
                cfg,
            });
        }
    }

    let mut runner = String::from(PREAMBLE);

    infos.sort_by_key(|x| x.order);
    for info in &infos {
        if let Some(cfg) = &info.cfg {
            writeln!(&mut runner, "#[cfg({})]", cfg)?;
        }
        writeln!(&mut runner, "    {}(interface);", info.function)?;
    }

    runner.push_str(APPENDIX);

    let path = ctx.target_directory().join("runner.rs");
    fs::write(&path, runner)?;

    Ok(path)
}

struct ConstructorInfo {
    function: String,
    order: u64,
    cfg: Option<String>,
}
