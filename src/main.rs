use clap::{ArgAction, Parser};
use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};
use translator_lib::{generate_code, parse_lines};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    target: String,

    #[arg(short, long)]
    output: Option<String>,

    #[arg(long, short, action=ArgAction::SetFalse)]
    release: bool,
}

fn main() {
    let args = Args::parse();

    // read file into memory
    let target_path = Path::new(&args.target);
    if !target_path.exists() {
        println!("Target file path `{}` does not exist", args.target);
        return;
    }
    let target = File::open(target_path).expect("Unable to access target file");

    // parse file into lines
    let buf = BufReader::new(target);
    let unprocessed_lines: Vec<String> = buf
        .lines()
        .map(|l| l.expect("Unable to parse line"))
        .collect();

    let parsed_lines_result = parse_lines(unprocessed_lines);
    let parsed_lines = match parsed_lines_result {
        Ok(res) => res,
        Err(e) => {
            println!("{}", e);
            return;
        }
    };

    let target_file_name = target_path
        .file_name()
        .expect("Unable to parse file name")
        .to_str()
        .expect("Unable to parse file name")
        .to_owned();
    let output = generate_code(parsed_lines, target_file_name);

    for x in output {
        println!("{}", x)
    }
}
