use clap::{ArgAction, Parser};
use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};
use translator_lib::{generate_code, parse_lines, TranslatorError};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    target: String,

    #[arg(short, long)]
    output: Option<String>,

    #[arg(long, action=ArgAction::SetTrue)]
    debug: bool,
}

fn main() {
    let args = Args::parse();

    if args.debug {
        simple_logger::init_with_level(log::Level::Debug).unwrap();
    } else {
        simple_logger::init_with_level(log::Level::Error).unwrap();
    }

    // read file into memory
    let target_path = Path::new(&args.target);
    if !target_path.exists() {
        log::error!(
            "{}",
            TranslatorError::FileIOError {
                message: format!("Target file path `{}` does not exist", args.target),
            }
        );
        return;
    }
    let target_result = File::open(target_path);
    if target_result.is_err() {
        log::error!("{}", TranslatorError::from(target_result.unwrap_err()));
        return;
    }
    let target = target_result.unwrap();

    // parse file into lines
    let buf = BufReader::new(target);
    let unprocessed_lines: Vec<String> = buf
        .lines()
        .map(|l| l.expect("Unable to parse line"))
        .collect();

    let parsed_lines_result = parse_lines(unprocessed_lines);
    if parsed_lines_result.is_err() {
        log::error!("{}", parsed_lines_result.unwrap_err());
        return;
    }
    let parsed_lines = parsed_lines_result.unwrap();

    let target_file_name = target_path
        .with_extension("")
        .file_name()
        .expect("Unable to parse file name")
        .to_str()
        .expect("Unable to parse file name")
        .to_owned();
    let generation_result = generate_code(parsed_lines, &target_file_name);
    if generation_result.is_err() {
        log::error!("{}", generation_result.unwrap_err());
        return;
    }
    let generated_code = generation_result.unwrap();

    let output_file_name = format!("{}.asm", target_file_name);
    let outfile_create = File::create(&output_file_name);
    if outfile_create.is_err() {
        log::error!("{}", TranslatorError::from(outfile_create.unwrap_err()));
        return;
    }

    let mut outfile = outfile_create.unwrap();
    for translated_line in generated_code {
        outfile
            .write_all(format!("{}\n", translated_line).as_bytes())
            .expect("Unable to write to outfile");
        if args.debug {
            outfile
                .write_all("\n".as_bytes())
                .expect("Unable to write to outfile");
        }
    }

    println!("Done");
}
