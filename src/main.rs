use structopt::StructOpt;
use crate::cli::Cli;

mod cli;

fn main() {
    let args = Cli::from_args();
    println!("{:?}", args.project_dir)
}