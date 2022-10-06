use std::fs;

fn main() {
    let input_string = fs::read_to_string("input.txt").unwrap();
    let mut input_vector: Vec<i32> = input_string
        .lines()
        .next()
        .unwrap()
        .split(",")
        .map(|number| number.parse::<i32>().unwrap())
        .collect();

    //let median = calculate_median(&mut input_vector);
    //println!("{}", move_to_median(input_vector, median));
    let fuel = calculate_nonlinear(input_vector);
    println!("{fuel}");
}

fn calculate_nonlinear(mut input: Vec<i32>) -> i32 {
    let mut counter = 0;
    input.sort();
    for i in (0..input[input.len() - 1]).rev() {
        let mut counter_min = 0;
        for j in 0..input.len() {
            let n = (input[j] - i).abs();
            counter_min += (n * (n + 1)) / 2;
        }
        if counter == 0 {
            counter = counter_min;
        }
        if counter_min > counter {
            return counter;
        }
        counter = counter_min;
    }

    0
}

fn calculate_median(input: &mut Vec<i32>) -> i32 {
    input.sort();

    if input.len() % 2 == 0 {
        (input[input.len() / 2] + input[input.len() / 2 - 1]) / 2
    } else {
        input[input.len() / 2 - 1]
    }
}

fn move_to_median(input: Vec<i32>, median: i32) -> i32 {
    let mut counter = 0;
    for element in input {
        counter += (element - median).abs();
    }
    counter
}

#[cfg(test)]
mod tests {
    use crate::{calculate_median, calculate_nonlinear, move_to_median};

    #[test]
    fn test_input() {
        let median = calculate_median(&mut vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14]);
        assert_eq!(median, 2);
    }

    #[test]
    fn test_move() {
        let fuel = move_to_median(vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14], 2);
        assert_eq!(fuel, 37);
        let fuel = move_to_median(vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14], 1);
        assert_eq!(fuel, 41);
        let fuel = move_to_median(vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14], 3);
        assert_eq!(fuel, 39);
        let fuel = move_to_median(vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14], 10);
        assert_eq!(fuel, 71);
    }

    #[test]
    fn test_nonlinear() {
        let fuel = calculate_nonlinear(vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14]);
        assert_eq!(fuel, 168);
    }
}
