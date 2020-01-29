mod addext;
mod crawler;
mod error;
//mod fileinfo;
mod finddups;
mod findneardups;
mod opts;

pub use crate::imt::error::{ImtError, Result};
pub use crate::imt::opts::{process_command, Command};
