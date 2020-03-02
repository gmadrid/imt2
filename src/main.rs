/*
use anyhow::Result;
use imt::{process_command, Command, Filer};
use log::LevelFilter;
use simplelog::{CombinedLogger, Config, SharedLogger, TermLogger, TerminalMode, WriteLogger};
use std::fs::File;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
pub struct Opts {
    /// If true, log (INFO) to stderr.
    #[structopt(long)]
    log: bool,

    /// If set, log (DEBUG) to a file.
    #[structopt(long = "log_file")]
    log_file: Option<String>,

    #[structopt(subcommand)]
    command: Command,
}

fn set_up_logs(opts: &Opts) -> Result<()> {
    let mut logs: Vec<Box<dyn SharedLogger>> = Vec::new();

    if opts.log {
        // unwrap: DANGER!
        logs.push(
            TermLogger::new(LevelFilter::Info, Config::default(), TerminalMode::Stderr).unwrap(),
        );
    }

    if let Some(filename) = &opts.log_file {
        let file = File::create(filename)?;
        logs.push(WriteLogger::new(
            LevelFilter::Debug,
            Config::default(),
            file,
        ));
    }

    CombinedLogger::init(logs)?;
    Ok(())
}

fn start_filer() -> Result<Filer> {
    Filer::new()
}
*/
fn main() -> anyhow::Result<()> {
    /*
        let opts = Opts::from_args();
        set_up_logs(&opts)?;

        let filer = start_filer()?;

        process_command(opts.command, &filer)?;
        filer.write_to_path("files.toml")?;
    */

    Ok(())
}
