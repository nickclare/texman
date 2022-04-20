//! Main module for the CLI interface

use clap::{Parser, Subcommand};
use color_eyre::eyre;

use crate::{
    build::{self, BuildOpts},
    init,
};

/// Manage a LaTeX workspace.
#[derive(Parser, Debug)]
pub struct Opts {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    /// Initialize a new workspace.
    Init(InitOpts),
    /// Build a document
    Build(BuildOpts),
}

#[derive(Parser, Debug)]
pub struct GeneralOpts {
    #[clap(short, long)]
    pub verbose: bool,
}

impl GeneralOpts {}

#[derive(Parser, Debug)]
pub struct InitOpts {
    /// Name for the new workspace. Required unless you are in an empty directory.
    name: Option<String>,

    #[clap(flatten)]
    general: GeneralOpts,
}

pub fn main() -> eyre::Result<()> {
    let opts = Opts::parse();
    match opts.command {
        Command::Init(ref opts) => Ok(init::init_workspace(opts.name.clone(), &opts.general)?),
        Command::Build(ref opts) => Ok(build::build(opts)?),
    }
}
