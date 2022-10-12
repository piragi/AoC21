extern crate core;

use std::fs;

fn main() {
    let input = read_file("input.txt");
    let score = check_validity(input);
    println!("{}", score);
}

fn check_validity(lines: Vec<String>) -> i32 {
    let mut score = 0;
    for line in lines {
        let mut stack = Vec::new();
        for char in line.chars() {
            match char {
                '{' | '[' | '(' | '<' => stack.push(char),
                ')' | ']' | '}' | '>' => {
                    if !stack.is_empty() & (char == matching_delimiter(*stack.last().unwrap())) {
                        stack.pop();
                    } else {
                        score += get_score(char);
                        break;
                    }
                }
                _ => panic!("invalid character in input"),
            }
        }
    }
    score
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
    use crate::{check_validity, read_file};

    #[test]
    fn test_validity() {
        let input = vec![String::from("{{}")];
        let score = check_validity(input);
        assert_eq!(score, 0);
    }
    #[test]
    fn test_illegal() {
        let input = read_file("test_input.txt");
        let score = check_validity(input);
        assert_eq!(score, 26397);
    }
}
