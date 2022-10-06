use std::fs;

fn main() {
    let mut x = 0;
    let mut z = 0;
    let mut aim = 0;
    let input_string: String = fs::read_to_string("./input.txt").expect("File is in directory");
    let input_vector: Vec<&str> = input_string.lines().collect();

    for line in input_vector {
        let separated: Vec<&str> = line.split_ascii_whitespace().collect();
        let separated_num = separated[1].parse::<i32>().unwrap();
        match separated[0] {
            "forward" => {
                x += separated_num;
                z += aim * separated_num;
            }
            "down" => aim += separated_num,
            "up" => aim -= separated_num,
            _ => println!("Error"),
        }
    }

    println!("{}", x * z);
}
