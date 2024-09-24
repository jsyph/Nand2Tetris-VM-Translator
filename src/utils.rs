use std::{fs::File, io::{BufRead, BufReader}, path::Path};

pub fn file_into_lines(file: File) -> Vec<String> {
    let buf: BufReader<_> = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Unable to parse line"))
        .collect()
}

pub fn name_from_path(path: &Path) -> String {
    path
        .with_extension("")
        .file_name()
        .expect("Unable to parse file name")
        .to_str()
        .expect("Unable to parse file name")
        .to_owned()
}

