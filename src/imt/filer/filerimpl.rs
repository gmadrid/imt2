use std::path::{Path, PathBuf};
use std::sync::Arc;

use anyhow::Result;
use parking_lot::RwLock;

use crate::imt::filer::fileinfo::Files;
use crate::imt::image_type::ImageType;

#[derive(Clone)]
pub struct Filer {
    files: Arc<RwLock<Files>>,
}

impl Filer {
    pub fn new() -> Result<Filer> {
        let files = Arc::new(RwLock::new(Files::default()));
        Ok(Filer { files })
    }

    pub fn set_image_type<P: Into<PathBuf>>(&mut self, path: P, image_type: ImageType)  {
        self.files.write().set_image_type(path, image_type)
    }

    pub fn image_type<P: Into<PathBuf>>(&self, path: P) -> Option<ImageType> {
        self.files.read().image_type(path)
    }

    pub fn add_file<P: Into<PathBuf>>(&self, path: P)  {
        self.files.write().add_file(path)
    }

    pub fn with_files<F>(&self, mut f: F) where F: FnMut(&PathBuf) {
        self.files.read().file_iter().for_each(|path| {
            f(path);
        })
    }

    pub fn add_hash<P: Into<PathBuf>, S: Into<String>, V: Into<String>>(
        &self,
        path: P,
        hash_name: S,
        hash_value: V,
    ) {
        self.files.write().add_hash(path, hash_name, hash_value)
    }

    pub fn contains_hash<P: Into<PathBuf>, S: Into<String>>(&self, path: P, hash_name: S) -> bool {
        self.files.write().contains_hash(path, hash_name)
    }

    pub fn hash_value<P: Into<PathBuf>, S: Into<String>>(&self, path: P, hash_name: S) -> Option<String> {
        self.files.read().hash_value(path, hash_name).map(|v| v.to_owned() )
    }

    pub fn write_to_path<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        self.files.write().write_to_path(path)
    }
}
