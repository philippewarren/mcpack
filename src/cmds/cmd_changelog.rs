use clap::Args;

#[derive(Debug, Args)]
pub struct ChangelogCommand {
    #[clap(short, long)]
    /// Force the changelog to be printed
    pub force: bool,
}
