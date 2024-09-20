use handlebars::Handlebars;

use crate::{
    error::TranslatorResult,
    parse::{line_command::LineCommand, memory_segment::MemorySegment, ParsedLine}, TranslatorError,
};

#[derive(serde::Serialize)]
struct MemoryTemplateData<'a> {
    i: usize,
    mem_segment: String,
    file_name: &'a str,
}

pub fn gen_memory(
    handlebars: &Handlebars,
    line: ParsedLine,
    file_name: &str,
) -> TranslatorResult<String> {
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
        ) => "memory/push/normal.hbs",

        (
            LineCommand::Pop,
            MemorySegment::Local
            | MemorySegment::Argument
            | MemorySegment::This
            | MemorySegment::That,
        ) => "memory/pop/normal.hbs",

        (LineCommand::Push, MemorySegment::Constant) => "memory/push/constant.hbs",

        (LineCommand::Push, MemorySegment::Static) => "memory/push/static.hbs",
        (LineCommand::Pop, MemorySegment::Static) => "memory/pop/static.hbs",

        (LineCommand::Push, MemorySegment::Pointer) if data.i == 0 => "memory/push/pointer0.hbs",
        (LineCommand::Pop, MemorySegment::Pointer) if data.i == 0 => "memory/pop/pointer0.hbs",
        (LineCommand::Push, MemorySegment::Pointer) if data.i == 1 => "memory/push/pointer1.hbs",
        (LineCommand::Pop, MemorySegment::Pointer) if data.i == 1 => "memory/pop/pointer1.hbs",

        (LineCommand::Push, MemorySegment::Temp) => "memory/push/temp.hbs",
        (LineCommand::Pop, MemorySegment::Temp) => "memory/pop/temp.hbs",
        _ => return Err(TranslatorError::UnDefinedBehavior),
    };

    Ok(handlebars.render(template_name, &data)?)
}
