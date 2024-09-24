use handlebars::Handlebars;
use random_string::generate;

use crate::{
    error::TranslatorResult,
    parse::{LineCommand, ParsedLine},
    TranslatorError,
};

#[derive(serde::Serialize)]
struct FunctionCallTemplateData<'a> {
    func: &'a str,
    n_args: usize,
    arg_addr_offset: usize,
    random_str: String,
    optimize: bool,
}

const CHARSET: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ123456789";
const RAND_STRING_SIZE: usize = 10;
fn gen_function_call(
    handlebars: &Handlebars,
    func: &str,
    n_args: usize,
    optimize: bool,
) -> TranslatorResult<String> {
    let random_str = generate(RAND_STRING_SIZE, CHARSET);
    
    let data = FunctionCallTemplateData {
        func: &func,
        n_args: n_args,
        arg_addr_offset: n_args + 5, // ADDR = SP - (5 + n_args)
        random_str,
        optimize,
    };

    Ok(handlebars.render("function/call.hbs", &data)?)
}

#[derive(serde::Serialize)]
struct FunctionDefinitionTemplateData<'a> {
    func: &'a str,
    n_vars: usize,
    n_vars_vec: Vec<u8>,
    optimize: bool,
}

fn gen_function_definition(
    handlebars: &Handlebars,
    func: &str,
    n_vars: usize,
    optimize: bool,
) -> TranslatorResult<String> {
    let data = FunctionDefinitionTemplateData {
        func,
        n_vars,
        // handlebars doesnt support looping for n times, this is a workaround
        n_vars_vec: vec![0; n_vars],
        optimize,
    };

    Ok(handlebars.render("function/definition.hbs", &data)?)
}

#[derive(serde::Serialize)]
struct FunctionReturnTemplateData {
    optimize: bool,
}

fn gen_function_return(handlebars: &Handlebars, optimize: bool) -> TranslatorResult<String> {
    let data = FunctionReturnTemplateData { optimize };

    Ok(handlebars.render("function/return.hbs", &data)?)
}

pub fn gen_function(
    handlebars: &Handlebars,
    line: ParsedLine,
    optimize: bool,
) -> TranslatorResult<String> {
    let line_command = line.command;

    Ok(match line_command {
        LineCommand::Call => {
            let n = line.i.unwrap();
            let func = line.func.unwrap();
            gen_function_call(handlebars, &func, n, optimize)?
        }
        LineCommand::Function => {
            let n = line.i.unwrap();
            let func = line.func.unwrap();
            gen_function_definition(handlebars, &func, n, optimize)?
        }
        LineCommand::Return => gen_function_return(handlebars, optimize)?,
        _ => return Err(TranslatorError::UnDefinedBehavior),
    })
}
