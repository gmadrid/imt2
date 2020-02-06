use imt::process_command;
use structopt::StructOpt;
use simplelog::{TermLogger, Config, TerminalMode};
use log::LevelFilter;

fn main() -> anyhow::Result<()> {
    TermLogger::init(LevelFilter::Info, Config::default(), TerminalMode::Stderr)?;
    process_command(imt::Command::from_args())
}
