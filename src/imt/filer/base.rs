use crate::imt::filer::fileinfo::FileInfo;
use crate::imt::filer::FilerTrait;
use crate::imt::image_type::ImageType;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

// NEEDS
// - background saving
// - mutation aware to avoid saving unchanged data
// - iterate over all files
// - save multiple named hashes for each file
// - record mod time of file
//   - erase saved hashes and other data when mod time changes.
// - save image type

type FilesType = HashMap<PathBuf, FileInfo>;

/// A Base implementation of FilerTrait.
#[derive(Default, Debug)]
pub struct Base {
    files: FilesType,
}

impl Base {
    fn files_iter(&self) -> impl Iterator<Item = &PathBuf> {
        self.files.keys()
    }
}

impl FilerTrait for Base {
    fn add_file<P: Into<PathBuf>>(&mut self, path: P) {
        self.files.entry(path.into()).or_default();
    }

    fn file_known<P: AsRef<Path>>(&self, path: P) -> bool {
        self.files.contains_key(path.as_ref())
    }

    fn mod_time<P: AsRef<Path>>(&self, path: P) -> Option<SystemTime> {
        self.files.get(path.as_ref()).and_then(|fi| fi.mod_time())
    }

    fn set_mod_time<P: Into<PathBuf>>(&mut self, path: P, time: SystemTime) {
        self.files
            .entry(path.into())
            .or_default()
            .set_mod_time(time);
    }

    fn image_type<P: AsRef<Path>>(&self, path: P) -> Option<ImageType> {
        self.files.get(path.as_ref()).and_then(|fi| fi.image_type())
    }

    fn set_image_type<P: Into<PathBuf>>(&mut self, path: P, image_type: ImageType) {
        self.files
            .entry(path.into())
            .or_default()
            .set_image_type(image_type);
    }

    fn hash_for_name<P: AsRef<Path>, S: AsRef<str>>(
        &self,
        path: P,
        hash_name: S,
    ) -> Option<String> {
        self.files
            .get(path.as_ref())
            .and_then(|fi| fi.hash_value(hash_name.as_ref()))
    }

    fn contains_hash_for_name<P: AsRef<Path>, S: AsRef<str>>(&self, path: P, hash_name: S) -> bool {
        self.files
            .get(path.as_ref())
            .map(|fi| fi.contains_hash(hash_name.as_ref()))
            .unwrap_or(false)
    }

