use std::path::{Path, PathBuf};

use crate::imt::error::Result;
use walkdir::{WalkDir, DirEntry};
use std::error::Error;

pub struct Crawler<H> where H: CrawlHelper {
    path: PathBuf,
    helper: H,
}

pub trait CrawlHelper {
    fn handle_error<E>(&self, err: E) -> Result<()> where E: std::error::Error;

}

impl<H> Crawler<H> where H: CrawlHelper {
    pub fn new<P>(p: P, h: H) -> Crawler<H> where PathBuf: From<P> {
        Crawler { path: PathBuf::from(p), helper: h }
    }

    pub fn crawl(&self) -> Result<()>  {
        for entry in WalkDir::new(&self.path).into_iter().filter_entry(|e| self.filter(e)) {
            self.process_entry(&entry?);
        }

//        for entry in WalkDir::new(&self.path) {
//            match entry {
//                Err(err) => {
//                    helper.handle_error(err)?;
//                }
//                Ok(entry) => {
//                    if entry.path().is_dir() && helper.should_descend(entry) {
//
//                    }
//                }
//            }
//        }


        Ok(())
    }

    pub fn filter(&self, e: &DirEntry) -> bool {
        if e.path().is_dir() {
            self.filter_dir(e)
        } else if e.path().is_file() && e.path().exists() {
            self.filter_file(e)
        } else {
            false
        }
    }

    pub fn filter_dir(&self, e: &DirEntry) -> bool {
        unimplemented!()
    }
    pub fn filter_file(&self, e: &DirEntry) -> bool { unimplemented!()}

    pub fn process_entry(&self, e: &DirEntry) -> Result<()> {
        unimplemented!()
    }
}