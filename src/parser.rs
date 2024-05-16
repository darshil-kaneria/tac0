
use std::fs;

fn parse(filename: String) {
    let file_str = fs::read_to_string(filename).expect("Unable to parse file");
}