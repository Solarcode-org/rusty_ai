//! CLI Definitions + Documentation

use clap::{Parser, Subcommand};

/// The best home-cooked AI of the century!
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about=format!(r"
The best home-cooked AI of the century!
{}
Wake up this crab from its sleep and talk.
***Beware, he's kind of an introvert***", super::art::CRAB_ART[0]))]
pub(crate) struct Args {
    #[command(subcommand)]
    pub(crate) command: Commands,

    /// Verbosity level (0-6)
    #[arg(short, long, default_value_t=0)]
    verbose: u8,

    /// MEGA-CRAB SPECIAL DEVELOPERS PASS (password: **********)
    #[arg(short='X', long="xspecialx", default_value_t=false)]
    pub(crate) mega_pass: bool,
}

#[derive(Subcommand, Debug)]
pub(crate) enum Commands {
    /// Let's wake up the crab!
    Start
}