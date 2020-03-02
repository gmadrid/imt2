/*
use serde::{Deserialize, Serialize};
use crate::imt::filer::FilerTrait;
use std::path::PathBuf;
use crate::imt::image_type::ImageType;
use std::time::SystemTime;

#[derive(Default, Debug, Deserialize, Serialize)]
struct HashMapFiler {

}

impl FilerTrait for HashMapFiler {
    fn add_file<P: Into<PathBuf>>(&mut self, path: P) {

        unimplemented!()
    }

    fn mod_time<P: Into<PathBuf>>(&self, path: P) -> Option<SystemTime> {
        unimplemented!()
    }

    fn set_mod_time<P: Into<PathBuf>>(&self, path: P, time: SystemTime) {
        unimplemented!()
    }

    fn image_type<P: Into<PathBuf>>(&self, path: P) -> Option<ImageType> {
        unimplemented!()
    }

    fn set_image_type<P: Into<PathBuf>>(&mut self, path: P, image_type: ImageType) {
        unimplemented!()
    }

    fn hash_for_name<P: Into<PathBuf>, S: AsRef<str>>(&self, path: P, hash_name: S) -> Option<String> {
        unimplemented!()
    }

    fn contains_hash_for_name<P: Into<PathBuf>, S: AsRef<str>>(&self, path: P, hash_name: S) -> bool {
        unimplemented!()
    }

    fn set_hash_for_name<P: Into<PathBuf>, S: AsRef<str>, H: AsRef<str>>(&self, path: P, hash_name: S, value: H) {
        unimplemented!()
    }

    fn with_files<F: FnMut(&PathBuf)>(&self, f: F) {
        unimplemented!()
    }
}*/
