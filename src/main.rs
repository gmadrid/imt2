use imt::{process_command};
use structopt::StructOpt;

fn main() -> anyhow::Result<()> {
    process_command(imt::Command::from_args())
}
