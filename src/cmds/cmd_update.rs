use clap::Args;

// use clap::{Parser};
//
// struct subcommand {
//     #[clap(subcommand)]
//     action: Action,
// }
//
// enum Action {
//     #[clap(name = "status")]
//     Status,
//     #[clap(name = "update")]
//     GetName,
// }

#[derive(Debug, Args)]
pub struct UpdateCommand {
    #[clap(short, long)]
    /// Forces the update
    pub force: bool,
}


pub fn update(cmd: UpdateCommand) {
    println!("Printing status {:?}", cmd);
}
