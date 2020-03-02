use super::FilerTrait;
use crate::imt::image_type::ImageType;
use parking_lot::RwLock;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::SystemTime;

/// Filer implementation that may be shared between threads.
pub struct SharedFiler<F: FilerTrait> {
    base: Arc<RwLock<F>>,
}

impl<F: FilerTrait> FilerTrait for SharedFiler<F> {
    fn add_file<P: Into<PathBuf>>(&mut self, path: P) {
        unimplemented!()
    }

    fn file_known<P: AsRef<Path>>(&self, path: P) -> bool {
        unimplemented!()
    }

    fn mod_time<P: AsRef<Path>>(&self, path: P) -> Option<SystemTime> {
        unimplemented!()
    }

    fn set_mod_time<P: Into<PathBuf>>(&mut self, path: P, time: SystemTime) {
        unimplemented!()
    }

    fn image_type<P: AsRef<Path>>(&self, path: P) -> Option<ImageType> {
        unimplemented!()
    }

    fn set_image_type<P: Into<PathBuf>>(&mut self, path: P, image_type: ImageType) {
        unimplemented!()
    }

    fn hash_for_name<P: AsRef<Path>, S: AsRef<str>>(
        &self,
        path: P,
        hash_name: S,
    ) -> Option<String> {
        unimplemented!()
    }

    fn contains_hash_for_name<P: AsRef<Path>, S: AsRef<str>>(&self, path: P, hash_name: S) -> bool {
        unimplemented!()
    }

    fn set_hash_for_name<P: Into<PathBuf>, S: AsRef<str>, H: AsRef<str>>(
        &mut self,
        path: P,
        hash_name: S,
        value: H,
    ) {
        unimplemented!()
    }
}

#[cfg(test)]
mod test {}
