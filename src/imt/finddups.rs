use crate::imt::error::Result;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
pub struct FindDups {}

pub fn process_finddups(_fd: &FindDups) -> Result<()> {
    unimplemented!()
}
