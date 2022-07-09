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

use clap::{ArgAction, Parser, Subcommand};

use cmd_changelog::ChangelogCommand;
use cmd_dev::DevCommand;
use cmd_edit::EditCommand;
use cmd_generate_completions::GenerateCompletionsCommand;
use cmd_init::InitCommand;
use cmd_status::StatusCommand;
use cmd_update::UpdateCommand;

pub mod cmd_update;
pub mod cmd_changelog;
pub mod cmd_status;
pub mod cmd_generate_completions;
pub mod cmd_self_upgrade;
pub mod cmd_init;
pub mod cmd_edit;
pub mod cmd_dev;

#[derive(Debug, Parser)]
#[clap(author, about, version)]
pub struct Cli {
    #[clap(short, long, action = ArgAction::Count)]
    /// Prints verbose output (-v, -vv, -vvv, etc.)
    pub verbose: u8,
    #[clap(short, long, action = ArgAction::SetTrue)]
    /// Don't display anything to stdout or stderr
    pub quiet: bool,
    #[clap(subcommand)]
    /// The action to perform
    pub action: Action,
}

#[derive(Debug, Subcommand)]
pub enum Action {
    #[clap()]
    /// Prints the status of the modpack
    Status(StatusCommand),
    #[clap()]
    /// Updates the modpack
    Update(UpdateCommand),
    #[clap()]
    /// Prints the changelog of the modpack
    Changelog(ChangelogCommand),
    #[clap()]
    /// Generates shell completions for the cli
    GenerateCompletions(GenerateCompletionsCommand),
    #[clap()]
    /// Upgrades to the latest version of mcpack
    SelfUpgrade,
    #[clap()]
    /// Initializes the modpack
    Init(InitCommand),
    #[clap()]
    /// Edit modpack informations
    Edit(EditCommand),
    #[clap()]
    /// Command group for modpack development
    Dev(DevCommand),
}
