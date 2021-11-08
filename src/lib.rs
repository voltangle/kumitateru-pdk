use clap::App;

#[derive(Debug, Clone)]
pub struct PluginConfig<'a, 'b> {
    /// Plugin name
    pub name: String,
    /// Plugin version
    pub version: String,
    /// Plugin author
    pub author: String,
    /// Subscriptions to kumitateru's events.
    /// Creates a new event, and
    /// It is basically a vector, which contains
    /// tuples of two strings. The first string is
    /// the event you call your function, and
    /// second string is the name of your function.
    /// Note that your function **must be public**.
    pub subscriptions: Vec<(String, String)>,
    /// Commands, that can be called as a kumitateru
    /// subcommand.
    pub cli_commands: Vec<App<'a, 'b>>
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
