mod gen_arithmetic;
mod gen_logical;
mod gen_memory;

use gen_memory::gen_memory;
use gen_logical::gen_logical;
use gen_arithmetic::gen_arithmetic;

use crate::parse::{
    line_command::LineCommandType,
    ParsedLine,
};
use handlebars::Handlebars;

fn init_handlebars() -> Handlebars<'static> {
    let mut handlebars = Handlebars::new();

    handlebars
        .register_template_string(
            "memory/push/constant",
            include_str!("templates/memory/push/constant.template"),
        )
        .unwrap();
    handlebars
        .register_template_string(
            "memory/push/normal",
            include_str!("templates/memory/push/normal.template"),
        )
        .unwrap();
    handlebars
        .register_template_string(
            "memory/push/pointer0",
            include_str!("templates/memory/push/pointer0.template"),
        )
        .unwrap();
    handlebars
        .register_template_string(
            "memory/push/pointer1",
            include_str!("templates/memory/push/pointer1.template"),
        )
        .unwrap();
    handlebars
        .register_template_string(
            "memory/push/static",
            include_str!("templates/memory/push/static.template"),
        )
        .unwrap();
    handlebars
        .register_template_string(
            "memory/push/temp",
            include_str!("templates/memory/push/temp.template"),
        )
        .unwrap();

    handlebars
        .register_template_string(
            "memory/pop/normal",
            include_str!("templates/memory/pop/normal.template"),
        )
        .unwrap();
    handlebars
        .register_template_string(
            "memory/pop/pointer0",
            include_str!("templates/memory/pop/pointer0.template"),
        )
        .unwrap();
    handlebars
        .register_template_string(
            "memory/pop/pointer1",
            include_str!("templates/memory/pop/pointer1.template"),
        )
        .unwrap();
    handlebars
        .register_template_string(
            "memory/pop/static",
            include_str!("templates/memory/pop/static.template"),
        )
        .unwrap();
    handlebars
        .register_template_string(
            "memory/pop/temp",
            include_str!("templates/memory/pop/temp.template"),
        )
        .unwrap();

    handlebars
}

pub fn generate_code(parsed_lines: Vec<ParsedLine>, file_name: String) -> Vec<String> {
    let handlebars = init_handlebars();

    let mut res: Vec<String> = Vec::new();
    for line in parsed_lines {
        let generated_code = match line.command.command_type() {
            LineCommandType::Arithmetic => gen_arithmetic(&handlebars, line),
            LineCommandType::Logical => gen_logical(&handlebars, line),
            LineCommandType::Memory => gen_memory(&handlebars, line, file_name.clone()),
        };
        res.push(generated_code);
    }
    return res;
}
