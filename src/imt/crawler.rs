use std::path::PathBuf;

use crate::Result;
use walkdir::{DirEntry, WalkDir};

pub struct Crawler<H>
where
    H: CrawlHelper,
{
    path: PathBuf,
    helper: H,
}

pub trait CrawlHelper {
    type InfoType: Default;

    fn handle_error<E>(&self, err: E)
    where
        E: std::error::Error,
    {
        eprintln!("Error: {}", err.description());
    }
    fn should_descend(&self, _e: &DirEntry) -> Result<bool> {
        Ok(true)
    }
    fn process_directory(&self, _e: &DirEntry) -> Result<()> {
        Ok(())
    }
    fn should_process_file(&self, _e: &DirEntry, _it: &mut Self::InfoType) -> Result<bool> {
        Ok(true)
    }
    fn process_file(&self, e: &DirEntry, it: &mut Self::InfoType) -> Result<()>;
}

struct EntryInfo<T> where T: Default {
    entry: DirEntry,
    info: T,
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
        for mut ei in WalkDir::new(&self.path).into_iter().filter_map(|re| {
            re.map_err(|err| self.helper.handle_error(err))
                .ok()
                .and_then(|e| {
                    let mut ei = EntryInfo { entry: e, info: H::InfoType::default() };
                    if self.filter(&mut ei) {
                        Some(ei)
                    } else {
                        None
                    }
                })
        }) {
            match self.process_entry(&mut ei) {
                Err(err) => self.helper.handle_error(err),
                Ok(_) => {}
            }
        }

        Ok(())
    }

    fn filter(&self, ei: &mut EntryInfo<H::InfoType>) -> bool {
        let path = ei.entry.path();
        if path.is_dir() {
            self.filter_dir(ei)
        } else if path.is_file() && path.exists() {
            self.filter_file(ei)
        } else {
            false
        }
    }

    fn filter_dir(&self, ei: &EntryInfo<H::InfoType>) -> bool {
        self.helper
            .should_descend(&ei.entry)
            .map_err(|e| self.helper.handle_error(e))
            .unwrap_or(false)
    }

    fn filter_file(&self, ei: &mut EntryInfo<H::InfoType>) -> bool {
        self.helper
            .should_process_file(&ei.entry, &mut ei.info)
            .map_err(|e| self.helper.handle_error(e))
            .unwrap_or(false)
    }

    fn process_entry(&self, ei: &mut EntryInfo<H::InfoType>) -> Result<()> {
        if ei.entry.path().is_dir() {
            self.process_dir(&ei.entry)
        } else {
            self.process_file(ei)
        }
    }

    fn process_dir(&self, e: &DirEntry) -> Result<()> {
        self.helper.process_directory(e)
    }

    fn process_file(&self, ei: &mut EntryInfo<H::InfoType>) -> Result<()> {
        self.helper.process_file(&ei.entry, &mut ei.info)
    }
}
