use clap::{ArgAction, Parser, Subcommand};

use cmd_changelog::ChangelogCommand;
use cmd_update::UpdateCommand;

pub mod cmd_update;
pub mod cmd_changelog;
pub mod cmd_status;

#[derive(Debug, Parser)]
#[clap(author, about, version)]
pub struct Cli {
    #[clap(short, long, action = ArgAction::Count)]
    /// Prints verbose output (-v, -vv, -vvv, etc.)
    pub verbose: u8,
    #[clap(short, long, action = ArgAction::SetTrue)]
    pub quiet: bool,
    #[clap(subcommand)]
    /// The action to perform
    pub action: Action,
}

#[derive(Debug, Subcommand)]
pub enum Action {
    #[clap(name = "status")]
    /// Prints the status of the modpack
    Status,
    #[clap(name = "update")]
    /// Updates the modpack
    Update(UpdateCommand),
    #[clap(name = "changelog")]
    /// Prints the changelog of the modpack
    Changelog(ChangelogCommand),
}
