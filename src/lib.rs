type CliCommandArgs = Vec<CliCommandArg>;
type CliCommands = Vec<CliCommand>;
type EventSubscription = (String, String);
type EventSubscriptions = Vec<EventSubscription>;

#[derive(Clone)]
pub struct PluginConfig {
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
    pub subscriptions: EventSubscriptions,
    /// Commands, that can be called as a kumitateru
    /// subcommand.
    pub cli_commands: CliCommands
}

pub struct CliCommand {
    pub name: String,
    pub args: CliCommandArgs
}

pub struct CliCommandArg {
    pub name: String,
    pub long_name: String,
    pub value_name: String,
    pub takes_value: bool
}
