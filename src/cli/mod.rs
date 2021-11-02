#[derive(Debug)]
pub struct Command {
    /// A name of the function. Will be used when called from command line.
    pub name: String,
    /// Help text displayed in help menu.
    pub help: String,
    /// Args, which this command will accept.
    pub args: Vec<String>,
    /// A handler for this command.
    pub func: fn(Vec<String>),
}