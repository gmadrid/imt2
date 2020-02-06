use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Deserialize, Serialize)]
pub struct Files {
    files: HashMap<String, FileInfo>,
}

impl Files {
    pub fn add_file(&mut self, fi: FileInfo) {
        // TODO: can you avoid the clone?
        self.files.insert(fi.path.clone(), fi);
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct FileInfo {
    path: String,
    hashes: HashMap<String, String>,
}

impl FileInfo {
    pub fn new(path: String) -> FileInfo {
        let hashes = HashMap::new();
        FileInfo { path, hashes }
    }

    pub fn add_hash(&mut self, hash_name: String, hash_value: String) {
        if self.hashes.contains_key(&hash_name) {
            panic!("fileinfo already contains hash for {}", hash_name);
        }
        self.hashes.insert(hash_name, hash_value);
    }
}

//fn _exercise_serialization() {
//    let mut files = Files::default();
//
//    let mut fi = FileInfo::new("first/bar/baz".into());
//    fi.add_hash("foo".into(), "foovalueofthehash".into());
//    fi.add_hash("bar".into(), "barvalueofthehash".into());
//    files.add_file(fi);
//
//    let mut fi = FileInfo::new("second/quux.jpg".into());
//    fi.add_hash("quux".into(), "quuxvalueofhash".into());
//    fi.add_hash("quip".into(), "quipvalueofhash".into());
//    files.add_file(fi);
//
//    let s = toml::to_string(&files).unwrap();
//    println!("THETOML: {}", s);
//
//    let newfiles = toml::from_str::<Files>(&s).unwrap();
//    println!("RECON: {:?}", newfiles);
//}
//
