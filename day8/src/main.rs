use std::fs;

#[derive(Debug)]
struct Digits {
    pattern: Vec<String>,
    output: Vec<String>,
}

fn main() {
    let digits = read_file("input.txt");
    let counter = unique_patterns(digits);
    println!("{}", counter);
}

fn read_file(path: &str) -> Vec<Digits> {
    let input_string = fs::read_to_string(path).expect("input in directory");
    input_string
        .lines()
        .map(|line| {
            let collected = line.split('|').collect::<Vec<&str>>();
            let pattern = collected[0]
                .split_ascii_whitespace()
                .map(String::from)
                .collect();
            let output = collected[1]
                .split_ascii_whitespace()
                .map(String::from)
                .collect();

            Digits { pattern, output }
        })
        .collect()
}

fn unique_patterns(digits: Vec<Digits>) -> i32 {
    let mut counter = 0;
    for digit in digits {
        for output in digit.output {
            match output.len() {
                2 | 3 | 4 | 7 => counter += 1,
                _ => continue,
            }
        }
    }
    counter
}

#[cfg(test)]
mod tests {
    use crate::{read_file, unique_patterns};

    #[test]
    fn test_input() {
        let digits = read_file("test_input.txt");
        let counter = unique_patterns(digits);
        assert_eq!(26, counter);
    }
}
