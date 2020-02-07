use anyhow::Result;
use structopt::StructOpt;

use crate::imt::addext::{process_addext, AddExt};
use crate::imt::filer::Filer;
use crate::imt::finddups::{process_finddups, FindDups};
use crate::imt::findneardups::{process_findneardups, FindNearDups};

#[derive(StructOpt, Debug)]
#[structopt(name = "imt2", about = "image tools")]
pub enum Command {
    AddExt(AddExt),
    FindDups(FindDups),
    FindNearDups(FindNearDups),
}

pub fn process_command(command: Command, filer: Option<Filer>) -> Result<()> {
    match command {
        Command::AddExt(ae) => process_addext(&ae, filer),
        Command::FindDups(fd) => process_finddups(&fd),
        Command::FindNearDups(fnd) => process_findneardups(&fnd),
    }
}
