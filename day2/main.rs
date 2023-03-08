use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

enum MatchResult {
    Lose = -1,
    Draw = 0,
    Win = 1,
}

struct ResultMap<'a> {
    win: &'a HashMap<char, char>,
    draw: &'a HashMap<char, char>,
    lose: &'a HashMap<char, char>,
}

impl<'a> ResultMap<'a> {
    fn return_what_to_play(&self, opponent: char, desired_score: char) -> Option<&char> {
        match desired_score {
            'Y' => self.draw.get(&opponent),
            'X' => self.lose.get(&opponent),
            'Z' => self.win.get(&opponent),
            _ => None,
        }
    }

    fn calculate_match_result2(&self, result_match: char) -> Option<MatchResult> {
        match result_match {
            'Y' => Some(MatchResult::Draw),
            'X' => Some(MatchResult::Lose),
            'Z' => Some(MatchResult::Win),
            _ => None,
        }
    }

    fn calculate_total_points2(&self, file_content: &[String]) -> u32 {
        let point_map: HashMap<char, u32> = HashMap::from([('X', 1), ('Y', 2), ('Z', 3)]);
        let mut total_point = 0;
        for st in file_content.iter() {
            let opponent = st.chars().next().expect("Error while reading the char");
            let me = st.chars().nth(1).expect("Error while reading the char");
            let what_to_play = self.return_what_to_play(opponent, me).unwrap();

            match self.calculate_match_result2(me).unwrap() {
                MatchResult::Win => {
                    total_point += 6;
                }
                MatchResult::Draw => {
                    total_point += 3;
                }
                _ => {}
            }

            total_point += point_map
                .get(what_to_play)
                .expect("Error while getting the point");
        }
        total_point
    }
}

fn calculate_match_result(me: char, opponent: char) -> MatchResult {
    match opponent {
        'A' => match me {
            'X' => MatchResult::Draw,
            'Y' => MatchResult::Win,
            'Z' => MatchResult::Lose,
            _ => MatchResult::Lose,
        },

        'B' => match me {
            'X' => MatchResult::Lose,
            'Y' => MatchResult::Draw,
            'Z' => MatchResult::Win,
            _ => MatchResult::Lose,
        },

        'C' => match me {
            'X' => MatchResult::Win,
            'Y' => MatchResult::Lose,
            'Z' => MatchResult::Draw,
            _ => MatchResult::Lose,
        },
        _ => MatchResult::Lose,
    }
}

fn calculate_total_points2(file_content: &[String]) -> u32 {
    let point_map: HashMap<char, u32> = HashMap::from([('X', 1), ('Y', 2), ('Z', 3)]);
    let mut total_point = 0;
    for st in file_content.iter() {
        let opponent = st.chars().next().expect("Error while reading the char");
        let me = st.chars().nth(1).expect("Error while reading the char");

        match calculate_match_result(me, opponent) {
            MatchResult::Win => {
                total_point += 6;
            }
            MatchResult::Draw => {
                total_point += 3;
            }
            _ => {}
        }

        total_point += point_map.get(&me).expect("Error while getting the point");
    }
    total_point
}
fn calculate_total_points(file_content: &[String]) -> u32 {
    let point_map: HashMap<char, u32> = HashMap::from([('X', 1), ('Y', 2), ('Z', 3)]);
    let mut total_point = 0;
    for st in file_content.iter() {
        let opponent = st.chars().next().expect("Error while reading the char");
        let me = st.chars().nth(1).expect("Error while reading the char");

        match calculate_match_result(me, opponent) {
            MatchResult::Win => {
                total_point += 6;
            }
            MatchResult::Draw => {
                total_point += 3;
            }
            _ => {}
        }

        total_point += point_map.get(&me).expect("Error while getting the point");
    }
    total_point
}

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

fn main() {
    let file_content = read_from_file();

    let win = HashMap::from([('A', 'Y'), ('B', 'Z'), ('C', 'X')]);
    let draw = HashMap::from([('A', 'X'), ('B', 'Y'), ('C', 'Z')]);
    let lose = HashMap::from([('A', 'Z'), ('B', 'X'), ('C', 'Y')]);
    let result_map = ResultMap {
        win: &win,
        draw: &draw,
        lose: &lose,
    };
    let result = calculate_total_points(&file_content);
    let result2 = result_map.calculate_total_points2(&file_content);

    println!("result1 {:?}", result);
    println!("result2 {:?}", result2);
}
