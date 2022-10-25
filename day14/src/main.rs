use std::{collections::HashMap, fs, time::Instant};

fn main() {
    let start_time = Instant::now();

    let (mut pattern, rules) = read_file("input.txt");
    for i in 0..40 {
        println!("{i}");
        pattern = apply_rules(pattern.clone(), &rules);
    }

    println!("{:?}", count_polymers(pattern));
    let duration = start_time.elapsed();
    println!("Duration: {:?}\n", duration);
}

fn count_polymers(pattern: String) -> i64 {
    let mut counter = HashMap::new();
    for char in pattern.chars() {
        let counted = counter.entry(char).or_insert(0);
        *counted += 1;
    }

    let mut min = *counter.iter().next().unwrap().1;
    let mut max = *counter.iter().next().unwrap().1;
    for (_, element) in counter {
        if element > max {
            max = element;
        } else if element < min {
            min = element;
        }
    }

    max - min
}

fn read_file(path: &str) -> (String, HashMap<String, String>) {
    let input = fs::read_to_string(path).unwrap();
    let pattern = input.lines().next().unwrap().to_string();
    let input = input.trim_start_matches(&pattern).trim_start();
    let rules = input
        .lines()
        .map(|line| {
            let split = line.split(" -> ").collect::<Vec<&str>>();
            (split[0].to_string(), split[1].to_string())
        })
        .collect::<HashMap<String, String>>();

    (pattern, rules)
}

fn apply_rules(pattern: String, rules: &HashMap<String, String>) -> String {
    let mut pattern_new = String::new();
    pattern_new.push(pattern.chars().next().unwrap());

    for i in 0..pattern.len() - 1 {
        let mut check = String::new();
        check.push(pattern.chars().nth(i).unwrap());
        check.push(pattern.chars().nth(i + 1).unwrap());
        match rules.get(&check) {
            Some(x) => {
                pattern_new.push_str(x);
                pattern_new.push(pattern.chars().nth(i + 1).unwrap());
            }
            None => continue,
        }
    }
    pattern_new
}

#[cfg(test)]
mod tests {
    use crate::{apply_rules, count_polymers, read_file};

    #[test]
    fn test_first_rule() {
        let (pattern, rules) = read_file("test_input.txt");
        let pattern = apply_rules(pattern, &rules);
        assert_eq!("NCNBCHB", &pattern);
    }
    #[test]
    fn test_ten_rules() {
        let (mut pattern, rules) = read_file("test_input.txt");
        for _i in 0..10 {
            pattern = apply_rules(pattern.clone(), &rules);
        }
        let count = count_polymers(pattern);

        assert_eq!(1588, count);
    }
}
