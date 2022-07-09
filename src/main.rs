/*
 * Copyright (c) 2022 Philippe Warren (philippewarren)
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 *
 */

use clap::Parser;
use log::{debug, error, info, LevelFilter, trace, warn};
use simplelog::{ColorChoice, ConfigBuilder, TerminalMode, TermLogger};

use cmds::{Action, Cli};
use cmds::cmd_changelog;
use cmds::cmd_dev;
use cmds::cmd_edit;
use cmds::cmd_generate_completions;
use cmds::cmd_init;
use cmds::cmd_self_upgrade;
use cmds::cmd_status;
use cmds::cmd_update;

mod modfiles;
mod cmds;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    if cfg!(windows) {
        cmds::cmd_self_upgrade::cleanup_backup_file()?;
    }

    let args: Cli = Cli::parse();
    let log_level = match args.verbose {
        0 => LevelFilter::Warn,
        1 => LevelFilter::Info,
        2 => LevelFilter::Debug,
        _ => LevelFilter::Trace,
    };

    let log_level = if args.quiet { LevelFilter::Off } else { log_level };

    let cfg = ConfigBuilder::new().set_time_level(LevelFilter::Debug).build();
    TermLogger::init(log_level, cfg, TerminalMode::Stderr, ColorChoice::Auto).unwrap();

    match args.action {
        Action::Status(cmd) => {
            cmd_status::print_status(cmd)?
        }
        Action::Update(cmd) => {
            error!("Updating error");
            warn!("Updating warning");
            info!("Updating info");
            debug!("Updating debug");
            trace!("Updating trace");
            cmd_update::update(cmd)?
        }
        Action::Changelog(cmd) => {
            cmd_changelog::show_changelog(cmd)?
        }
        Action::GenerateCompletions(cmd) => {
            cmd_generate_completions::generate_completions(cmd.shell, &mut std::io::stdout())?
        }
        Action::Init(cmd) => {
            cmd_init::init(cmd)?
        }
        Action::Edit(cmd) => {
            cmd_edit::edit(cmd)?
        }
        Action::Dev(cmd) => {
            cmd_dev::dev(cmd)?
        }
        Action::SelfUpgrade => {
            cmd_self_upgrade::upgrade(args.quiet)?
        }
    }

    Ok(())
}
