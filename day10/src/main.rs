extern crate core;

use std::fs;

fn main() {
    let input = read_file("input.txt");
    let (score, legal) = check_validity(input);
    let overall_score = calc_corrected(legal);
    println!("score: {}", score);
    println!("overall_score: {}", overall_score);
}

fn calc_corrected(legal: Vec<String>) -> i64 {
    let mut scores = Vec::new();

    for line in legal {
        let mut score = 0;
        for char in line.chars() {
            match char {
                '}' => {
                    score *= 5;
                    score += 3;
                }
                ']' => {
                    score *= 5;
                    score += 2;
                }
                ')' => {
                    score *= 5;
                    score += 1;
                }
                '>' => {
                    score *= 5;
                    score += 4;
                }
                _ => panic!("invalid character in score"),
            };
        }
        scores.push(score);
    }
    scores.sort_unstable();
    scores[scores.len() / 2]
}

fn check_validity(lines: Vec<String>) -> (i32, Vec<String>) {
    let mut score = 0;
    let mut auto_corrections = Vec::new();
    for line in lines {
        let mut illegal = false;
        let mut stack = Vec::new();
        for char in line.chars() {
            match char {
                '{' | '[' | '(' | '<' => stack.push(char),
                ')' | ']' | '}' | '>' => {
                    if !stack.is_empty() & (char == matching_delimiter(*stack.last().unwrap())) {
                        stack.pop();
                    } else {
                        score += get_score(char);
                        illegal = true;
                        break;
                    }
                }
                _ => panic!("invalid character in input"),
            }
        }
        if !illegal {
            let mut auto_corrected = String::new();
            stack.reverse();
            for element in &stack {
                auto_corrected.push(matching_delimiter(*element));
            }
            auto_corrections.push(auto_corrected);
        }
    }
    (score, auto_corrections)
}

fn get_score(char: char) -> i32 {
    match char {
        '}' => 1197,
        ']' => 57,
        ')' => 3,
        '>' => 25137,
        _ => panic!("invalid character in score"),
    }
}

fn matching_delimiter(char: char) -> char {
    match char {
        '{' => '}',
        '[' => ']',
        '(' => ')',
        '<' => '>',
        _ => panic!("invalid character in stack"),
    }
}

fn read_file(path: &str) -> Vec<String> {
    fs::read_to_string(path)
        .expect("file in directory")
        .lines()
        .map(|line| line.to_string())
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::{calc_corrected, check_validity, read_file};

    #[test]
    fn test_validity() {
        let input = vec![String::from("{{}")];
        let (score, _) = check_validity(input);
        assert_eq!(score, 0);
    }
    #[test]
    fn test_illegal() {
        let input = read_file("test_input.txt");
        let (score, _) = check_validity(input);
        assert_eq!(score, 26397);
    }
    #[test]
    fn test_auto_correct() {
        let input = read_file("test_input.txt");
        let (_, auto_corrected) = check_validity(input);
        let expected = vec![
            String::from("}}]])})]"),
            String::from(")}>]})"),
            String::from("}}>}>))))"),
            String::from("]]}}]}]}>"),
            String::from("])}>"),
        ];
        assert_eq!(auto_corrected, expected)
    }

    #[test]
    fn test_auto_score() {
        let input = read_file("test_input.txt");
        let (_, auto_corrected) = check_validity(input);
        let score = calc_corrected(auto_corrected);
        assert_eq!(score, 288957);
    }
}
