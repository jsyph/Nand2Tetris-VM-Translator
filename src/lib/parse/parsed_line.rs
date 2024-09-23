use super::{line_command::LineCommand, memory_segment::MemorySegment};
use std::fmt::{self};

pub struct ParsedLine {
    pub command: LineCommand,
    pub memory_segment: Option<MemorySegment>,
    pub memory_addr: Option<usize>,
    pub label: Option<String>,
}

impl fmt::Debug for ParsedLine {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ParsedLine")
            .field("command", &self.command)
            .field("memory_segment", &self.memory_segment)
            .field("memory_addr", &self.memory_addr)
            .finish()
    }
}

impl fmt::Display for ParsedLine {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            format!(
                "{} {:?} {:?}",
                self.command, self.memory_segment, self.memory_addr
            )
            .to_lowercase()
        )
    }
}
