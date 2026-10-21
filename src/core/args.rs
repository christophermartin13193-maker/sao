use clap::{ArgAction, ArgGroup, Parser};
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[command(name = "sao")]
#[command(version = "1.0.0")]
#[command(author = "Chris Martin")]
#[command(about, long_about)]
#[command(group = ArgGroup::new("target").args(["next", "previous", "name"]).required(true))]
pub struct Args {
    #[arg(short, long, action = ArgAction::SetTrue, conflicts_with = "quiet")]
    pub debug: bool,
    #[arg(short, long, action = ArgAction::SetTrue, conflicts_with = "debug")]
    pub quiet: bool,

    #[arg(short, long, action = ArgAction::SetTrue)]
    pub next: bool,
    #[arg(short, long, action = ArgAction::SetTrue)]
    pub previous: bool,
    #[arg()]
    pub name: Option<String>,

    #[arg(long)]
    pub path: Option<PathBuf>,
}

#[derive(Debug, Clone, Copy)]
pub enum Verbosity {
    Quiet,
    Debug,
    Normal,
}

impl Verbosity {
    pub fn dbg(&self, s: &str) {
        match self {
            Self::Debug => println!("{s}"),
            _ => (),
        }
    }
    pub fn err(&self, s: &str) {
        match self {
            Self::Quiet => (),
            _ => eprintln!("{s}"),
        }
    }
}
