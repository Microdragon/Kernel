// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::dependencies::{Dependency, DependencyManager, ResolvedDependency};
use color_eyre::Result;
use std::fs;
use std::path::{Path, PathBuf};
use xshell::{cmd, Shell};

const DEPS_DIRECTORY_NAME: &str = "deps";

pub struct CommandContext {
    shell: Shell,
    deps: DependencyManager,
    workspace: PathBuf,
    target: PathBuf,
    sysroot: PathBuf,
}

impl CommandContext {
    pub fn new() -> Result<Self> {
        let shell = Shell::new()?;
        let workspace = get_workspace_dir(&shell)?;
        let deps = DependencyManager::load(workspace.join(DEPS_DIRECTORY_NAME))?;
        let target = workspace.join("target");
        let sysroot = target.join("sysroot");

        if !sysroot.exists() {
            fs::create_dir(&sysroot)?;
        }

        Ok(CommandContext {
            shell,
            deps,
            workspace,
            target,
            sysroot,
        })
    }

    pub fn shell(&self) -> &Shell {
        &self.shell
    }

    pub fn workspace_directory(&self) -> &Path {
        &self.workspace
    }

    pub fn workspace_at(&self, path: &[impl AsRef<Path>]) -> PathBuf {
        let mut result = self.workspace.clone();
        for segment in path {
            result.push(segment);
        }

        result
    }

    pub fn target_directory(&self) -> &Path {
        &self.target
    }

    pub fn sysroot_directory(&self) -> &Path {
        &self.sysroot
    }

    pub fn sysroot_at(&self, path: &[impl AsRef<Path>]) -> Result<PathBuf> {
        let mut result = self.sysroot.clone();
        for segment in path {
            if !result.exists() {
                fs::create_dir(&result)?;
            }

            result.push(segment);
        }

        Ok(result)
    }

    pub fn resolve_dependency(&mut self, dep: &impl Dependency) -> Result<ResolvedDependency> {
        self.deps.resolve(dep, &self.shell)
    }
}

fn get_workspace_dir(sh: &Shell) -> Result<PathBuf> {
    let path = cmd!(
        sh,
        "cargo locate-project --workspace --message-format=plain"
    )
    .quiet()
    .read()?;
    let toml = PathBuf::from(path);

    Ok(toml.parent().unwrap().into())
}
