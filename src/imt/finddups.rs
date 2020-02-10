use anyhow::Result;
use structopt::StructOpt;
use walkdir::DirEntry;

use crate::imt::crawler::{Crawler, CrawlHelper};
use crate::imt::direntryutil::is_hidden;
use crate::imt::filer::Filer;

#[derive(StructOpt, Debug)]
pub struct FindDups {
    /// The directories to search
    // TODO: figure out what this does if the filename is not UTF-8.
    #[structopt(min_values(1))]
    directories: Vec<String>,
}

struct FindDupsHelper {
    filer: Option<Filer>,
}

#[derive(Debug, Default)]
struct FindDupsInfo;

impl CrawlHelper for FindDupsHelper {
    type InfoType = FindDupsInfo;

    fn should_descend(&self, e: &DirEntry) -> Result<bool> {
        // Basically just trimming off hidden directories.
        Ok(!is_hidden(e))
    }

    fn should_process_file(&self, e: &DirEntry, _it: &mut Self::InfoType) -> Result<bool> {
        //self.filer.contains_hash(e.path(), )
        unimplemented!()
    }

    fn process_file(&self, e: &DirEntry, _it: &mut Self::InfoType) -> Result<()> {
        unimplemented!()
    }
}

pub fn process_finddups(fd: &FindDups, filer: Option<Filer>) -> Result<()> {
    for dir in &fd.directories {
        let crawler = Crawler::new(
            dir,
            FindDupsHelper { filer: filer.clone() },
        );
        crawler.crawl()?;
    }
    Ok(())
}
