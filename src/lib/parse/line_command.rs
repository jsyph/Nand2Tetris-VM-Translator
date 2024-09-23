#[derive(strum::Display, Debug)]
pub enum LineCommandType {
    Arithmetic,
    Logical,
    Memory,
    Branching,
    // Function,
}

#[derive(strum::Display, Debug)]
pub enum LineCommand {
    Push,
    Pop,
    Add,
    Sub,
    Neg,
    Equal,
    GreaterThan,
    LessThan,
    And,
    Or,
    Not,
    GoTo,
    IfGoTo,
    Label,
    // Call,
    // Function,
    // Return,
}

impl LineCommand {
    /// Parses string to relevant LineCommand type
    /// Returns None if there is no relevant type
    pub fn parse(s: &str) -> Option<Self> {
        match s {
            "push" => Some(Self::Push),
            "pop" => Some(Self::Pop),
            "add" => Some(Self::Add),
            "sub" => Some(Self::Sub),
            "neg" => Some(Self::Neg),
            "eq" => Some(Self::Equal),
            "gt" => Some(Self::GreaterThan),
            "lt" => Some(Self::LessThan),
            "and" => Some(Self::And),
            "or" => Some(Self::Or),
            "not" => Some(Self::Not),
            "goto" => Some(Self::GoTo),
            "if-goto" => Some(Self::IfGoTo),
            "label" => Some(Self::Label),
            _ => None,
        }
    }

    pub fn command_type(&self) -> LineCommandType {
        match self {
            Self::Push | Self::Pop => LineCommandType::Memory,
            Self::Add | Self::Sub | Self::Neg => LineCommandType::Arithmetic,
            Self::Equal | Self::GreaterThan | Self::LessThan | Self::And | Self::Or | Self::Not => {
                LineCommandType::Logical
            }
            Self::GoTo | Self::IfGoTo | Self::Label => LineCommandType::Branching,
        }
    }
}
