use imt::{process_command, Result};
use structopt::StructOpt;

fn main() -> Result<()> {
    let command = imt::Command::from_args();
    process_command(command)
}
