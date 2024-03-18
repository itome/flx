use std::path::PathBuf;

use clap::Parser;

use crate::utils::version;

#[derive(Parser, Debug)]
#[command(author, version = version(), about)]
pub struct Cli {
    #[arg(
        short,
        long,
        value_name = "String",
        help = "Path to the Flutter project"
    )]
    pub project_root: Option<String>,

    #[arg(long, value_name = "bool", help = "Enable FVM support")]
    pub fvm: bool,
}
