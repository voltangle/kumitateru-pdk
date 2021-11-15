pub type CommandArgsDecl = Vec<CommandArgDecl>;
pub type Commands = Vec<Command>;

#[derive(Debug, Clone)]
pub struct Command {
    pub name: String,
    pub args: CommandArgsDecl,
    pub func: fn(Vec<CommandArgDef>),
}

impl Command {
    fn new(name: &str, args: CommandArgsDecl, func: fn(Vec<CommandArgDef>)) -> Self {
        Self {
            name: name.to_string(),
            args,
            func,
        }
    }
}

/**
 * This struct will be passed to the function specified
 * in `func` member variable in [Command](Command) struct.
 * Struct name is "decoded" as Command arg definition.
 */
#[derive(Debug, Clone)]
pub struct CommandArgDef {
    pub name: String,
    pub value: String,
}

/**
 * This struct is used to declare command arguments. See
 * [Command](Command) struct for more info
 */
#[derive(Debug, Clone)]
pub struct CommandArgDecl {
    pub name: String,
    pub long_name: Option<String>,
    pub value_name: String,
    pub takes_value: Option<bool>,
}

impl CommandArgDecl {
    fn new(
        name: &str,
        long_name: Option<String>,
        value_name: &str,
        takes_value: Option<bool>,
    ) -> Self {
        Self {
            name: name.to_string(),
            long_name,
            value_name: value_name.to_string(),
            takes_value,
        }
    }
}
