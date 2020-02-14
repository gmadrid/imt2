use std::path::Path;

use anyhow::Result;
use log::{info, warn};
use structopt::StructOpt;
use walkdir::DirEntry;

use super::crawler::{CrawlHelper, Crawler};
use super::direntryutil::is_hidden;
use super::filer::Filer;
use super::image_type::ImageType;

/// Add extensions to image files with no extensions.
#[derive(StructOpt, Debug)]
pub struct AddExt {
    /// Print actions only.
    #[structopt(short = "n", long)]
    dry_run: bool,

    /// The directories to search
    // TODO: figure out what this does if the filename is not UTF-8.
    #[structopt(min_values(1))]
    directories: Vec<String>,
}

fn has_extension(path: &Path) -> bool {
    path.extension().map(|e| !e.is_empty()).unwrap_or(false)
}

struct Helper<'a> {
    dry_run: bool,
    filer: &'a Filer,
}

#[derive(Default)]
struct Info {
    image_type: Option<ImageType>,
}

impl Info {
    fn image_type(&mut self, e: &DirEntry) -> Result<ImageType> {
        match self.image_type {
            Some(it) => Ok(it),
            None => {
                let it = ImageType::type_of_file_at(e.path())?;
                self.image_type = Some(it);
                Ok(it)
            }
        }
    }
}

impl<'a> CrawlHelper for Helper<'a> {
    type InfoType = Info;

    fn should_descend(&self, e: &DirEntry) -> Result<bool> {
        // Basically just trimming off hidden directories.
        Ok(!is_hidden(e))
    }

    fn should_process_file(&self, e: &DirEntry, it: &mut Self::InfoType) -> Result<bool> {
        // We only want to process
        //   1) image files of a format we can determine,
        //   2) that have no file extension.
        let path = e.path();

        if has_extension(path) {
            Ok(false)
        } else {
            Ok(it.image_type(e)? != ImageType::UNKNOWN)
        }
    }

    fn process_file(&self, e: &DirEntry, it: &mut Self::InfoType) -> Result<()> {
        self.filer.add_file(e.path())?;

        let image_type = it.image_type(e)?;

        // should_process_file should filter out anything that is not an image.
        assert!(image_type != ImageType::UNKNOWN);

        let path = e.path();
        let ext = image_type.preferred_extension();
        info!("Adding {} extension to {}.", ext, path.display());
        if self.dry_run {
            info!("Dry run. File operation skipped.");
            eprintln!("Adding '{}' to {}", ext, path.display());
        } else {
            let mut new_name = path.to_path_buf();
            if new_name.set_extension(ext) {
                // TODO: verbose option?
                std::fs::rename(path, new_name)?;
            } else {
                warn!("Failed to add extension, {}, to {}", ext, path.display());
            }
        }
        Ok(())
    }
}

pub fn process_addext(ae: &AddExt, filer: &Filer) -> Result<()> {
    for dir in &ae.directories {
        let crawler = Crawler::new(
            dir,
            Helper {
                dry_run: ae.dry_run,
                filer,
            },
        );
        crawler.crawl()?;
    }
    Ok(())
}
