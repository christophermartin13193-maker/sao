//! `sao` (Switch Audio Output) is a lightweight CLI tool and library designed to effortlessly 
//! switch default audio sinks using PipeWire's `wpctl`.
//!
//! # Problem Statement
//!
//! Switching default audio devices using `wpctl` requires the target device's numeric ID.
//! However, PipeWire assigns these IDs dynamically at runtime. Every time an audio device is 
//! plugged in, removed, or PipeWire restarts, these IDs can change. 
//! 
//! Referencing sinks by ID creates friction when creating keyboard shortcuts, scripting, 
//! or automating audio configuration. `sao` eliminates this friction by allowing you to target 
//! audio sinks directly by their name or custom aliases.
//!
//! # Features
//!
//! - **Direct Name Matching**: Target a sink using its raw device name without any setup.
//! - **Custom Aliases**: Define short, easy-to-remember aliases for long sink names in a simple TOML configuration file.
//! - **Sink Cycling**: Safely cycle forward ([`--next`](crate)) or backward ([`--previous`](crate)) through a predefined list of favorite sinks.
//!
//! # Quick Start
//!
//! Below is a minimal example showing how to initialize and run the [`Automaton`] lifecycle in Rust code:
//!
//! ```no_run
//! use sao::{Automaton, Output};
//!
//! fn main() {
//!     let mut automaton = Automaton::default();
//!     loop {
//!         match automaton.run() {
//!             Ok(Output::Continue) => continue,
//!             Ok(Output::Exit(code)) => std::process::exit(code),
//!             Err(err) => std::process::exit(automaton.handle_error(err)),
//!         }
//!     }
//! }
//! ```
//!
//! # Configuration File
//!
//! To use aliases or sink cycling, create a configuration file named `config.toml` in one of the following locations:
//! - `$XDG_CONFIG_HOME/sao/config.toml` (e.g. `~/.config/sao/config.toml`)
//! - `$XDG_DATA_HOME/sao/config.toml` (e.g. `~/.local/share/sao/config.toml`)
//!
//! ### Example `config.toml`
//!
//! ```toml
//! [List]
//! Headphones = "alsa_output.pci-0000_00_1f.3.analog-stereo"
//! Speakers    = "alsa_output.usb-Audioengine_Audioengine_A2_-00.analog-stereo"
//! ```

mod core;
// API
pub use core::{Automaton, Output};
