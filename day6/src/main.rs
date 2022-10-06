use std::fs;

const LANTERN_REGEN_CYCLE: usize = 6;
const LANTERN_NEWBORN_CYCLE: usize = 8;

fn main() {
    let days = 256;
    let lantern_vec = read_file("input.txt");
    println!(
        "{:#?}",
        evolution(lantern_vec, days).into_iter().sum::<i64>()
    );
}

fn read_file(path: &str) -> Vec<i64> {
    let input_string = fs::read_to_string(path).expect("file in directory");
    let input_vector: Vec<&str> = input_string.lines().next().unwrap().split(",").collect();
    let mut lantern_vec = vec![0; LANTERN_NEWBORN_CYCLE + 1];

    for entry in input_vector {
        let value = entry.parse::<usize>().unwrap();
        lantern_vec[value] += 1;
    }
    lantern_vec
}

fn evolution(mut lantern_vec: Vec<i64>, days: usize) -> Vec<i64> {
    for _i in 0..days {
        let mut cache = lantern_vec[LANTERN_NEWBORN_CYCLE];
        for i in (0..lantern_vec.len()).rev() {
            match i {
                0 => {
                    lantern_vec[LANTERN_NEWBORN_CYCLE] = cache;
                    lantern_vec[LANTERN_REGEN_CYCLE] += cache;
                }
                _ => {
                    let cache2 = cache;
                    cache = lantern_vec[i - 1];
                    lantern_vec[i - 1] = cache2;
                }
            }
        }
    }

    lantern_vec
}

#[cfg(test)]
mod tests {
    use crate::{evolution, read_file};

    #[test]
    fn test_input() {
        let test_output = read_file("./test_input.txt");
        assert_eq!(test_output, [0, 1, 1, 2, 1, 0, 0, 0, 0]);
    }
    #[test]
    fn test_evolution_once() {
        let test_output = read_file("./test_input.txt");
        let test_evolution = evolution(test_output, 1);
        assert_eq!(test_evolution, [1, 1, 2, 1, 0, 0, 0, 0, 0]);
    }
    #[test]
    fn test_evolution_overflow() {
        let test_output = vec![1, 0, 0, 0, 0, 0, 0, 0, 0];
        let test_evolution = evolution(test_output, 1);
        assert_eq!(test_evolution, [0, 0, 0, 0, 0, 0, 1, 0, 1]);
    }
    #[test]
    fn test_evolution_twice() {
        let test_output = read_file("./test_input.txt");
        let test_evolution = evolution(test_output, 2);
        assert_eq!(test_evolution, [1, 2, 1, 0, 0, 0, 1, 0, 1]);
    }
}
