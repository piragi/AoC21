use std::{collections::HashMap, fs, time::Instant};

//runtime release
//with encoding of pairs to numbers: 355.188µs
//without encoding: 776.151µs
fn main() {
    let start_time = Instant::now();

    let (pattern, rules) = read_file("input.txt");
    let mut pattern = encode_pattern(pattern);

    for _i in 0..40 {
        pattern = apply_rules_encoding(pattern.clone(), &rules);
    }

    count_polymers_encoded(pattern);

    let duration = start_time.elapsed();
    println!("Duration: {:?}\n", duration);
}

fn count_polymers_encoded(pattern: HashMap<String, u64>) {
    let mut polymers_counter = HashMap::new();

    for (key, value) in pattern {
        let c1 = key.chars().next().unwrap();
        let c2 = key.chars().nth(1).unwrap();
        let entry = polymers_counter.entry(c1).or_insert(0);
        *entry += value;
        let entry = polymers_counter.entry(c2).or_insert(0);
        *entry += value;
    }

    let mut min = *polymers_counter.iter().next().unwrap().1;
    let mut max = *polymers_counter.iter().next().unwrap().1;
    for (_, element) in polymers_counter {
        if element > max {
            max = element;
        } else if element < min {
            min = element;
        }
    }

    let max = (max as f64 / 2.0).ceil();
    let min = (min as f64 / 2.0).ceil();

    println!("{}", max - min);
}

fn read_file(path: &str) -> (String, HashMap<String, (String, String)>) {
    let input = fs::read_to_string(path).unwrap();
    let pattern = input.lines().next().unwrap().to_string();
    let input = input.trim_start_matches(&pattern).trim_start();
    let rules = input
        .lines()
        .map(|line| {
            let split = line.split(" -> ").collect::<Vec<&str>>();
            let mut result1 = String::new();
            let mut result2 = String::new();
            result1.push(split[0].chars().next().unwrap());
            result1.push_str(split[1]);
            result2.push_str(split[1]);
            result2.push(split[0].chars().nth(1).unwrap());

            (split[0].to_string(), (result1, result2))
        })
        .collect::<HashMap<String, (String, String)>>();

    (pattern, rules)
}

fn encode_pattern(pattern: String) -> HashMap<String, u64> {
    let mut pattern_new = HashMap::new();

    for i in 0..pattern.len() - 1 {
        let mut check = String::new();
        check.push(pattern.chars().nth(i).unwrap());
        check.push(pattern.chars().nth(i + 1).unwrap());

        let entry = pattern_new.entry(check).or_insert(0_u64);
        *entry += 1;
    }
    pattern_new
}

fn apply_rules_encoding(
    pattern: HashMap<String, u64>,
    rules: &HashMap<String, (String, String)>,
) -> HashMap<String, u64> {
    let mut pattern_new = HashMap::new();
    for (key, value) in pattern {
        let (pattern1, pattern2) = rules.get(&key).unwrap();
        let count1 = pattern_new.entry(pattern1.clone()).or_insert(0);
        *count1 += value;
        let count2 = pattern_new.entry(pattern2.clone()).or_insert(0);
        *count2 += value;
    }
    pattern_new
}
