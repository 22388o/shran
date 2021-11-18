#[derive(Debug)]
pub struct SubCommandName;

impl<'c> SubCommandName {
    pub const GENERATE: &'c str = "generate";
    pub const BUILD: &'c str = "build";
    pub const AUTH: &'c str = "auth";
}

pub struct ArgName;

impl<'c> ArgName {
    // Args for SubCommandName::GENERATE
    pub const BITCOIN: &'c str = "bitcoin";
    pub const LITECOIN: &'c str = "litecoin";
    // Args for SubCommandName::BUILD
    pub const CONFIG: &'c str = "config";
    // Args for SubCommandName::AUTH
    pub const TOKEN: &'c str = "token";
}


#[derive(Debug)]
pub struct ActiveCommand {
    command: String,
    arg: String,
}

impl ActiveCommand {
    pub fn new(command: &str, arg: &str) -> Self {
        Self {
            command: String::from(command),
            arg: String::from(arg)
        }
    }
}
