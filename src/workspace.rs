//! Model for a workspace

use std::{
    fs::File,
    io::{BufReader, Read},
    path::PathBuf,
};

use once_cell::sync::OnceCell;
use serde::{Deserialize, Serialize};
use tera::Tera;

type Result<T> = std::result::Result<T, WorkspaceError>;

struct Templates(Tera);

impl Templates {
    fn get() -> &'static Tera {
        static INSTANCE: OnceCell<Templates> = OnceCell::new();
        &INSTANCE.get_or_init(|| Templates::load()).0
    }

    fn load() -> Self {
        let mut tera = Tera::default();
        tera.add_raw_templates(vec![("document", include_str!("templates/doc.tera.tex"))])
            .unwrap();
        Templates(tera)
    }
}

#[derive(thiserror::Error, Debug)]
pub enum WorkspaceError {
    #[error("path not found: {0}")]
    NotFound(PathBuf),
    #[error("not a valid workspace: {msg}")]
    NotValid { msg: String },
    #[error("i/o error: {0}")]
    IoError(#[from] std::io::Error),
}

#[derive(Debug, Clone)]
pub struct Workspace {
    path: PathBuf,
    metadata: Metadata,
}

/// Metadata for a workspace
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Metadata {}

impl Metadata {
    pub fn load(root: &PathBuf) -> Result<Self> {
        let path = root.join("workspace.toml");
        let mut file = BufReader::new(
            File::open(path).map_err(|_| WorkspaceError::NotFound(root.to_path_buf()))?,
        );
        let mut content = Vec::new();
        let _count = file.read_to_end(&mut content)?;
        Ok(
            toml::from_slice(&content).map_err(|e| WorkspaceError::NotValid {
                msg: format!("{}", e),
            })?,
        )
    }
}

// TODO: better name?
/// Metadata for a given document
#[derive(Debug, Deserialize, Serialize)]
pub struct DocumentMeta {
    #[serde(alias = "document-class", default = "DocumentMeta::default_class")]
    pub document_class: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub document_options: Vec<String>,
}

impl DocumentMeta {
    fn default_class() -> String {
        String::from("article")
    }

    pub fn load(root: &PathBuf) -> Result<Self> {
        let path = root.join("metadata.toml");
        let mut file = BufReader::new(File::open(path)?);
        let mut content = Vec::new();
        let _count = file.read_to_end(&mut content)?;
        toml::from_slice(&content).map_err(|e| WorkspaceError::NotValid { msg: e.to_string() })
    }
}
pub struct Document<'w> {
    workspace: &'w Workspace,
    key: String,
    metadata: DocumentMeta,
}

impl<'w> Document<'w> {
    /// Construct the template engine context.
    fn build_context(&self) -> tera::Context {
        let mut ctx = tera::Context::new();
        ctx.insert("metadata", &self.metadata);
        ctx.insert(
            "workspace_root",
            &self
                .workspace
                .path
                .canonicalize()
                .unwrap_or_else(|_| panic!("for now")),
        );
        ctx.insert("prelude_path", "prelude/main.tex");
        let main = PathBuf::new().join("docs").join(&self.key).join("main.tex");
        ctx.insert("main", &main);
        ctx
    }

    /// Generate the content of the main document file
    pub fn generate(&self) -> String {
        let ctx = self.build_context();
        Templates::get()
            .render("document", &ctx)
            .unwrap_or_else(|_e| panic!("couldn't render template: {_e}"))
    }
}

impl Workspace {
    pub fn new(path: impl Into<PathBuf>) -> Result<Self> {
        let path = path.into();
        let metadata = Metadata::load(&path)?;

        Ok(Self { path, metadata })
    }

    /// Fetch the document with the given key
    pub fn document(&self, key: impl Into<String>) -> Result<Document> {
        let key = key.into();
        let doc_path = self.path.join("docs").join(&key);
        let metadata = DocumentMeta::load(&doc_path)?;
        Ok(Document {
            workspace: self,
            key,
            metadata,
        })
    }
}
