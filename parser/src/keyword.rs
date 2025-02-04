use regex::Regex;
use std::{collections::HashMap, sync::LazyLock};

static FUNCTION_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new("([f|u|n|c|t|i|o|n])").unwrap());

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Keyword {
    Const,
    Var,

    New,

    Function,

    Async,

    When,

    If,
    Else,

    Import,
    Export,
    To, // export to ..

    Delete,
    Reverse,

    Current,
    Previous,

    Class,
    ClassName
}

impl Keyword {
    pub fn parse(value: &str, aliases: &HashMap<String, Self>) -> Option<Self> {
        match value {
            "const" => Some(Self::Const),
            "var" => Some(Self::Var),
            
            "new" => Some(Self::New),
            
            "async" => Some(Self::Async),
            
            "when" => Some(Self::When),

            "if" => Some(Self::If),
            "else" => Some(Self::Else),

            "import" => Some(Self::Import),
            "export" => Some(Self::Export),
            "to" => Some(Self::To),
            
            "delete" => Some(Self::Delete),
            "reverse" => Some(Self::Reverse),

            "current" => Some(Self::Current),
            "previous" => Some(Self::Previous),
            
            "class" => Some(Self::Class),
            "className" => Some(Self::Class),

            _ if FUNCTION_REGEX.is_match(value) => Some(Self::Function),
            _ if aliases.contains_key(value) => Some(*aliases.get(value).unwrap()),
            _ => None,
        }
    }
}
