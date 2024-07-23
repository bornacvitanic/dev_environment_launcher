use std::path::PathBuf;
use structopt_derive::StructOpt;

#[derive(StructOpt)]
pub enum Command {
    /// Specify a project path to open
    Path { path: PathBuf },

    /// Specify a recent project index to open
    Open { index: usize },

    /// Specify a recent project index to remove
    Remove { index: usize },

    /// Clears all recent projects
    Clear,

    /// List recent projects
    Options,

    /// Interactive menu to select recent project to open
    Recent,
}

#[derive(StructOpt)]
#[structopt(
    name = "devenv",
    about = "A tool to open development project workspaces"
)]
pub struct Cli {
    #[structopt(subcommand)]
    pub command: Option<Command>,
}
