use clap::Parser;
use log::{debug, error, info, LevelFilter, trace, warn};
use simplelog::{ColorChoice, ConfigBuilder, TerminalMode, TermLogger};

use cmds::{Action, Cli};
use cmds::cmd_changelog;
use cmds::cmd_status;
use cmds::cmd_update;

mod modfiles;
mod cmds;

fn main() {
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
        Action::Status => cmd_status::print_status(),
        Action::Update(cmd) => {
            cmd_update::update(cmd);
            error!("Updating error");
            warn!("Updating warning");
            info!("Updating info");
            debug!("Updating debug");
            trace!("Updating trace");
        }
        Action::Changelog(cmd) => {
            debug!("Printing changelog");
            // modfiles::curseforge::changelog(force);
        }
    }

    // match args {
    //     Cli { action: Action::Status, .. } => { cmd_status::print_status() }
    //     Cli { action: Action::Update(UpdateCommand { .. }), .. } => {
    //         // println!("Updating");
    //         error!("Updating error");
    //         warn!("Updating warning");
    //         info!("Updating info");
    //         debug!("Updating debug");
    //         trace!("Updating trace");
    //     }
    //     Cli { action: Action::Changelog(ChangelogCommand { .. }), .. } => {
    //         // println!("Printing changelog");
    //         // warn!("Printing changelog");
    //     }
    // }
}
