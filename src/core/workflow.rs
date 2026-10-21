use super::{Args, ERROR_CODE, ExitCode, Output, SaoError, Verbosity, STANDARD_CODE, Config};
use super::{init, target, next, previous, wpctl};
use clap::Parser;

pub type ID = u32;

pub enum Step {
    Init { v: Verbosity, args: Args },
    Target { v: Verbosity, name: String, sinks: Vec<String>, config: Option<Config> },
    Next { v: Verbosity, sinks: Vec<String>, config: Config },
    Previous { v: Verbosity, sinks: Vec<String>, config: Config },
    Wpctl { v: Verbosity, id: ID },
    Exit,
}

pub struct Automaton {
    step: Step,
    v: Verbosity,
}

impl Automaton {
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

    pub fn run(&mut self) -> Result<Output, SaoError> {
        let current = std::mem::replace(&mut self.step, Step::Exit);
        self.step = match current {
            Step::Init { v, args } => init(v, args)?,
            Step::Target { v, name, sinks, config } => target(v, name, sinks, config)?,
            Step::Next { v, sinks, config } => next(v, sinks, config)?,
            Step::Previous { v, sinks, config } => previous(v, sinks, config)?,
            Step::Wpctl { v, id } => wpctl(v, id)?,
            Step::Exit => return Ok(Output::Exit(STANDARD_CODE)),
        };

        Ok(Output::Continue)
    }

    pub fn handle_error(&self, err: SaoError) -> ExitCode {
        self.v.err(&err.to_string());
        ERROR_CODE
    }
}

impl Default for Automaton {
    fn default() -> Self {
        Self::new()
    }
}
