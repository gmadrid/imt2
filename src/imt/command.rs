use anyhow::Result;
use structopt::StructOpt;

use crate::imt::addext::{process_addext, AddExt};
use crate::imt::filer::FilerTrait;
//use crate::imt::finddups::{process_finddups, FindDups};
//use crate::imt::findneardups::{process_findneardups, FindNearDups};

pub struct Filer();

#[derive(StructOpt, Debug)]
#[structopt(name = "imt2", about = "image tools")]
pub enum Command {
    AddExt(AddExt),
//    FindDups(FindDups),
//    FindNearDups(FindNearDups),
}

pub fn process_command<F: FilerTrait>(command: Command, filer: &mut F) -> Result<()> {
    match command {
        Command::AddExt(ae) => process_addext(&ae, filer),
//        Command::FindDups(fd) => process_finddups(&fd, filer),
//        Command::FindNearDups(fnd) => process_findneardups(&fnd),
    }
}
