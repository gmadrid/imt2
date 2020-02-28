use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

use crate::imt::image_type::ImageType;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Write;

#[derive(Default, Debug, Deserialize, Serialize)]
pub struct Files {
    files: HashMap<PathBuf, FileInfo>,
}

impl Files {
    pub fn file_iter(&self) -> impl Iterator<Item = &PathBuf> {
        self.files.keys()
    }

    pub fn image_type<P: Into<PathBuf>>(&self, path: P) -> Option<ImageType> {
        self.files.get(&path.into()).and_then(|fi| fi.image_type)
    }

    pub fn set_image_type<P: Into<PathBuf>>(&mut self, path: P, image_type: ImageType) {
        self.files
            .entry(path.into())
            .or_insert_with(FileInfo::new)
            .image_type = Some(image_type);
    }

    pub fn add_file<P: Into<PathBuf>>(&mut self, path: P) {
        self.files.entry(path.into()).or_insert_with(FileInfo::new);
    }

    pub fn contains_hash<P: Into<PathBuf>, S: Into<String>>(&self, path: P, hash_name: S) -> bool {
        self.files
            .get(&path.into())
            .map_or(false, |fi| fi.hashes.contains_key(&hash_name.into()))
    }

    pub fn hash_value<P: Into<PathBuf>, S: Into<String>>(
        &self,
        path: P,
        hash_name: S,
    ) -> Option<&String> {
        self.files
            .get(&path.into())
            .and_then(|fi| fi.hashes.get(&hash_name.into()))
    }

    pub fn add_hash<P: Into<PathBuf>, S: Into<String>, V: Into<String>>(
        &mut self,
        path: P,
        hash_name: S,
        hash_value: V,
    ) {
        let entry = self.files.entry(path.into());

        match entry {
            Entry::Vacant(_) => unimplemented!("put an error here"),
            Entry::Occupied(mut occupied) => occupied
                .get_mut()
                .add_hash(hash_name.into(), hash_value.into()),
        }
    }

    pub fn write_to_path<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let s = toml::to_string(&self.files)?;
        let mut file = File::create(path)?;
        file.write_all(s.as_bytes())?;
        file.flush()?;
        Ok(())
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct FileInfo {
    hashes: HashMap<String, String>,
    image_type: Option<ImageType>,
}

impl FileInfo {
    pub fn new() -> FileInfo {
        FileInfo {
            hashes: HashMap::new(),
            image_type: Option::default(),
        }
    }

    pub fn add_hash<S: Into<String>>(&mut self, hash_name: S, hash_value: S) {
        let name = hash_name.into();
        if self.hashes.contains_key(&name) {
            panic!("fileinfo already contains hash for {}", name);
        }
        self.hashes.insert(name, hash_value.into());
    }
}
