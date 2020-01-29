use std::path::{PathBuf};

use crate::{ImtError, Result};
use walkdir::{DirEntry, WalkDir};

pub struct Crawler<H>
where
    H: CrawlHelper,
{
    path: PathBuf,
    helper: H,
}

pub trait CrawlHelper {
    fn handle_error<E>(&self, err: E) -> E
    where
        E: std::error::Error,
    {
        eprintln!("Error: {}", err.description());
        err
    }
    fn should_descend(&self, _e: &DirEntry) -> Result<bool> {
        Ok(true)
    }
    fn process_directory(&self, _e: &DirEntry) -> Result<()> {
        Ok(())
    }
    fn should_process_file(&self, _e: &DirEntry) -> Result<bool> {
        Ok(true)
    }
    fn process_file(&self, e: &DirEntry) -> Result<()>;
}

impl<H> Crawler<H>
where
    H: CrawlHelper,
{
    pub fn new<P>(p: P, h: H) -> Crawler<H>
    where
        PathBuf: From<P>,
    {
        Crawler {
            path: PathBuf::from(p),
            helper: h,
        }
    }

    pub fn crawl(&self) -> Result<()> {
        for entry in WalkDir::new(&self.path)
            .into_iter()
            .filter_entry(|e| self.filter(e))
        {
            let result = entry
                .map_err(|err| ImtError::from(err))
                .and_then(|e| self.process_entry(&e));
            if result.is_err() {
                // unwrap: safe because we are inside is_err() case.
                self.helper.handle_error(result.unwrap_err());
            }
        }

        Ok(())
    }

    fn filter(&self, e: &DirEntry) -> bool {
        if e.path().is_dir() {
            self.filter_dir(e)
        } else if e.path().is_file() && e.path().exists() {
            self.filter_file(e)
        } else {
            false
        }
    }

    fn filter_dir(&self, e: &DirEntry) -> bool {
        self.helper
            .should_descend(e)
            .map_err(|e| self.helper.handle_error(e))
            .unwrap_or(false)
    }

    fn filter_file(&self, e: &DirEntry) -> bool {
        self.helper
            .should_process_file(e)
            .map_err(|e| self.helper.handle_error(e))
            .unwrap_or(false)
    }

    fn process_entry(&self, e: &DirEntry) -> Result<()> {
        if e.path().is_dir() {
            self.process_dir(e)
        } else {
            self.process_file(e)
        }
    }

    fn process_dir(&self, e: &DirEntry) -> Result<()> {
        self.helper
            .process_directory(e)
            .map_err(|e| self.helper.handle_error(e))
    }

    fn process_file(&self, e: &DirEntry) -> Result<()> {
        self.helper
            .process_file(e)
            .map_err(|e| self.helper.handle_error(e))
    }
}
