pub mod cli;

use crate::cli::*;

pub struct PluginConfig {
    /// Plugin name
    name: String,
    /// Plugin version
    version: String,
    /// Plugin author
    author: String,
    /// Subscriptions to Kumitateru's events.
    /// Creates a new event, and
    /// It is basically a vector, which contains
    /// tuples of two strings. The first string is
    /// the event you call your function, and
    /// second string is the name of your function.
    /// Note that your function **must be public**.
    subscriptions: Vec<(String, String)>
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
