use std::fmt;

pub enum LineCommandType {
    Arithmetic,
    Logical,
    Memory,
}

impl fmt::Debug for LineCommandType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Arithmetic => write!(f, "Arithmetic"),
            Self::Logical => write!(f, "Logical"),
            Self::Memory => write!(f, "Memory"),
        }
    }
}

#[derive(strum::Display)]
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
        }
    }
}

impl fmt::Debug for LineCommand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Push => write!(f, "Push"),
            Self::Pop => write!(f, "Pop"),
            Self::Add => write!(f, "Add"),
            Self::Sub => write!(f, "Sub"),
            Self::Neg => write!(f, "Neg"),
            Self::Equal => write!(f, "Equal"),
            Self::GreaterThan => write!(f, "GreaterThan"),
            Self::LessThan => write!(f, "LessThan"),
            Self::And => write!(f, "And"),
            Self::Or => write!(f, "Or"),
            Self::Not => write!(f, "Not"),
        }
    }
}