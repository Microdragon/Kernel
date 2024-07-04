// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use color_eyre::Result;
use std::path::{Path, PathBuf};
use xshell::Shell;

mod download;
mod git;
mod hooks;
mod manager;
mod predefined;
mod rust;

pub use git::GitDependency;
pub use manager::DependencyManager;
pub use predefined::*;

pub trait Dependency {
    fn id(&self) -> &'static str;
    fn install(&self, sh: &Shell, metadata: &mut serde_json::Value) -> Result<()>;
    fn update(&self, sh: &Shell, metadata: &mut serde_json::Value) -> Result<()>;
}

pub struct ResolvedDependency {
    path: PathBuf,
}

impl ResolvedDependency {
    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn at(&self, path: &[impl AsRef<Path>]) -> PathBuf {
        let mut result = self.path.to_path_buf();
        for segment in path {
            result.push(segment);
        }

        result
    }
}
