pub mod cli;

type EventSubscription = (String, String, String);

pub struct EventSubscriptions {
    /// Only for safety and ease of use purpose
    pub(in kumitateru) currently_writing_plugin: String,
    pub(in kumitateru) list: Vec<EventSubscription>,
}

impl EventSubscriptions {
    /// Pushes a new subscriber.
    fn push(&mut self, name: &str, func: &str) {
        self.list.push((
            self.currently_writing_plugin.clone(),
            name.to_string(),
            func.to_string(),
        ));
    }
}

/**
 * This struct will be passed to an activate() function
 * of a plugin as a mutable reference, for it to be filled
 * by it.
 */
#[derive(Clone, Debug)]
pub struct PluginEnvironment {
    /// Subscriptions to kumitateru's events.
    /// To create new subscriptions, use push()
    /// function.
    pub subscriptions: EventSubscriptions,
    pub cli_commands: CliCommands,
}

/**
 * A struct that is returned by a plugin on activation, to
 * define what that plugin can do. Deprecated.
 */
#[derive(Clone)]
#[deprecated(since = "0.5.4", note = "Replaced by the plugin bundle concept.")]
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
    pub cli_commands: CliCommands,
}
