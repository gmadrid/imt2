use std::fs::File;
use std::io::Read;

use anyhow::Result;
use sha2::{Sha256, Digest};
use structopt::StructOpt;
use walkdir::DirEntry;

use crate::imt::crawler::{CrawlHelper, Crawler};
use crate::imt::direntryutil::is_hidden;
use crate::imt::filer::Filer;

const HASH_NAME: &str = "SHA256";

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
        // TODO: put mod time in filer.
        // TODO: if mod time changes, then delete cached data
        // TODO: don't read file if cached
        self.filer.add_file(e.path());

        let mut hasher = Sha256::new();
        let mut file = File::open(e.path())?;
        let mut buffer = [0; 10000];

        loop {
            let n = file.read(&mut buffer)?;
            if n == 0 {
                break;
            }
            hasher.input(&buffer[0..n]);
        }
        let result = hasher.result();
        let result_str = hex::encode(result);

        self.filer.add_hash(e.path(), HASH_NAME, result_str);
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
