use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Deserialize, Serialize)]
pub struct Files {
    files: Vec<FileInfo>,
}

impl Files {
    pub fn add_file(&mut self, fi: FileInfo) {
        self.files.push(fi);
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct FileInfo {
    name: String,
    hashes: HashMap<String, String>,
}

impl FileInfo {
    pub fn new(name: String) -> FileInfo {
        let hashes = HashMap::new();
        FileInfo { name, hashes }
    }

    pub fn add_hash(&mut self, hash_name: String, hash_value: String) {
        if self.hashes.contains_key(&hash_name) {
            panic!("fileinfo already contains hash for {}", hash_name);
        }
        self.hashes.insert(hash_name, hash_value);
    }
}
