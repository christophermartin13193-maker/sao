use crate::core::control::code;

use super::{Args, Config, ExitCode, Output, SaoError, Verbosity};
use super::{init, next, previous, target, wpctl};
use clap::Parser;

// The internal type alias representing a PipeWire numerical sink ID.
pub type ID = u32;

// Represents the sequential execution steps in the application state machine.
pub enum Step {
    // Initial step: validates system prerequisites and reads CLI arguments and configuration.
    Init {
        v: Verbosity,
        args: Args,
    },
    // Direct targeting step: resolves a specific sink name or alias to a PipeWire sink ID.
    Target {
        v: Verbosity,
        name: String,
        sinks: Vec<String>,
        config: Option<Config>,
    },
    // Sink cycling step: selects the next valid sink alias from the configured list.
    Next {
        v: Verbosity,
        sinks: Vec<String>,
        config: Config,
    },
    // Sink cycling step: selects the previous valid sink alias from the configured list.
    Previous {
        v: Verbosity,
        sinks: Vec<String>,
        config: Config,
    },
    // Execution step: issues the `wpctl set-default` command with the resolved sink ID.
    Wpctl {
        v: Verbosity,
        id: ID,
    },
    // Final step: marks the completion of the state machine execution.
    Exit,
}

/// The main state machine driving the application lifecycle.
///
/// `Automaton` parses CLI input, reads optional configuration files, resolves target audio 
/// sink IDs, and executes `wpctl` calls step-by-step.
pub struct Automaton {
    step: Step,
    v: Verbosity,
}

impl Automaton {
    /// Creates a new `Automaton` instance.
    ///
    /// Parses CLI arguments using [`clap`], determines verbosity levels, 
    /// and initializes the state machine to `Step::Init`.
    pub fn new() -> Self {
        let args = Args::parse();
        let v = match (args.quiet, args.debug) {
            (true, _) => Verbosity::Quiet,
            (_, true) => Verbosity::Debug,
            _ => Verbosity::Normal,
        };
        let step = Step::Init { v, args };
        Self { step, v }
    }

    /// Advances the state machine by executing the current `Step`.
    ///
    /// # Returns
    ///
    /// - `Ok(Output::Continue)` if the step finished successfully and more steps remain.
    /// - `Ok(Output::Exit(code))` if the lifecycle has completed.
    /// - `Err(SaoError)` if an unrecoverable error occurred at the current step.
    pub fn run(&mut self) -> Result<Output, SaoError> {
        let current = std::mem::replace(&mut self.step, Step::Exit);
        self.step = match current {
            Step::Init { v, args } => init(v, args)?,
            Step::Target {
                v,
                name,
                sinks,
                config,
            } => target(v, name, sinks, config)?,
            Step::Next { v, sinks, config } => next(v, sinks, config)?,
            Step::Previous { v, sinks, config } => previous(v, sinks, config)?,
            Step::Wpctl { v, id } => wpctl(v, id)?,
            Step::Exit => return Ok(Output::Exit(code::STANDARD)),
        };

        Ok(Output::Continue)
    }

    /// Translates an `SaoError` into the appropriate process exit code and logs 
    /// human-readable error details according to the configured verbosity.
    pub fn handle_error(&self, err: SaoError) -> ExitCode {
        self.v.err(&err.to_string());

        match err {
            SaoError::Io(_) => code::IO,
            SaoError::Toml(_) | SaoError::NoConfigFile | SaoError::NoValidSinkInConfig => code::CONFIG,
            SaoError::NoCmd | SaoError::ParseInt(_) | SaoError::Utf8(_) => code::WPCTL,
            SaoError::NoID(_) | SaoError::NoSinks => code::TARGET,
        }
    }
}

impl Default for Automaton {
    fn default() -> Self {
        Self::new()
    }
}
