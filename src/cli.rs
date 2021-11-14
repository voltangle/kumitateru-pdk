type CommandArgs = Vec<CommandArgDef>;
type Commands = Vec<Command>;

#[derive(Debug)]
#[no_mangle]
pub struct Command {
    pub name: String,
    pub args: CommandArgs,
    pub func: fn(Vec<CommandArgDef>),
}

#[no_mangle]
impl Command {
    fn new(name: &str, args: CommandArgs, func: fn(Vec<CommandArgDef>)) -> Self {
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
#[no_mangle]
pub struct CommandArgDef {
    pub name: String,
    pub value: String,
}

/**
 * This struct is used to declare command arguments. See
 * [Command](Command) struct for more info
 */
#[derive(Debug)]
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