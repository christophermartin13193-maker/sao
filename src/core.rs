mod args;
mod config;
mod control;
mod runner;
mod workflow;
mod getter;

// Internal API
use args::{Args, Verbosity};
use config::{Config, get, get_default};
use control::{ERROR_CODE, ExitCode, STANDARD_CODE, SaoError};
use workflow::{Step, ID};
use runner::{init, target, next, previous, wpctl};
use getter::{check_cmd, sinks, id, id_from_config};

// Both API
pub use control::Output;

// External API
pub use workflow::Automaton;
