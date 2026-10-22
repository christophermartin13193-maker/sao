use clap::{ArgAction, ArgGroup, Parser};
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[command(name = "sao")]
#[command(version = "1.0.0")]
#[command(author = "Chris Martin")]
#[command(about, long_about)]
#[command(group = ArgGroup::new("target").args(["next", "previous", "name"]).required(true))]
// Parsed command-line arguments.
pub struct Args {
    // Enables debug mode (prints verbose execution step logs).
    #[arg(short, long, action = ArgAction::SetTrue, conflicts_with = "quiet")]
    pub debug: bool,
    // Enables quiet mode (suppresses error messages on stderr).
    #[arg(short, long, action = ArgAction::SetTrue, conflicts_with = "debug")]
    pub quiet: bool,

    // Switch to the next sink defined in the configuration file.
    #[arg(short, long, action = ArgAction::SetTrue)]
    pub next: bool,
    // Switch to the previous sink defined in the configuration file.
    #[arg(short, long, action = ArgAction::SetTrue)]
    pub previous: bool,
    // The name or alias of the target audio sink to set as default.
    #[arg()]
    pub name: Option<String>,

    // Optional explicit path to the configuration file.
    #[arg(long)]
    pub path: Option<PathBuf>,
}

// Controls output verbosity during execution.
#[derive(Debug, Clone, Copy)]
pub enum Verbosity {
    // Suppress non-critical output.
    Quiet,
    // Print step-by-step execution details to stdout.
    Debug,
    // Standard execution output.
    Normal,
}

impl Verbosity {
    pub fn dbg(&self, s: &str) {
        if let Self::Debug = self { println!("{s}") }
    }
    pub fn err(&self, s: &str) {
        match self {
            Self::Quiet => (),
            _ => eprintln!("{s}")
        }
    }
}
