// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use arguments::ProgramArguments;
use clap::Parser;
use color_eyre::Result;
use utils::CommandContext;

mod arguments;
mod build;
mod dependencies;
mod iso;
mod license;
mod run;
mod utils;

fn main() -> Result<()> {
    color_eyre::install()?;
    env_logger::init();
    log::set_max_level(log::LevelFilter::Info);

    let args = ProgramArguments::parse();
    let ctx = CommandContext::new()?;

    match args {
        ProgramArguments::Build(build) => build.run(&ctx),
        ProgramArguments::Run(run) => run.run(ctx),
        ProgramArguments::Iso(iso) => iso.run(ctx),
        ProgramArguments::License => license::run(ctx),
    }
}
