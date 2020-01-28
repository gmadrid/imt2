use crate::imt::error::Result;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
pub struct AddExt {
}

pub fn process_addext(_ae: &AddExt) -> Result<()> {
    unimplemented!()
}
