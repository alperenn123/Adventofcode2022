use std::cmp;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

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

fn find_common_badge(source_str: &[String]) -> Option<char> {
    let mut common = [std::u32::MAX; 58];

    for st in source_str {
        let mut freq = [0; 58];

        for ch in st.chars() {
            if ch.is_ascii() {
                let ascii = ch as u32;
                freq[(ascii - 'A' as u32) as usize] += 1;
            }
        }

        for (i, _el) in common.into_iter().enumerate() {
            common[i] = cmp::min(common[i], freq[i]);
        }
    }

    for (i, _el) in common.into_iter().enumerate() {
        if common[i] > 0 {
            let ascii_to_char = char::from_u32(i as u32 + 65);
            return ascii_to_char;
        }
    }

    None
}

fn calculate_point(ch: char) -> u32 {
    let ascii_value = ch as u32;
    if ch.is_lowercase() {
        ascii_value - 96
    } else {
        ascii_value - 64 + 26
    }
}

fn main() {
    let input = read_from_file();

    let vector_length = input.len();
    let mut index = 0;
    let mut total_point = 0;
    while index < vector_length {
        if let Some(badge) = find_common_badge(&input[index..index + 3]) {
            let point = calculate_point(badge);
            total_point += point;
        }

        index += 3;
    }

    print!("Total point is {:?}", total_point);
}
