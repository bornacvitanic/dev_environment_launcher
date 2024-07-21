use std::path::PathBuf;
use structopt_derive::StructOpt;

#[derive(StructOpt)]
pub enum Command {
    /// Specify a project path to open
    Path { path: PathBuf },

    /// Specify a recent project index to open
    Recent { index: usize },

    /// Specify a recent project index to remove
    Remove { index: usize },

    /// List recent projects
    List,

    /// Interactive menu to select recent project to open
    Menu
}

#[derive(StructOpt)]
#[structopt(name = "devenv", about = "A tool to open development project workspaces")]
pub struct Cli {
    #[structopt(subcommand)]
    pub command: Option<Command>,
}