use std::fs::File;

use structopt::StructOpt;
use walkdir::{DirEntry, WalkDir};

use crate::imt::error::Result;
use std::io::{Read, Seek, SeekFrom};

enum ImageType {
    JPEG,
    GIF,
    PNG,
    TIFF,
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
    Ok(false)
}

fn is_tiff(file: &mut File) -> Result<bool> {
    Ok(false)
}

fn is_gif(file: &mut File) -> Result<bool> {
    Ok(false)
}

fn has_extension(e: &DirEntry) -> bool {
    e.path().extension().map(|e| e.len() > 0).unwrap_or(false)
}

fn is_image_without_extension(image_type: &mut Option<ImageType>, e: &DirEntry) -> Result<bool> {
    if has_extension(e) {
        return Ok(false);
    }

    let mut file = File::open(e.path())?;
    Ok(if is_jpeg(&mut file)? {
        *image_type = Some(ImageType::JPEG);
        true
    } else if is_png(&mut file)? {
        *image_type = Some(ImageType::PNG);
        true
    } else if is_gif(&mut file)? {
        *image_type = Some(ImageType::GIF);
        true
    } else if is_tiff(&mut file)? {
        *image_type = Some(ImageType::TIFF);
        true
    } else {
        false
    })
}

pub fn process_addext(ae: &AddExt) -> Result<()> {
    let do_action = !ae.dry_run;

    let mut image_type = None;
    for directory in &ae.directories {
        for entry in WalkDir::new(directory).into_iter().filter_entry(|e| {
            if e.path().is_dir() {
                return true;
            }

            let result = is_image_without_extension(&mut image_type, e);
            match result {
                Ok(b) => b,
                Err(err) => {
                    eprintln!("Error scanning {}: {}", e.path().display(), err);
                    false
                }
            }
        }) {
            // TODO: handle the matched entries here.
            println!("Matched: {}", entry?.path().display());
        }
    }

    //for entry in WalkDir::new()
    Ok(())
}
