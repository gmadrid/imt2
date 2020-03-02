use crate::imt::filer::FilerTrait;
use crate::imt::image_type::ImageType;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::borrow::Borrow;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

#[derive(Default, Deserialize, Serialize, Debug)]
pub struct FileInfo {
    hashes: HashMap<String, String>,
    image_type: Option<ImageType>,
    mod_time: Option<SystemTime>,
}

impl FileInfo {
    pub fn mod_time(&self) -> Option<SystemTime> {
        self.mod_time
    }

    /// Set the mod time, clearing out any other file data (because it may have changed).
    pub fn set_mod_time(&mut self, time: SystemTime) {
        self.mod_time = Some(time);
        self.image_type = None;
        self.hashes.clear();
    }

    pub fn image_type(&self) -> Option<ImageType> {
        self.image_type
    }

    pub fn set_image_type(&mut self, image_type: ImageType) {
        self.image_type = Some(image_type)
    }

    pub fn add_hash<H, V>(&mut self, hash_name: H, hash_value: V)
    where
        H: Into<String>,
        V: Into<String>,
    {
        self.hashes.insert(hash_name.into(), hash_value.into());
    }

    pub fn contains_hash<H>(&self, hash_name: H) -> bool
    where
        H: AsRef<str>,
    {
        self.hashes.contains_key(hash_name.as_ref())
    }

    pub fn hash_value<H>(&self, hash_name: H) -> Option<String>
    where
        H: AsRef<str>,
    {
        self.hashes.get(hash_name.as_ref()).cloned()
    }
}

#[cfg(test)]
mod test {
    use crate::imt::filer::fileinfo::FileInfo;
    use crate::imt::image_type::ImageType;
    use std::ops::Sub;
    use std::time::{Duration, Instant, SystemTime};

    #[test]
    fn test_mod_time() {
        let mut fi = FileInfo::default();

        let now = SystemTime::now();
        let yesterday = now.sub(Duration::new(60 * 60 * 24, 0));

        assert_eq!(None, fi.mod_time());

        fi.set_mod_time(yesterday);
        assert_eq!(yesterday, fi.mod_time().unwrap());

        fi.set_image_type(ImageType::PNG);
        fi.add_hash("FAKE", "HASH");

        assert_eq!(ImageType::PNG, fi.image_type().unwrap());
        assert!(fi.contains_hash("FAKE"));

        fi.set_mod_time(now);
        assert_eq!(now, fi.mod_time().unwrap());

        assert_eq!(None, fi.image_type());
        assert!(!fi.contains_hash("FAKE"));
    }

    #[test]
    fn test_image_type() {
        let mut fi = FileInfo::default();
        assert_eq!(None, fi.image_type());

        fi.set_image_type(ImageType::JPEG);
        assert_eq!(ImageType::JPEG, fi.image_type().unwrap());

        fi.set_image_type(ImageType::GIF);
        assert_eq!(ImageType::GIF, fi.image_type().unwrap());
    }

    #[test]
    fn test_hashes() {
        let mut fi = FileInfo::default();

        assert!(!fi.contains_hash("FAKE"));
        assert!(!fi.contains_hash("MISSING"));

        assert_eq!(None, fi.hash_value("FAKE"));
        assert_eq!(None, fi.hash_value("MISSING"));

        fi.add_hash("FAKE", "VALUE");

        assert!(fi.contains_hash("FAKE"));
        assert!(!fi.contains_hash("MISSING"));

        assert_eq!("VALUE", fi.hash_value("FAKE").unwrap());
        assert_eq!(None, fi.hash_value("MISSING"));
    }
}

// =========================================================
/*

#[derive(Default, Debug, Deserialize, Serialize)]
pub struct Files {
    files: HashMap<PathBuf, FileInfo>,
}

impl FilerTrait for Files {
    fn add_file<P: Into<PathBuf>>(&mut self, path: P) {
        self.files.entry(path).or_default();
    }

    fn mod_time<P: AsRef<Path>>(&self, path: P) -> Option<SystemTime> {
        unimplemented!()
    }

    fn set_mod_time<P: AsRef<Path>>(&self, path: P, time: SystemTime) {
        unimplemented!()
    }

    fn image_type<P: AsRef<Path>>(&self, path: P) -> Option<ImageType> {
        unimplemented!()
    }

    fn set_image_type<P: AsRef<Path>>(&mut self, path: P, image_type: ImageType) {
        unimplemented!()
    }

    fn hash_for_name<P: AsRef<Path>, S: AsRef<str>>(&self, path: P, hash_name: S) -> Option<String> {
        unimplemented!()
    }

    fn contains_hash_for_name<P: AsRef<Path>, S: AsRef<str>>(&self, path: P, hash_name: S) -> bool {
        unimplemented!()
    }

    fn set_hash_for_name<P: AsRef<Path>, S: AsRef<str>, H: AsRef<str>>(&self, path: P, hash_name: S, value: H) {
        unimplemented!()
    }

    fn with_files<F: FnMut(&Path)>(&self, f: F) {
        unimplemented!()
    }
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

impl FileInfo {
    pub fn new() -> FileInfo {
        FileInfo {
            hashes: HashMap::new(),
            image_type: Option::default(),
            mod_time: SystemTime::default(),
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
*/
