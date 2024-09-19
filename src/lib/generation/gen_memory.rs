use handlebars::Handlebars;

use crate::parse::{line_command::LineCommand, memory_segment::MemorySegment, ParsedLine};

#[derive(serde::Serialize)]
struct MemoryTemplateData {
    i: usize,
    mem_segment: String,
    file_name: String,
}

pub fn gen_memory(handlebars: &Handlebars, line: ParsedLine, file_name: String) -> String {
    let line_command = line.command;
    let memory_segment = line.memory_segment.unwrap();
    let memory_addr = line.memory_addr.unwrap();

    let data = MemoryTemplateData {
        i: memory_addr,
        mem_segment: memory_segment.to_string(),
        file_name: file_name,
    };

    let template_name = match (line_command, memory_segment) {
        (
            LineCommand::Push,
            MemorySegment::Local
            | MemorySegment::Argument
            | MemorySegment::This
            | MemorySegment::That,
        ) => "memory/push/normal",

        (
            LineCommand::Pop,
            MemorySegment::Local
            | MemorySegment::Argument
            | MemorySegment::This
            | MemorySegment::That,
        ) => "memory/pop/normal",

        (LineCommand::Push, MemorySegment::Constant) => "memory/push/constant",

        (LineCommand::Push, MemorySegment::Static) => "memory/push/static",
        (LineCommand::Pop, MemorySegment::Static) => "memory/pop/static",

        (LineCommand::Push, MemorySegment::Pointer) if data.i == 0 => "memory/push/pointer0",
        (LineCommand::Pop, MemorySegment::Pointer) if data.i == 0 => "memory/pop/pointer0",
        (LineCommand::Push, MemorySegment::Pointer) if data.i == 1 => "memory/push/pointer1",
        (LineCommand::Pop, MemorySegment::Pointer) if data.i == 1 => "memory/pop/pointer1",

        (LineCommand::Push, MemorySegment::Temp) => "memory/push/temp",
        (LineCommand::Pop, MemorySegment::Temp) => "memory/pop/temp",
        _ => panic!("Well, this should never happen"),
    };

    handlebars
        .render(template_name, &data)
        .expect("Unable to render Hack symbolic code template.")
}
