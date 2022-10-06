use std::fs;

fn main() {
    let input_string = fs::read_to_string("./input.txt").expect("File is in directory");
    let input_vector: Vec<&str> = input_string.lines().collect();
    let line_len = input_vector[0].len();
    let vector_len = input_vector.len();

    let mut mcb: Vec<u32> = vec![0; line_len];

    input_vector.into_iter().for_each(|line| {
        let mut numbers = line.chars();
        for pos in 0..mcb.len() {
            mcb[pos] += numbers.next().unwrap().to_digit(10).unwrap();
        }
    });

    let gamma_rate_vector: String = mcb
        .into_iter()
        .map(|n| {
            if n > (vector_len as u32 / 2 as u32) as u32 {
                1.to_string()
            } else {
                0.to_string()
            }
        })
        .collect();

    let base: u32 = 2;
    let gamma_rate = u32::from_str_radix(&gamma_rate_vector, 2).unwrap();

    let multiply = gamma_rate * (base.pow(line_len as u32) - 1 - gamma_rate);
    println!("{multiply}");
}
