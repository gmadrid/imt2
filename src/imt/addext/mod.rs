use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::path::Path;

use structopt::StructOpt;
use walkdir::DirEntry;

use crate::imt::crawler::{CrawlHelper, Crawler};
use crate::imt::Result;

enum ImageType {
    JPEG,
    GIF,
    PNG,
}

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

fn read_first_two_bytes(file: &mut File) -> Result<[u8; 2]> {
    let mut bytes = [0u8; 2];
    read_bytes(file, &mut bytes, SeekFrom::Start(0))?;
    Ok(bytes)
}

fn read_last_two_bytes(file: &mut File) -> Result<[u8; 2]> {
    let mut bytes = [0u8; 2];
    read_bytes(file, &mut bytes, SeekFrom::End(-2))?;
    Ok(bytes)
}

fn read_bytes(file: &mut File, buf: &mut [u8], location: SeekFrom) -> Result<()> {
    file.seek(location)?;
    file.read_exact(buf)?;

    Ok(())
}

fn read_first_bytes(file: &mut File, buf: &mut [u8]) -> Result<()> {
    read_bytes(file, buf, SeekFrom::Start(0))
}

fn is_jpeg(file: &mut File) -> Result<bool> {
    let head = read_first_two_bytes(file)?;
    if head != [0xff, 0xd8] {
        return Ok(false);
    }
    let tail = read_last_two_bytes(file)?;
    if tail != [0xff, 0xd9] {
        return Ok(false);
    }
    Ok(true)
}

fn is_png(file: &mut File) -> Result<bool> {
    let mut bytes = [0u8; 8];
    read_first_bytes(file, &mut bytes)?;

    Ok(bytes == [0x89, 0x50, 0x4e, 0x47, 0x0d, 0x0a, 0x1a, 0x0a])
}

fn is_gif(file: &mut File) -> Result<bool> {
    let mut bytes = [0u8; 6];
    read_first_bytes(file, &mut bytes)?;

    Ok(&bytes[0..4] == [0x47, 0x49, 0x46, 0x38] && // 'GIF8'
        (&bytes[4..6] == [0x37, 0x61] || // '7a'
            &bytes[4..6] == [0x39, 0x6a])) // '9a'
}

fn has_extension(path: &Path) -> bool {
    path.extension().map(|e| e.len() > 0).unwrap_or(false)
}

struct Helper {}

fn is_hidden(e: &DirEntry) -> bool {
    let name = e.path().file_name();
    name.map_or(false, |n| n.to_string_lossy().starts_with("."))
}

fn image_type(path: &Path) -> Result<Option<ImageType>> {
    let mut file = File::open(path)?;
    let image_type = if is_jpeg(&mut file)? {
        Some(ImageType::JPEG)
    } else if is_png(&mut file)? {
        Some(ImageType::PNG)
    } else if is_gif(&mut file)? {
        Some(ImageType::GIF)
    } else {
        None
    };
    Ok(image_type)
}

impl CrawlHelper for Helper {
    fn should_descend(&self, e: &DirEntry) -> Result<bool> {
        // Basically just trimming off hidden directories.
        Ok(!is_hidden(e))
    }

    fn should_process_file(&self, e: &DirEntry) -> Result<bool> {
        // We only want to process
        //   1) image files of a format we can determine,
        //   2) that have no file extension.
        let path = e.path();

        // So, if there is an extension, don't process it.
        if has_extension(path) {
            Ok(false)
        } else {
            Ok(image_type(path).unwrap_or(None).map_or(false, |_| true))
        }
    }

    fn process_file(&self, e: &DirEntry) -> Result<()> {
        eprintln!("PROCESS: {}", e.path().display());
        Ok(())
    }
}

pub fn process_addext(ae: &AddExt) -> Result<()> {
    for dir in &ae.directories {
        let crawler = Crawler::new(dir, Helper {});
        crawler.crawl()?;
    }
    Ok(())
}
