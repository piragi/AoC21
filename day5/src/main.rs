use std::collections::HashMap;
use std::fs;
use std::ops::RangeInclusive;

#[derive(Debug)]
struct Lines {
    p1: (i32, i32),
    p2: (i32, i32),
}

fn main() {
    let lines_vec = read_input();

    let mut lines_map = HashMap::new();

    for line in lines_vec {
        if (line.p1.0 - line.p2.0).abs() == (line.p1.1 - line.p2.1).abs() {
            for i in 0..=(line.p1.0 - line.p2.0).abs() {
                let gradient_x = match line.p1.0 >= line.p2.0 {
                    true => line.p1.0 - i,
                    false => line.p1.0 + i,
                };
                let gradient_y = match line.p1.1 >= line.p2.1 {
                    true => line.p1.1 - i,
                    false => line.p1.1 + i,
                };
                let entry = lines_map.entry((gradient_x, gradient_y)).or_insert(0);
                *entry += 1;
            }
        } else if line.p1.0 == line.p2.0 {
            for i in range(line.p1.1, line.p2.1) {
                let entry = lines_map.entry((line.p1.0, i)).or_insert(0);
                *entry += 1;
            }
        } else if line.p1.1 == line.p2.1 {
            for i in range(line.p1.0, line.p2.0) {
                let entry = lines_map.entry((i, line.p1.1)).or_insert(0);
                *entry += 1;
            }
        }
    }

    let mut counter = 0;
    for entry in &lines_map {
        if *entry.1 > 1 {
            counter += 1;
        }
    }
    println!("{:?}", counter);
}

fn range(p1: i32, p2: i32) -> RangeInclusive<i32> {
    match p1 <= p2 {
        true => p1..=p2,
        false => p2..=p1,
    }
}

fn read_input() -> Vec<Lines> {
    let input_string = fs::read_to_string("./input.txt").expect("file in directory");
    input_string
        .lines()
        .map(|line| {
            let points: Vec<&str> = line.split(" -> ").collect();
            let point1: Vec<&str> = points[0].split(",").collect();
            let point2: Vec<&str> = points[1].split(",").collect();

            Lines {
                p1: (
                    point1[0].parse::<i32>().unwrap(),
                    point1[1].parse::<i32>().unwrap(),
                ),
                p2: (
                    point2[0].parse::<i32>().unwrap(),
                    point2[1].parse::<i32>().unwrap(),
                ),
            }
        })
        .collect()
}
