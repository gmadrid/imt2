//use std::collections::HashMap;
//
//use serde::{Deserialize, Serialize};
//
//#[derive(Default, Debug, Deserialize, Serialize)]
//pub struct Files {
//    files: HashMap<String, FileInfo>,
//}
//
//impl Files {
////    pub fn add_file(&mut self, fi: FileInfo) {
////        // TODO: can you avoid the clone?
////        self.files.insert(fi.path.clone(), fi);
////    }
//}
//
//#[derive(Deserialize, Serialize, Debug)]
//pub struct FileInfo {
//    path: String,
//    hashes: HashMap<String, String>,
//}
//
//impl FileInfo {
//    pub fn new<S: Into<String>>(path: S) -> FileInfo {
//        let hashes = HashMap::new();
//        FileInfo { path: path.into(), hashes }
//    }
//
//    pub fn add_hash<S: Into<String>>(&mut self, hash_name: S, hash_value: S) {
//        let name = hash_name.into();
//        let value = hash_value.into();
//        if self.hashes.contains_key(&name) {
//            panic!("fileinfo already contains hash for {}", name);
//        }
//        self.hashes.insert(name, value);
//    }
//}
//
////fn _exercise_serialization() {
////    let mut files = Files::default();
////
////    let mut fi = FileInfo::new("first/bar/baz");
////    fi.add_hash("foo", "foovalueofthehash");
////    fi.add_hash("bar", "barvalueofthehash");
////    files.add_file(fi);
////
////    let mut fi = FileInfo::new("second/quux.jpg");
////    fi.add_hash("quux", "quuxvalueofhash");
////    fi.add_hash("quip", "quipvalueofhash");
////    files.add_file(fi);
////
////    let s = toml::to_string(&files).unwrap();
////    println!("THETOML: {}", s);
////
////    let newfiles = toml::from_str::<Files>(&s).unwrap();
////    println!("RECON: {:?}", newfiles);
////}
////
