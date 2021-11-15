use crate::cli::Commands;

pub mod cli;

type EventSubscription = (String, String);

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
    pub subscriptions: Vec<EventSubscription>,
    pub cli_commands: Commands,
}
