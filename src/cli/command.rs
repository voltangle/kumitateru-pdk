pub struct Command {
    /// A name of the function. Will be used when called from command line.
    name: String,
    /// Help text displayed in help menu.
    help: String,
    /// Args, which this command will accept.
    args: Vec<String>,
    /// A handler for this command.
    func: fn(Vec<&str>),
}