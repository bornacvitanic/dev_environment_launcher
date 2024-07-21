use std::path::PathBuf;
use structopt_derive::StructOpt;
use crate::ProjectType;

#[derive(StructOpt)]
pub struct Cli {
    #[structopt(parse(try_from_str))]
    pub project_type: ProjectType,
    #[structopt(parse(from_os_str))]
    pub project_dir: Option<PathBuf>
}