    fn set_hash_for_name<P: Into<PathBuf>, S: AsRef<str>, H: AsRef<str>>(
        &mut self,
        path: P,
        hash_name: S,
        value: H,
    ) {
        self.files
            .entry(path.into())
            .or_default()
            .add_hash(hash_name.as_ref(), value.as_ref());
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::time::Duration;

    #[test]
    fn test_default() {
        let _base = Base::default();
    }

    #[test]
    fn test_add_files() {
        let mut base = Base::default();
        let name = PathBuf::from("/foo/bar");

        assert!(!base.file_known(&name));
        base.add_file(&name);
        assert!(base.file_known(&name))
    }

    #[test]
    fn test_add_files_doesnt_overwrite() {
        let mut base = Base::default();
        let name = PathBuf::from("/foo/bar");

        base.add_file(&name);
        base.set_image_type(&name, ImageType::PNG);
        assert_eq!(ImageType::PNG, base.image_type(&name).unwrap());

        // Add it again.
        base.add_file(&name);

        // Make sure that the image type is still there.
        assert_eq!(ImageType::PNG, base.image_type(&name).unwrap());
    }

    #[test]
    fn test_add_hash() {
        let mut base = Base::default();
        let name = PathBuf::from("/foo/bar");
        let hash_name = "FAKE";
        let hash_value = "VALUE";

        base.add_file(&name);
        assert!(!base.contains_hash_for_name(&name, hash_name));
        assert_eq!(None, base.hash_for_name(&name, hash_name));

        base.set_hash_for_name(&name, hash_name, hash_value);
        assert!(base.contains_hash_for_name(&name, hash_name));
        assert_eq!("VALUE", base.hash_for_name(&name, hash_name).unwrap());
    }

    #[test]
    fn test_reset_values() {
        let mut base = Base::default();
        let name = PathBuf::from("/foo/bar");
        let mod_time = SystemTime::now();
        let hash_name = "FAKE";
        let hash_value = "VALUE1";

        base.add_file(&name);
        base.set_image_type(&name, ImageType::PNG);
        base.set_mod_time(&name, mod_time);
        base.set_hash_for_name(&name, hash_name, hash_value);

        // Don't test mod time, since that will clear the values.
        let new_hash_value = "VALUE2";
        base.set_image_type(&name, ImageType::GIF);
        base.set_hash_for_name(&name, hash_name, new_hash_value);
        assert_eq!(ImageType::GIF, base.image_type(&name).unwrap());
        assert_eq!(
            new_hash_value,
            base.hash_for_name(&name, hash_name).unwrap()
        );
    }

    #[test]
    fn test_mod_time() {
        let mut base = Base::default();
        let name = PathBuf::from("/foo/bar");
        let mod_time = SystemTime::now();

        base.add_file(&name);
        assert_eq!(None, base.mod_time(&name));

        base.set_mod_time(&name, mod_time);
        assert_eq!(mod_time, base.mod_time(&name).unwrap());
    }

    #[test]
    fn test_multiple_files() {
        let mut base = Base::default();
        let name1 = PathBuf::from("/foo/bar");
        let name2 = PathBuf::from("/foo/quux");

        base.set_image_type(&name1, ImageType::GIF);
        base.set_image_type(&name2, ImageType::PNG);

        assert_eq!(ImageType::GIF, base.image_type(&name1).unwrap());
        assert_eq!(ImageType::PNG, base.image_type(&name2).unwrap());
    }

    #[test]
    fn test_multiple_hashes() {
        let mut base = Base::default();
        let name = PathBuf::from("/foo/bar");
        let hash_name1 = "FAKE1";
        let hash_value1 = "VALUE1";
        let hash_name2 = "FAKE2";
        let hash_value2 = "VALUE2";

        assert_eq!(None, base.hash_for_name(&name, hash_name1));
        assert_eq!(None, base.hash_for_name(&name, hash_name2));

        base.set_hash_for_name(&name, hash_name1, hash_value1);
        base.set_hash_for_name(&name, hash_name2, hash_value2);

        assert_eq!(hash_value1, base.hash_for_name(&name, hash_name1).unwrap());
        assert_eq!(hash_value2, base.hash_for_name(&name, hash_name2).unwrap());
    }

    #[test]
    fn test_adding_data_adds_files() {
        let mut base = Base::default();
        let now = SystemTime::now();
        let name1 = PathBuf::from("/foo/bar1");
        let name2 = PathBuf::from("/foo/bar2");
        let name3 = PathBuf::from("/foo/bar3");
        let hash_name = "FAKE";
        let hash_value = "VALUE";

        base.set_mod_time(&name1, now);
        base.set_image_type(&name2, ImageType::PNG);
        base.set_hash_for_name(&name3, hash_name, hash_value);

        assert!(base.file_known(&name1));
        assert!(base.file_known(&name2));
        assert!(base.file_known(&name3));
    }

    #[test]
    fn test_reading_data_doesnt_add_files() {
        let mut base = Base::default();
        let name = PathBuf::from("/foo/bar");

        assert!(!base.file_known(&name));

        assert_eq!(None, base.image_type(&name));
        assert_eq!(None, base.mod_time(&name));
        assert_eq!(None, base.hash_for_name(&name, "FAKE"));

        assert!(!base.file_known(&name));
    }

    #[test]
    fn test_iterator() {
        let mut base = Base::default();
        let name1 = PathBuf::from("/foo/bar1");
        let name2 = PathBuf::from("/foo/bar2");
        let name3 = PathBuf::from("/foo/bar3");

        base.add_file(&name1);
        base.add_file(&name2);
        base.add_file(&name3);

        let mut foo = base.files_iter().cloned().collect::<Vec<_>>();
        foo.sort();
        assert_eq!(vec! { name1, name2, name3 }, foo);
    }
}
