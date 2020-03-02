/*
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::{Duration, SystemTime};

use anyhow::Result;
use parking_lot::RwLock;

use crate::imt::filer::fileinfo::Files;
use crate::imt::image_type::ImageType;

struct PeriodicFiler {}

impl PeriodicFiler {
    fn new<P: AsRef<Path>>(p: P, period: Duration) -> Self {
        unimplemented!()
    }

    fn save() -> Result<()> {
        unimplemented!()
    }

    fn stop() -> Result<()> {
        unimplemented!()
    }

    fn stop_and_save() -> Result<()> {
        unimplemented!()
    }
}

// NEEDS
// - background saving
// - mutation aware to avoid saving unchanged data
// - iterate over all files
// - save multiple named hashes for each file
// - record mod time of file
//   - erase saved hashes and other data when mod time changes.
// - save image type
#[derive(Clone)]
pub struct Filer {
    files: Arc<RwLock<Files>>,
}

impl Default for Filer {
    fn default() -> Self {
        Filer {
            files: Arc::new(RwLock::new(Files::default())),
        }
    }
}

impl Filer {
    pub fn set_image_type<P: Into<PathBuf>>(&mut self, path: P, image_type: ImageType) {
        self.files.write().set_image_type(path, image_type)
    }

    pub fn image_type<P: Into<PathBuf>>(&self, path: P) -> Option<ImageType> {
        self.files.read().image_type(path)
    }

    pub fn add_file<P: Into<PathBuf>>(&self, path: P) {
        self.files.write().add_file(path)
    }

    pub fn with_files<F>(&self, mut f: F)
    where
        F: FnMut(&PathBuf),
    {
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

    pub fn hash_value<P: Into<PathBuf>, S: Into<String>>(
        &self,
        path: P,
        hash_name: S,
    ) -> Option<String> {
        self.files
            .read()
            .hash_value(path, hash_name)
            .map(|v| v.to_owned())
    }

    pub fn write_to_path<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        self.files.write().write_to_path(path)
    }
}
*/
