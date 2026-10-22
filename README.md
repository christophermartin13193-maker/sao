# SAO (Switch Audio Output)

`sao` is a lightweight CLI tool and library written in Rust designed to effortlessly switch PipeWire's default audio sinks using `wpctl`.

## The Problem

Switching default audio devices using PipeWire's `wpctl` requires referencing the target device by its dynamic numerical ID (e.g., `wpctl set-default 42`). 

Because PipeWire reassigns these IDs whenever system audio sinks change, a system restarts, or devices are re-plugged, referencing sinks by ID in custom scripts or desktop shortcuts quickly breaks. `sao` solves this by resolving targets by device names or custom user aliases at runtime.

## Features

- **Direct Name Matching**: Instantly switch to any sink using its raw device name.
- **Custom Aliases**: Map long, ugly sink names to clean aliases via a TOML configuration file.
- **Sink Cycling**: Toggle sequentially through your favorite devices with `--next` and `--previous`.
- **Zero Configuration Required**: Works out of the box for direct name matching without requiring a config file.

---

## Installation

### Prerequisites

- [PipeWire](https://pipewire.org/) with `wpctl` installed and available in your `PATH`.
- [Rust](https://www.rust-lang.org/) toolchain (if building from source).

### Building from Source

```bash
git clone https://github.com/your-username/sao.git
cd sao
cargo build --release
```

The compiled binary will be available at `target/release/sao`.

---

## Usage

### Switch by Raw Sink Name

No configuration needed. Simply provide part of the target device's name:

```bash
sao "Headphones"
```

### Switch by Alias (`--next` / `--previous`)

Cycle through your configured audio devices:

```bash
# Switch to the next sink in your configured list
sao --next

# Switch to the previous sink in your configured list
sao --previous
```

### Options

```text
Usage: sao [OPTIONS] [NAME]

Arguments:
  [NAME]  Name or alias of the target audio sink

Options:
  -d, --debug     Enable verbose logging
  -q, --quiet     Suppress error output
  -n, --next      Switch to the next sink in the configuration list
  -p, --previous  Switch to the previous sink in the configuration list
      --path      Explicit path to a custom config.toml file
  -h, --help      Print help information
  -V, --version   Print version information
```

---

## Configuration

To use aliases or sink cycling (`--next` / `--previous`), create a `config.toml` file in one of the following locations:

- `$XDG_CONFIG_HOME/sao/config.toml` (typically `~/.config/sao/config.toml`)
- `$XDG_DATA_HOME/sao/config.toml` (typically `~/.local/share/sao/config.toml`)

### Example `config.toml`

```toml
[List]
Headphones = "alsa_output.pci-0000_00_1f.3.analog-stereo"
Speakers   = "alsa_output.usb-Audioengine_Audioengine_A2_-00.analog-stereo"
HDMI       = "alsa_output.pci-0000_01_00.1.hdmi-stereo"
```

When running `sao Headphones`, `sao` checks if `Headphones` is defined in your config and matches it to its corresponding PipeWire sink string.

---

## Library Usage

`sao` can also be integrated into your own Rust applications:

```rust
use sao::{Automaton, Output};

fn main() {
    let mut automaton = Automaton::default();
    loop {
        match automaton.run() {
            Ok(Output::Continue) => continue,
            Ok(Output::Exit(code)) => std::process::exit(code),
            Err(err) => std::process::exit(automaton.handle_error(err)),
        }
    }
}
```

---

## License

Distributed under the MIT License. See `LICENSE` for more information.