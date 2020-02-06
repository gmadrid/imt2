use std::path::PathBuf;

use anyhow::Result;
use log::{debug, error, info};
use std::error::Error;
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

    fn handle_error(&self, err: &anyhow::Error) {
        log::error!("{}", err.description());
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

struct EntryInfo<T>
where
    T: Default,
{
    entry: DirEntry,
    info: T,
}

impl<T> EntryInfo<T>
where
    T: Default,
{
    fn path_to_display(&self) -> std::path::Display {
        self.entry.path().display()
    }
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
        // This is basically a for loop, but we have to expand it out
        // and write it ourselves so that we can call it.skip_current_dir().
        // We cannot use filter_entry() directly since we want to create
        // EntryInfos to pass to all of the helpers.
        // TODO: return Result from all of the helpers and DTRT with errors.
        debug!("Walking into {}", self.path.display());
        let mut it = WalkDir::new(&self.path).into_iter();
        loop {
            let entry = match it.next() {
                None => break,
                Some(Err(err)) => {
                    error!("Error getting next DirEntry: {}", err.description());
                    self.helper.handle_error(&(err.into()));
                    continue;
                }
                Some(Ok(e)) => e,
            };
            let mut ei = EntryInfo {
                entry,
                info: H::InfoType::default(),
            };
            match self.filter(&mut ei) {
                Err(err) => {
                    error!(
                        "Error processing filter for file {}: {}",
                        ei.path_to_display(),
                        err.description()
                    );
                    self.helper.handle_error(&(err.into()));
                }
                Ok((b, is_dir)) => {
                    if b {
                        if let Err(err) = self.process_entry(&mut ei) {
                            error!(
                                "Error processing entry for {}: {}",
                                ei.path_to_display(),
                                err.description()
                            );
                            self.helper.handle_error(&err);
                        }
                    } else {
                        debug!("Skipping {}", ei.path_to_display());
                        if is_dir {
                            info!("Not walking into {}", ei.path_to_display());
                            it.skip_current_dir();
                        }
                    }
                }
            }
        }
        Ok(())
    }

    // Returns => Ok((filter, is_dir))
    fn filter(&self, ei: &mut EntryInfo<H::InfoType>) -> Result<(bool, bool)> {
        let path = ei.entry.path();
        Ok(if path.is_dir() {
            (self.filter_dir(ei)?, true)
        } else if path.is_file() && path.exists() {
            (self.filter_file(ei)?, false)
        } else {
            (false, false)
        })
    }

    fn filter_dir(&self, ei: &EntryInfo<H::InfoType>) -> Result<bool> {
        self.helper.should_descend(&ei.entry)
    }

    fn filter_file(&self, ei: &mut EntryInfo<H::InfoType>) -> Result<bool> {
        self.helper.should_process_file(&ei.entry, &mut ei.info)
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
