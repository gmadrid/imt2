use anyhow::Result;
use structopt::StructOpt;
use walkdir::DirEntry;

use crate::imt::crawler::{CrawlHelper, Crawler};
use crate::imt::direntryutil::is_hidden;
use crate::imt::filer::Filer;

const HASH_NAME: &str = "MD5";

#[derive(StructOpt, Debug)]
pub struct FindDups {
    /// The directories to search
    // TODO: figure out what this does if the filename is not UTF-8.
    #[structopt(min_values(1))]
    directories: Vec<String>,
}

struct FindDupsHelper {
    filer: Filer,
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
        // TODO: add is_image()
        Ok(!self.filer.contains_hash(e.path(), HASH_NAME))
    }

    fn process_file(&self, e: &DirEntry, _it: &mut Self::InfoType) -> Result<()> {
        self.filer.add_file(e.path())?;
        self.filer.add_hash(e.path(), "foo", "bar")?;
        Ok(())
    }
}

pub fn process_finddups(fd: &FindDups, filer: &Filer) -> Result<()> {
    for dir in &fd.directories {
        let crawler = Crawler::new(
            dir,
            FindDupsHelper {
                filer: filer.clone(),
            },
        );
        crawler.crawl()?;
    }
    filer.write_output("foobar")?;
    Ok(())
}
