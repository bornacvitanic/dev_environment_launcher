use std::path::PathBuf;
use structopt_derive::StructOpt;
#[derive(StructOpt)]
pub struct Cli {
    #[structopt(parse(from_os_str))]
    pub project_dir: PathBuf
}