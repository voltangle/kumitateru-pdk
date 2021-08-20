pub mod cli;

use anyhow::Result;
use crate::cli::*;

pub struct Kumitateru {
    /// Subscribes a function to
    /// an event in any place
    /// Kumitateru allows to extend.
    pub subscribe: fn(&str, fn()),

    /// Creates a custom command, which can
    /// be called like any other command in
    /// Kumitateru.
    pub create_command: fn(command::Command),
}

// static AVAILABLE_COMMANDS: [&str; 9] = [
//     "build::before",
//     "build::after",
//     "run::build::before",
//     "run::build::after",
//     "run::execution::after",
//     "package::before",
//     "package::after",
//     "clean::before",
//     "clean::after"
// ];

