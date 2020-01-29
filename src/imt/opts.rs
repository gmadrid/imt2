use structopt::StructOpt;

use crate::imt::addext::{self, process_addext};
use crate::imt::error::Result;
use crate::imt::fileinfo::{FileInfo, Files};
use crate::imt::finddups::{self, process_finddups};
use crate::imt::findneardups::{self, process_findneardups};

#[derive(StructOpt, Debug)]
#[structopt(name = "imt2", about = "image tools")]
pub enum Command {
    AddExt(addext::AddExt),
    FindDups(finddups::FindDups),
    FindNearDups(findneardups::FindNearDups),
}

fn _exercise_serialization() {
    let mut files = Files::default();

    let mut fi = FileInfo::new("first/bar/baz".into());
    fi.add_hash("foo".into(), "foovalueofthehash".into());
    fi.add_hash("bar".into(), "barvalueofthehash".into());
    files.add_file(fi);

    let mut fi = FileInfo::new("second/quux.jpg".into());
    fi.add_hash("quux".into(), "quuxvalueofhash".into());
    fi.add_hash("quip".into(), "quipvalueofhash".into());
    files.add_file(fi);

    let s = toml::to_string(&files).unwrap();
    println!("THETOML: {}", s);

    let newfiles = toml::from_str::<Files>(&s).unwrap();
    println!("RECON: {:?}", newfiles);
}

pub fn process_command(command: Command) -> Result<()> {
    //_exercise_serialization();

    match command {
        Command::AddExt(ae) => process_addext(&ae),
        Command::FindDups(fd) => process_finddups(&fd),
        Command::FindNearDups(fnd) => process_findneardups(&fnd),
    }
}
