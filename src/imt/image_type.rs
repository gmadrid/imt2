use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::path::Path;

use anyhow::Result;

#[derive(Copy, Clone, PartialOrd, PartialEq)]
pub enum ImageType {
    JPEG,
    GIF,
    PNG,

    // Either an image type that we don't know, or not an image.
    UNKNOWN,
}

impl ImageType {
    pub fn type_of_file_at<P: AsRef<Path>>(path: P) -> Result<ImageType> {
        let mut file = File::open(path)?;
        ImageType::type_of_file(&mut file)
    }

    pub fn type_of_file(file: &mut File) -> Result<ImageType> {
        let bytes = read_first_ten_bytes(file)?;
        let image_type = if is_jpeg(file, &bytes)? {
            ImageType::JPEG
        } else if is_png(&bytes)? {
            ImageType::PNG
        } else if is_gif(&bytes)? {
            ImageType::GIF
        } else {
            ImageType::UNKNOWN
        };
        Ok(image_type)
    }

    pub fn preferred_extension(self) -> &'static str {
        match self {
            ImageType::JPEG => "jpg",
            ImageType::GIF => "gif",
            ImageType::PNG => "png",

            ImageType::UNKNOWN => "",
        }
    }
}

type BufType = [u8; 10];

fn read_first_ten_bytes(file: &mut File) -> Result<BufType> {
    let mut buf = [0; 10];
    read_first_bytes(file, &mut buf)?;
    Ok(buf)
}

fn read_first_bytes(file: &mut File, buf: &mut [u8]) -> Result<()> {
    read_bytes(file, buf, SeekFrom::Start(0))
}

fn read_bytes(file: &mut File, buf: &mut [u8], location: SeekFrom) -> Result<()> {
    file.seek(location)?;
    file.read_exact(buf)?;
    Ok(())
}

fn is_jpeg(file: &mut File, buf: &BufType) -> Result<bool> {
    if buf[0..2] != [0xff, 0xd8] {
        return Ok(false);
    }
    let mut tail = [0; 2];
    read_bytes(file, &mut tail, SeekFrom::End(0))?;
    if tail != [0xff, 0xd9] {
        return Ok(false);
    }
    Ok(true)
}

fn is_png(buf: &BufType) -> Result<bool> {
    Ok(buf[0..8] == [0x89, 0x50, 0x4e, 0x47, 0x0d, 0x0a, 0x1a, 0x0a])
}

fn is_gif(buf: &BufType) -> Result<bool> {
    Ok(buf[0..4] == [0x47, 0x49, 0x46, 0x38] && // 'GIF8'
        (buf[4..6] == [0x37, 0x61] || // '7a'
            buf[4..6] == [0x39, 0x6a])) // '9a'
}
