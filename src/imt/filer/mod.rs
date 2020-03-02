use crate::imt::image_type::ImageType;
use std::path::{Path, PathBuf};
use std::time::{Duration, SystemTime};

mod base;
mod fileinfo;
mod filerimpl;
mod hm_impl;

pub trait FilerTrait {
    fn add_file<P: Into<PathBuf>>(&mut self, path: P);
    fn file_known<P: AsRef<Path>>(&self, path: P) -> bool;

    fn mod_time<P: AsRef<Path>>(&self, path: P) -> Option<SystemTime>;
    fn set_mod_time<P: Into<PathBuf>>(&mut self, path: P, time: SystemTime);

    fn image_type<P: AsRef<Path>>(&self, path: P) -> Option<ImageType>;
    fn set_image_type<P: Into<PathBuf>>(&mut self, path: P, image_type: ImageType);

    fn hash_for_name<P: AsRef<Path>, S: AsRef<str>>(&self, path: P, hash_name: S)
        -> Option<String>;
    fn contains_hash_for_name<P: AsRef<Path>, S: AsRef<str>>(&self, path: P, hash_name: S) -> bool;
    fn set_hash_for_name<P: Into<PathBuf>, S: AsRef<str>, H: AsRef<str>>(
        &mut self,
        path: P,
        hash_name: S,
        value: H,
    );

    fn with_files<F: FnMut(&PathBuf)>(&self, f: F);
}

//pub use filerimpl::Filer;
