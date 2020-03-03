use super::base::Base;
use super::FilerTrait;
use crate::imt::image_type::ImageType;
use parking_lot::RwLock;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::SystemTime;

/// Filer implementation that may be shared between threads.
pub struct SharedFiler<F: FilerTrait> {
    data: Arc<RwLock<Data<F>>>,
}

struct Data<F: FilerTrait> {
    base: F,
    modified: bool,
}

pub type BasicSharedFiler = SharedFiler<Base>;

impl SharedFiler<Base> {
    pub fn new_basic() -> Self {
        SharedFiler::new(Base::default())
    }
}

impl<F: FilerTrait> SharedFiler<F> {
    /// Creates a new SharedFiler and starts tracking modifications.
    fn new(base: F) -> SharedFiler<F> {
        SharedFiler {
            data: Arc::new(RwLock::new(Data {
                base: base,
                modified: false,
            })),
        }
    }

    fn mod_flag(&self) -> bool {
        self.data.read().modified
    }

    fn set_mod_flag(&mut self, val: bool) {
        self.data.write().modified = val;
    }
}

impl<F: FilerTrait> FilerTrait for SharedFiler<F> {
    fn add_file<P: Into<PathBuf>>(&mut self, path: P) {
        let mut data = self.data.write();
        data.base.add_file(path);
        data.modified = true;
    }

    fn file_known<P: AsRef<Path>>(&self, path: P) -> bool {
        self.data.read().base.file_known(path)
    }

    fn mod_time<P: AsRef<Path>>(&self, path: P) -> Option<SystemTime> {
        self.data.read().base.mod_time(path)
    }

    fn set_mod_time<P: Into<PathBuf>>(&mut self, path: P, time: SystemTime) {
        let mut data = self.data.write();
        data.base.set_mod_time(path, time);
        data.modified = true;
    }

    fn image_type<P: AsRef<Path>>(&self, path: P) -> Option<ImageType> {
        self.data.read().base.image_type(path)
    }

    fn set_image_type<P: Into<PathBuf>>(&mut self, path: P, image_type: ImageType) {
        let mut data = self.data.write();
        data.base.set_image_type(path, image_type);
        data.modified = true;
    }

    fn hash_for_name<P: AsRef<Path>, S: AsRef<str>>(
        &self,
        path: P,
        hash_name: S,
    ) -> Option<String> {
        self.data.read().base.hash_for_name(path, hash_name)
    }

    fn contains_hash_for_name<P: AsRef<Path>, S: AsRef<str>>(&self, path: P, hash_name: S) -> bool {
        self.data
            .read()
            .base
            .contains_hash_for_name(path, hash_name)
    }

    fn set_hash_for_name<P: Into<PathBuf>, S: AsRef<str>, H: AsRef<str>>(
        &mut self,
        path: P,
        hash_name: S,
        value: H,
    ) {
        let mut data = self.data.write();
        data.base.set_hash_for_name(path, hash_name, value);
        data.modified = true;
    }
}

#[cfg(test)]
mod test {
    use super::super::base::Base;
    use super::*;

    fn setup() -> SharedFiler<Base> {
        SharedFiler::new(Base::default())
    }

    #[test]
    fn test_startup() {
        let filer = setup();

        assert_eq!(false, filer.mod_flag());
    }

    #[test]
    fn test_no_mods() {
        let filer = setup();
        let path = PathBuf::from("/foo/bar");

        filer.file_known(&path);
        filer.mod_time(&path);
        filer.image_type(&path);
        filer.hash_for_name(&path, "FAKE");
        filer.contains_hash_for_name(&path, "FAKE");

        assert_eq!(false, filer.mod_flag());
    }

    #[test]
    fn test_add_file() {
        let mut filer = setup();
        let path = PathBuf::from("/foo/bar");

        filer.add_file(path);

        assert_eq!(true, filer.mod_flag());
    }
    #[test]
    fn test_set_mod_time() {
        let mut filer = setup();
        let path = PathBuf::from("/foo/bar");

        filer.set_mod_time(path, SystemTime::now());

        assert_eq!(true, filer.mod_flag());
    }
    #[test]
    fn test_set_image_type() {
        let mut filer = setup();
        let path = PathBuf::from("/foo/bar");

        filer.set_image_type(path, ImageType::PNG);

        assert_eq!(true, filer.mod_flag());
    }
    #[test]
    fn test_set_hash_for_name() {
        let mut filer = setup();
        let path = PathBuf::from("/foo/bar");

        filer.set_hash_for_name(path, "FAKE", "VALUE");

        assert_eq!(true, filer.mod_flag());
    }
}
