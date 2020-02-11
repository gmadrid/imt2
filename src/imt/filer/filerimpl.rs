use std::path::{Path, PathBuf};
use std::sync::Arc;

use anyhow::Result;
use parking_lot::RwLock;

use crate::imt::filer::fileinfo::Files;

#[derive(Clone)]
pub struct Filer {
    files: Arc<RwLock<Files>>,
}

impl Filer {
    pub fn new() -> Result<Filer> {
        let files = Arc::new(RwLock::new(Files::default()));
        Ok(Filer { files })
    }

    pub fn add_file<P: Into<PathBuf>>(&self, path: P) -> Result<()> {
        self.files.write().add_file(path)
    }

    pub fn add_hash<P: Into<PathBuf>, S: Into<String>, V: Into<String>>(
        &self,
        path: P,
        hash_name: S,
        hash_value: V,
    ) -> Result<()> {
        self.files.write().add_hash(path, hash_name, hash_value)
    }

    pub fn contains_hash<P: Into<PathBuf>, S: Into<String>>(&self, path: P, hash_name: S) -> bool {
        self.files.write().contains_hash(path, hash_name)
    }

    pub fn write_output<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        self.files.write().write(path)
    }
}
