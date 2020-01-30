use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::path::Path;

use structopt::StructOpt;
use walkdir::DirEntry;

use crate::imt::crawler::{CrawlHelper, Crawler};
use crate::imt::Result;

// TODO: figure out why it's seeing a file in the .hidden directory.

#[derive(Copy, Clone)]
enum ImageType {
    JPEG,
    GIF,
    PNG,
}

impl ImageType {
    fn preferred_extension(self) -> &'static str {
        match self {
            ImageType::JPEG => "jpg",
            ImageType::GIF => "gif",
            ImageType::PNG => "png",
        }
    }
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

fn is_jpeg(file: &mut File, buf: &InfoBufType) -> Result<bool> {
    if buf[0..2] != [0xff, 0xd8] {
        return Ok(false);
    }
    let tail = read_last_two_bytes(file)?;
    if tail != [0xff, 0xd9] {
        return Ok(false);
    }
    Ok(true)
}

fn is_png(buf: &InfoBufType) -> Result<bool> {
    Ok(buf[0..8] == [0x89, 0x50, 0x4e, 0x47, 0x0d, 0x0a, 0x1a, 0x0a])
}

fn is_gif(buf: &InfoBufType) -> Result<bool> {
    Ok(buf[0..4] == [0x47, 0x49, 0x46, 0x38] && // 'GIF8'
        (buf[4..6] == [0x37, 0x61] || // '7a'
            buf[4..6] == [0x39, 0x6a])) // '9a'
}

fn has_extension(path: &Path) -> bool {
    path.extension().map(|e| !e.is_empty()).unwrap_or(false)
}

struct Helper {}

fn is_hidden(e: &DirEntry) -> bool {
    let name = e.path().file_name();
    name.map_or(false, |n| n.to_string_lossy().starts_with('.'))
}

fn image_type(file: &mut File, bytes: &InfoBufType) -> Result<Option<ImageType>> {
    let image_type = if is_jpeg(file, bytes)? {
        Some(ImageType::JPEG)
    } else if is_png(bytes)? {
        Some(ImageType::PNG)
    } else if is_gif(bytes)? {
        Some(ImageType::GIF)
    } else {
        None
    };
    Ok(image_type)
}

#[derive(Default)]
struct Info {
    image_type: Option<Option<ImageType>>,
    buffer: Option<InfoBufType>,
}

type InfoBufType = [u8; 10];

impl Info {
    fn image_type(&mut self, e: &DirEntry) -> Result<Option<ImageType>> {
        match self.image_type {
            Some(it) => Ok(it),
            None => {
                let mut file = File::open(e.path())?;
                let bytes = self.first_ten_bytes(&mut file)?;
                let it = image_type(&mut file, bytes)?;
                self.image_type = Some(it);
                Ok(it)
            }
        }
    }

    fn first_ten_bytes(&mut self, file: &mut File) -> Result<&InfoBufType> {
        if self.buffer.is_none() {
            let mut buf = [0; 10];
            read_first_bytes(file, &mut buf)?;
            self.buffer = Some(buf);
        }

        // unwrap: either !is_none, or we have just filled in a value.
        Ok(&self.buffer.as_ref().unwrap())
    }
}

impl CrawlHelper for Helper {
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
            Ok(it.image_type(e).unwrap_or(None).map_or(false, |_| true))
        }
    }

    fn process_file(&self, e: &DirEntry, it: &mut Self::InfoType) -> Result<()> {
        let image_type = it.image_type(e)?;
        eprintln!(
            "PROCESS: {}: {}",
            image_type.map_or("???", |it| it.preferred_extension()),
            e.path().display()
        );
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
