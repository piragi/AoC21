use std::collections::HashMap;
use std::fs;

#[derive(PartialEq, Debug)]
struct Basins {
    lowest: (char, usize, usize),
    locations: Vec<(usize, usize)>,
}

fn main() {
    let height_map = read_file("input.txt");
    let (height, width) = (height_map.len(), height_map[0].len());
    let output = search_lowest(&height_map);
    //println!("{}", calculate_lowest(output));
    let basins = calculate_basins(output, &height_map, height, width);
    println!("{}", basins);
}

fn calculate_basins(
    lowest: Vec<(char, usize, usize)>,
    height_map: &Vec<String>,
    height: usize,
    width: usize,
) -> usize {
    let basins = lowest
        .iter()
        .map(|lowest| Basins {
            lowest: *lowest,
            locations: Vec::new(),
        })
        .collect::<Vec<Basins>>();

    let basins = basins
        .iter()
        .map(|basin| {
            let lowest_adjacent =
                get_adjacent_fields(basin.lowest.1 as i32, basin.lowest.2 as i32, height, width);

            let mut filtered_adjacent = lowest_adjacent
                .into_iter()
                .filter(|(k, l)| height_map[*k].chars().nth(*l).unwrap() < '9')
                .collect::<Vec<(usize, usize)>>();

            for i in 0.. {
                match filtered_adjacent.get(i) {
                    Some(element) => {
                        let adjacent = get_adjacent_fields(
                            filtered_adjacent[i].0 as i32,
                            filtered_adjacent[i].1 as i32,
                            height,
                            width,
                        );
                        let mut new_adjacent = adjacent
                            .into_iter()
                            .filter(|(k, l)| {
                                !filtered_adjacent.contains(&(*k, *l))
                                    & (height_map[*k].chars().nth(*l).unwrap() < '9')
                            })
                            .collect::<Vec<(usize, usize)>>();
                        filtered_adjacent.append(&mut new_adjacent);
                    }
                    None => break,
                }
            }

            Basins {
                lowest: basin.lowest,
                locations: filtered_adjacent,
            }
        })
        .collect::<Vec<Basins>>();

    let mut basins_len = basins
        .iter()
        .map(|basin| basin.locations.len())
        .collect::<Vec<usize>>();
    basins_len.sort();
    let mut counter = 1;
    for i in basins_len.len() - 3..basins_len.len() {
        counter *= basins_len[i];
    }
    counter
}

fn calculate_lowest(output: Vec<(char, usize, usize)>) -> u32 {
    let mut counter = 0;
    for element in output {
        counter += element.0.to_digit(10).unwrap() + 1;
    }
    counter
}

fn read_file(path: &str) -> Vec<String> {
    let input_string = fs::read_to_string(path).unwrap();
    input_string.lines().map(|line| line.to_string()).collect()
}

fn get_adjacent_fields(i: i32, j: i32, height: usize, width: usize) -> Vec<(usize, usize)> {
    let adjacent = vec![(i - 1, j), (i + 1, j), (i, j + 1), (i, j - 1)];
    adjacent
        .into_iter()
        .filter(|(k, l)| (k >= &0) & (k < &(height as i32)) & (l >= &0) & (l < &(width as i32)))
        .map(|(k, l)| (k as usize, l as usize))
        .collect::<Vec<(usize, usize)>>()
}

fn search_lowest(height_map: &Vec<String>) -> Vec<(char, usize, usize)> {
    let mut lowest = Vec::new();
    for i in 0..height_map.len() {
        for j in 0..height_map[i].len() {
            let i = i as i32;
            let j = j as i32;
            let adjacent =
                get_adjacent_fields(i, j, height_map.len(), height_map[i as usize].len());

            let i = i as usize;
            let j = j as usize;
            if adjacent.iter().all(|(k, l)| {
                height_map[i].chars().nth(j).unwrap() < height_map[*k].chars().nth(*l).unwrap()
            }) {
                lowest.push((height_map[i].chars().nth(j).unwrap(), i, j));
            }
        }
    }

    lowest
}

#[cfg(test)]
mod tests {
    use crate::{calculate_basins, calculate_lowest, read_file, search_lowest, Basins};
    use std::collections::HashMap;
    use std::fs::read;

    #[test]
    fn read_input() {
        let input = vec![
            "2199943210",
            "3987894921",
            "9856789892",
            "8767896789",
            "9899965678",
        ];
        let height_map = read_file("test_input.txt");
        assert_eq!(height_map, input);
    }

    #[test]
    fn output() {
        let height_map = read_file("test_input.txt");
        let lowest = search_lowest(&height_map);
        let lowest_expected: Vec<(char, usize, usize)> =
            vec![('1', 0, 1), ('0', 0, 9), ('5', 2, 2), ('5', 4, 6)];
        assert_eq!(lowest, lowest_expected);
    }

    #[test]
    fn calculate_output() {
        let height_map = read_file("test_input.txt");
        let lowest = search_lowest(&height_map);
        let output = calculate_lowest(lowest);
        assert_eq!(15, output);
    }

    #[test]
    fn get_basins() {
        let height_map = read_file("test_input.txt");
        let lowest = search_lowest(&height_map);
        let (height, width) = (height_map.len(), height_map[0].len());
        let basins = calculate_basins(lowest, &height_map, height_map.len(), height_map[0].len());
        assert_eq!(1134, basins);
    }
}
