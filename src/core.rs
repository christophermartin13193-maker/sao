mod args;
mod config;
mod control;
mod getter;
mod runner;
mod workflow;

// Internal API
use args::{Args, Verbosity};
use config::{Config, get, get_default};
use control::SaoError;
use control::code::ExitCode;
use getter::{check_cmd, id, id_from_config, sinks};
use runner::{init, next, previous, target, wpctl};
use workflow::{ID, Step};

// Both API
pub use control::Output;

// External API
pub use workflow::Automaton;
