use std::fs;

fn main() {
    let input_string = fs::read_to_string("./input.txt").expect("File is in directory");
    let input_vector: Vec<&str> = input_string.lines().collect();

    let oxygen_rating = rating(input_vector, &oxygen_rating);

    let input_vector: Vec<&str> = input_string.lines().collect();
    let co2_rating = rating(input_vector, &co2_rating);

    println!("{}", oxygen_rating * co2_rating);
}

fn rating(
    input_vector: Vec<&str>,
    rating_function: &dyn Fn(f32, u32, Vec<&str>, usize) -> Vec<&str>,
) -> u32 {
    let mut input_vector = input_vector;
    for i in 0..12 {
        let input_len = input_vector.len() as u32;
        let mut counter = 0.0;

        if input_len == 1 {
            break;
        }

        for line in &input_vector {
            counter += line.chars().nth(i).unwrap().to_digit(10).unwrap() as f32;
        }

        input_vector = rating_function(counter, input_len, input_vector, i);
    }
    u32::from_str_radix(input_vector[0], 2).unwrap()
}

fn co2_rating(counter: f32, input_len: u32, input_vector: Vec<&str>, position: usize) -> Vec<&str> {
    match counter < input_len as f32 / 2.0 {
        true => filter_vector(input_vector, position, '1'),
        false => filter_vector(input_vector, position, '0'),
    }
}

fn oxygen_rating(
    counter: f32,
    input_len: u32,
    input_vector: Vec<&str>,
    position: usize,
) -> Vec<&str> {
    match counter >= input_len as f32 / 2.0 {
        true => filter_vector(input_vector, position, '1'),
        false => filter_vector(input_vector, position, '0'),
    }
}

fn filter_vector(input_vector: Vec<&str>, position: usize, filter_for: char) -> Vec<&str> {
    input_vector
        .into_iter()
        .filter(|line| line.chars().nth(position).unwrap().eq(&filter_for))
        .collect()
}
