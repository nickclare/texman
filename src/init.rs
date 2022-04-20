//! Init command. Initializes an empty workspace.
use crate::cli;

#[derive(Debug, thiserror::Error)]
pub enum InitError {
    #[error("The current directory is not empty")]
    NotEmpty,
    #[error("The target directory already exists and is not empty")]
    AlreadyExists,
    #[error("i/o error: {0}")]
    IoError(#[from] std::io::Error),
}

pub fn init_workspace(name: Option<String>, opts: &cli::GeneralOpts) -> Result<(), InitError> {
    let mut target_dir = std::env::current_dir()?;
    if let Some(name) = name {
        target_dir.push(name);
    }
    if opts.verbose {
        println!("Initializing workspace at {:?}", target_dir);
    }

    todo!()
}
