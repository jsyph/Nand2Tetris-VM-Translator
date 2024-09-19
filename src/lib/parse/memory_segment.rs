use core::fmt;

pub enum MemorySegment {
    Local,
    Argument,
    This,
    That,
    Constant,
    Static,
    Pointer,
    Temp,
}

impl MemorySegment {
    pub fn parse(s: &str) -> Option<Self> {
        match s {
            "local" => Some(Self::Local),
            "argument" => Some(Self::Argument),
            "this" => Some(Self::This),
            "that" => Some(Self::That),
            "constant" => Some(Self::Constant),
            "static" => Some(Self::Static),
            "pointer" => Some(Self::Pointer),
            "temp" => Some(Self::Temp),
            _ => None,
        }
    }
}

impl fmt::Debug for MemorySegment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Local => write!(f, "Local"),
            Self::Argument => write!(f, "Argument"),
            Self::This => write!(f, "This"),
            Self::That => write!(f, "That"),
            Self::Constant => write!(f, "Constant"),
            Self::Static => write!(f, "Static"),
            Self::Pointer => write!(f, "Pointer"),
            Self::Temp => write!(f, "Temp"),
        }
    }
}

impl ToString for MemorySegment {
    fn to_string(&self) -> String {
        match self {
            MemorySegment::Local => "LCL".to_owned(),
            MemorySegment::Argument => "ARG".to_owned(),
            MemorySegment::This => "THIS".to_owned(),
            MemorySegment::That => "THAT".to_owned(),
            _ => "".to_owned(),
        }
    }
}
