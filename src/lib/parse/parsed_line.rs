use super::{line_command::LineCommand, memory_segment::MemorySegment};
use std::fmt::{self};

pub struct ParsedLine {
    pub command: LineCommand,
    pub memory_segment: Option<MemorySegment>,
    pub label: Option<String>,
    pub func: Option<String>,
    pub i: Option<usize>,
}

impl fmt::Debug for ParsedLine {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ParsedLine")
            .field("command", &self.command)
            .field("memory_segment", &self.memory_segment)
            .field("label", &self.label)
            .field("func", &self.func)
            .field("i", &self.i)
            .finish()
    }
}
