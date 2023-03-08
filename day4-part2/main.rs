use std::fs::File;
use std::io::{BufRead, BufReader};

fn read_from_file(file_path: String) -> Vec<String> {
    let file = File::open(file_path).expect("file not found!");
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

fn format_file_content(file_content: &[String]) -> Vec<((i32, i32), (i32, i32))> {
    let mut formatted_content: Vec<((i32, i32), (i32, i32))> =
        Vec::with_capacity(file_content.len());
    for content in file_content.iter() {
        let splitted_content = content.split(',').collect::<Vec<&str>>();
        let first = splitted_content[0].split('-').collect::<Vec<&str>>();
        let second = splitted_content[1].split('-').collect::<Vec<&str>>();
        let lower_boundries = (
            first[0].parse::<i32>().expect("Error parsing first"),
            second[0].parse::<i32>().expect("Error parsing second"),
        );
        let upper_boundries = (
            first[1].parse::<i32>().expect("Error parsing first"),
            second[1].parse::<i32>().expect("Error parsing second"),
        );

        formatted_content.push((lower_boundries, upper_boundries))
    }

    formatted_content
}

type InputType = [((i32, i32), (i32, i32))];

fn calcuate_answer(input: &InputType) -> u32 {
    let mut answer = 0;
    for input_value in input.iter() {
        let lower_boundries = input_value.0;
        let upper_boundries = input_value.1;

        if (lower_boundries.0 <= lower_boundries.1) && (upper_boundries.0 >= lower_boundries.1)
            || (lower_boundries.0 >= lower_boundries.1 && lower_boundries.0 <= upper_boundries.1)
        {
            answer += 1
        }
    }

    answer
}

fn main() {
    let file_content = read_from_file("input.txt".to_string());
    let formatted_file_content = format_file_content(&file_content);
    let answer = calcuate_answer(&formatted_file_content);
    println!("answer {:?}", answer);
}
