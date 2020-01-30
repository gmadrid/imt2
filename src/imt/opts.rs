use anyhow::Result;
use structopt::StructOpt;

use crate::imt::addext::{process_addext, AddExt};
//use crate::imt::fileinfo::{FileInfo, Files};
use crate::imt::finddups::{process_finddups, FindDups};
use crate::imt::findneardups::{process_findneardups, FindNearDups};

#[derive(StructOpt, Debug)]
#[structopt(name = "imt2", about = "image tools")]
pub enum Command {
    AddExt(AddExt),
    FindDups(FindDups),
    FindNearDups(FindNearDups),
}

//fn _exercise_serialization() {
//    let mut files = Files::default();
//
//    let mut fi = FileInfo::new("first/bar/baz".into());
//    fi.add_hash("foo".into(), "foovalueofthehash".into());
//    fi.add_hash("bar".into(), "barvalueofthehash".into());
//    files.add_file(fi);
//
//    let mut fi = FileInfo::new("second/quux.jpg".into());
//    fi.add_hash("quux".into(), "quuxvalueofhash".into());
//    fi.add_hash("quip".into(), "quipvalueofhash".into());
//    files.add_file(fi);
//
//    let s = toml::to_string(&files).unwrap();
//    println!("THETOML: {}", s);
//
//    let newfiles = toml::from_str::<Files>(&s).unwrap();
//    println!("RECON: {:?}", newfiles);
//}
//
pub fn process_command(command: Command) -> Result<()> {
    //_exercise_serialization();

    match command {
        Command::AddExt(ae) => process_addext(&ae),
        Command::FindDups(fd) => process_finddups(&fd),
        Command::FindNearDups(fnd) => process_findneardups(&fnd),
    }
}
