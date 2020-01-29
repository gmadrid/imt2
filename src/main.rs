use imt::{process_command, Result};
use structopt::StructOpt;

fn main() -> Result<()> {
    process_command(imt::Command::from_args())
}
