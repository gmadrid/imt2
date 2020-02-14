use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Write;

#[derive(Default, Debug, Deserialize, Serialize)]
pub struct Files {
    files: HashMap<PathBuf, FileInfo>,
}

impl Files {
    pub fn add_file<P: Into<PathBuf>>(&mut self, path: P) -> Result<()> {
        self.files.entry(path.into()).or_insert_with(FileInfo::new);
        Ok(())
    }

    pub fn contains_hash<P: Into<PathBuf>, S: Into<String>>(&self, path: P, hash_name: S) -> bool {
        self.files
            .get(&path.into())
            .map_or(false, |fi| fi.hashes.contains_key(&hash_name.into()))
    }

    pub fn add_hash<P: Into<PathBuf>, S: Into<String>, V: Into<String>>(
        &mut self,
        path: P,
        hash_name: S,
        hash_value: V,
    ) -> Result<()> {
        let entry = self.files.entry(path.into());

        match entry {
            Entry::Vacant(_) => unimplemented!("put an error here"),
            Entry::Occupied(mut occupied) => occupied
                .get_mut()
                .add_hash(hash_name.into(), hash_value.into()),
        }
    }

    pub fn write<P: AsRef<Path>>(&self, path: P) -> Result<()> {
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
}

impl FileInfo {
    pub fn new() -> FileInfo {
        FileInfo {
            hashes: HashMap::new(),
        }
    }

    pub fn add_hash<S: Into<String>>(&mut self, hash_name: S, hash_value: S) -> Result<()> {
        let name = hash_name.into();
        let value = hash_value.into();
        if self.hashes.contains_key(&name) {
            panic!("fileinfo already contains hash for {}", name);
        }
        self.hashes.insert(name, value);
        Ok(())
    }
}
