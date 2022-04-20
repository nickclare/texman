use std::{env, path::PathBuf};

use crate::{
    cli,
    workspace::{Workspace, WorkspaceError},
};
use clap::Parser;

#[derive(Parser, Debug)]
pub struct BuildOpts {
    /// Name of the document to build. Required if not run in specific document folder
    document: Option<String>,

    /// Just generate the document and write to stdout
    #[clap(short, long)]
    generate: bool,

    #[clap(flatten)]
    general: cli::GeneralOpts,
}

#[derive(thiserror::Error, Debug)]
pub enum BuildError {
    #[error("i/o error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("error loading workspace: {0}")]
    WorkspaceError(#[from] crate::workspace::WorkspaceError),
}

fn find_workspace(start: &PathBuf) -> Result<Workspace, WorkspaceError> {
    let mut current = start.to_owned();
    while current.exists() {
        if current.join("workspace.toml").exists() {
            return Ok(Workspace::new(&current)?);
        } else {
            if let Some(parent) = current.parent() {
                current = parent.to_path_buf();
            } else {
                return Err(WorkspaceError::NotFound(current));
            }
        }
    }
    Err(WorkspaceError::NotFound(current))
}

pub fn build(opts: &BuildOpts) -> Result<(), BuildError> {
    // Load workspace
    let current_dir = env::current_dir()?;
    let workspace = find_workspace(&current_dir)?;
    if opts.general.verbose {
        eprintln!("Loaded workspace: {:?}", workspace)
    }

    let document = match opts.document {
        Some(ref doc) => workspace.document(doc)?,
        None => todo!("currently document is a required parameter"),
    };

    let document_text = document.generate();
    if opts.generate {
        println!("{}", document_text);
    }

    Ok(())
}
