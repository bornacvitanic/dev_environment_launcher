use std::path::PathBuf;
use structopt_derive::StructOpt;

#[derive(StructOpt)]
pub enum Command {
    /// Specify a project path
    Path { path: PathBuf },

    /// Specify a recent project index
    Recent { index: usize },

    /// List recent projects
    List,

    /// Interactive menu to select recent project
    Menu
}

#[derive(StructOpt)]
pub struct Cli {
    #[structopt(subcommand)]
    pub command: Option<Command>,
}