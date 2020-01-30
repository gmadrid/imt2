use anyhow::Result;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
pub struct FindNearDups {}

pub fn process_findneardups(_fd: &FindNearDups) -> Result<()> {
    unimplemented!()
}
