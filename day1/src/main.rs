use std::fs;

fn main() {
    let input = fs::read_to_string("./input.txt").expect("input file is in directory");
    let mut counter = 0;

    let input_iterator: Vec<u32> = input.split('\n').map(|n| n.parse().unwrap()).collect();

    for i in 3..input_iterator.len() {
        if input_iterator[i] > input_iterator[i - 3] {
            counter += 1;
        }
    }

    println!("{counter}");
}
