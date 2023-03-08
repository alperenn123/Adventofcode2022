use std::fs::File;
use std::io::{BufRead, BufReader};

fn read_from_file() -> Vec<String> {
    let file = File::open("input.txt").expect("file not found!");
    let buf_reader = BufReader::new(file);
    let mut file_content: Vec<String> = Vec::new();

    for line in buf_reader.lines() {
        match line {
            Ok(mut line) => {
                line.retain(|ch| !ch.is_whitespace());
                file_content.push(line);
            }
            Err(err) => println!("Error while reading the file, Error::{}", err),
        }
    }
    file_content
}

fn find_common_char(source_str: String) -> Option<char> {
    let first_half = source_str.len() / 2;

    let first_string = &source_str[0..first_half];
    let second_string = &source_str[first_half..source_str.len()];

    for ch in first_string.chars() {
        if let Some(index) = second_string.find(ch) {
            return Some(
                second_string
                    .chars()
                    .nth(index)
                    .expect("Index is not valid"),
            );
        }
    }

    None
}

fn calculate_point(ch: char) -> u32 {
    let ascii_value = ch as u8;
    println!("Ascii value {:?}", ascii_value);
    if ch.is_lowercase() {
        (ascii_value - 96).into()
    } else {
        (ascii_value - 64 + 26).into()
    }
}

fn main() {
    let file_content = read_from_file();
    let mut total: u32 = 0;
    for st in file_content {
        if let Some(ch) = find_common_char(st) {
            let point = calculate_point(ch);
            total += point;
        }
    }

    println!("Result is {:?}", total);
}
