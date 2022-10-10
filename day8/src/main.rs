use std::fs;

#[derive(Debug)]
struct Digits {
    pattern: Vec<String>,
    output: Vec<String>,
}

fn main() {
    let digits = read_file("input.txt");
    let numbers = unique_patterns(digits);
    println!("{:?}", numbers)
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

fn unique_patterns(digits: Vec<Digits>) -> i64 {
    let mut counter: i64 = 0;

    for digit in digits {
        let mut numbers = vec![String::new(); 10];

        for i in 0..=2 {
            for pattern in &digit.pattern {
                let pattern = pattern.to_string();
                if i == 0 {
                    match pattern.len() {
                        2 => numbers[1] = pattern,
                        3 => numbers[7] = pattern,
                        4 => numbers[4] = pattern,
                        7 => numbers[8] = pattern,
                        _ => continue,
                    }
                } else if i == 1 && pattern.len() == 6 {
                    if numbers[4].chars().all(|char| pattern.contains(char)) {
                        numbers[9] = pattern;
                    } else if numbers[1].chars().all(|char| pattern.contains(char)) {
                        numbers[0] = pattern;
                    } else {
                        numbers[6] = pattern;
                    }
                } else if i == 2 && pattern.len() == 5 {
                    if numbers[1].chars().all(|char| pattern.contains(char)) {
                        numbers[3] = pattern;
                    } else if numbers[8]
                        .chars()
                        .filter(|char| numbers[6].contains(*char))
                        .filter(|char| numbers[9].contains(*char))
                        .all(|char| pattern.contains(char))
                    {
                        numbers[5] = pattern;
                    } else {
                        numbers[2] = pattern;
                    }
                }
            }
        }
        let mut outputs = String::from("");
        for output in digit.output {
            for i in 0..numbers.len() {
                if output.chars().all(|char| numbers[i].contains(char))
                    && output.len() == numbers[i].len()
                {
                    outputs.push_str(i.to_string().as_str());
                }
            }
        }
        counter += outputs.parse::<i64>().unwrap();
    }

    counter
}

#[cfg(test)]
mod tests {
    use crate::{read_file, unique_patterns};

    #[test]
    fn test_input() {
        let digits = read_file("test_input.txt");
        let numbers = unique_patterns(digits);
        assert_eq!(61229, numbers);
    }
}
