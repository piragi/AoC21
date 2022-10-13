use std::cmp::Ordering;
use std::fs;

const GRID_LENGTH: i32 = 10;

fn main() {
    let octopus = read_file("input.txt");
    let (_, score) = energy_evolution(100, octopus);
    println!("{}", score);
}

fn read_file(path: &str) -> Vec<Vec<u32>> {
    fs::read_to_string(path)
        .expect("file expected in directory")
        .lines()
        .map(|line| {
            line.chars()
                .map(|char| char.to_digit(10).unwrap())
                .collect()
        })
        .collect()
}

fn increase_adj(row: i32, column: i32, octopus: &mut Vec<Vec<u32>>) -> Vec<(usize, usize)> {
    let row = row as i32;
    let column = column as i32;
    let adjacent = vec![
        (row + 1, column + 1),
        (row + 1, column),
        (row + 1, column - 1),
        (row, column + 1),
        (row, column - 1),
        (row - 1, column + 1),
        (row - 1, column),
        (row - 1, column - 1),
    ];

    let adjacent = adjacent
        .iter()
        .filter(|(x, y)| (*x >= 0) && (*y >= 0) && (*x < GRID_LENGTH) && *y < GRID_LENGTH)
        .map(|(x, y)| (*x as usize, *y as usize))
        .collect::<Vec<(usize, usize)>>();

    for (adj_row, adj_column) in &adjacent {
        octopus[*adj_row][*adj_column] += 1;
    }

    for (adj_x, adj_y) in adjacent
        .into_iter()
        .filter(|(x, y)| octopus[*x][*y] == 10)
        .collect::<Vec<(usize, usize)>>()
    {
        increase_adj(adj_x as i32, adj_y as i32, octopus);
    }
    Vec::new()
}

fn energy_evolution(steps: usize, mut octopus: Vec<Vec<u32>>) -> (Vec<Vec<u32>>, i32) {
    let mut flash_counter = 0;
    let mut step_sync = 0;
    loop {
        let mut flash_queue = Vec::new();

        for row in 0..octopus.len() {
            for column in 0..octopus[row].len() {
                octopus[row][column] += 1;
                if octopus[row][column] > 9 {
                    flash_queue.push((row, column));
                }
            }
        }

        for i in 0..flash_queue.len() {
            increase_adj(
                flash_queue[i].0 as i32,
                flash_queue[i].1 as i32,
                &mut octopus,
            );
        }

        step_sync += 1;

        octopus = octopus
            .into_iter()
            .map(|row| {
                row.into_iter()
                    .map(|element| match element.cmp(&9) {
                        Ordering::Greater => {
                            flash_counter += 1;
                            0
                        }
                        _ => element,
                    })
                    .collect()
            })
            .collect();

        if flash_counter == 100 {
            println!("step: {step_sync}");
            break;
        }
        flash_counter = 0;
    }
    (octopus, flash_counter)
}

#[cfg(test)]
mod tests {
    use crate::{energy_evolution, read_file};

    #[test]
    fn read_test() {
        let input = read_file("test_input.txt");
        let expected = vec![
            [1, 1, 1, 1, 1],
            [1, 9, 9, 9, 1],
            [1, 9, 1, 9, 1],
            [1, 9, 9, 9, 1],
            [1, 1, 1, 1, 1],
        ];
        assert_eq!(input, expected);
    }

    #[test]
    fn step1() {
        let input = read_file("test_input.txt");
        let (output, _) = energy_evolution(1, input);
        let expected = vec![
            [3, 4, 5, 4, 3],
            [4, 0, 0, 0, 4],
            [5, 0, 0, 0, 5],
            [4, 0, 0, 0, 4],
            [3, 4, 5, 4, 3],
        ];
        assert_eq!(output, expected);
    }
    #[test]
    fn step2() {
        let input = read_file("test_input.txt");
        let (output, _) = energy_evolution(2, input);
        let expected = vec![
            [4, 5, 6, 5, 4],
            [5, 1, 1, 1, 5],
            [6, 1, 1, 1, 6],
            [5, 1, 1, 1, 5],
            [4, 5, 6, 5, 4],
        ];
        assert_eq!(output, expected);
    }

    #[test]
    fn flash_counter_small() {
        let input = read_file("test_input.txt");
        let (_, flash_counter) = energy_evolution(2, input);
        let expected = 9;
        assert_eq!(flash_counter, expected);
    }

    #[test]
    fn flash_counter() {
        let input = read_file("test_larger.txt");
        let (_, flash_counter) = energy_evolution(100, input);
        let expected = 1656;
        assert_eq!(flash_counter, expected);
    }
}
