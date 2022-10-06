use std::fs;

fn main() {
    let mut x = 0;
    let mut z = 0;
    let input_string: String = fs::read_to_string("./input.txt").expect("File is in directory");
    let input_vector: Vec<&str> = input_string.lines().collect();

    for line in input_vector {
        let separated: Vec<&str> = line.split_ascii_whitespace().collect();
        match separated[0] {
            "forward" => x += separated[1].parse::<i32>().unwrap(),
            "down" => z += separated[1].parse::<i32>().unwrap(),
            "up" => z -= separated[1].parse::<i32>().unwrap(),
            _ => println!("Error"),
        }
    }

    println!("{}", x * z);
}
