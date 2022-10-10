use std::fs;

fn main() {
    let height_map = read_file("input.txt");
    let output = search_lowest(height_map);

    println!("{}", calculate_lowest(output));
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

fn search_lowest(height_map: Vec<String>) -> Vec<(char, usize, usize)> {
    let mut lowest = Vec::new();
    for i in 0..height_map.len() {
        for j in 0..height_map[i].len() {
            let i = i as i32;
            let j = j as i32;
            let adjacent = vec![(i - 1, j), (i + 1, j), (i, j + 1), (i, j - 1)];
            let adjacent = adjacent
                .iter()
                .filter(|(k, l)| {
                    (k >= &0)
                        & (k < &(height_map.len() as i32))
                        & (l >= &0)
                        & (l < &(height_map[i as usize].len() as i32))
                })
                .collect::<Vec<&(i32, i32)>>();

            let i = i as usize;
            let j = j as usize;
            if adjacent.iter().all(|(k, l)| {
                height_map[i].chars().nth(j).unwrap()
                    < height_map[*k as usize].chars().nth(*l as usize).unwrap()
            }) {
                lowest.push((height_map[i].chars().nth(j).unwrap(), i, j));
            }
        }
    }

    lowest
}

#[cfg(test)]
mod tests {
    use crate::{calculate_lowest, read_file, search_lowest};
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
    fn ouptut() {
        let height_map = read_file("test_input.txt");
        let lowest = search_lowest(height_map);
        let lowest_expected: Vec<(char, usize, usize)> =
            vec![('1', 0, 1), ('0', 0, 9), ('5', 2, 2), ('5', 4, 6)];
        assert_eq!(lowest, lowest_expected);
    }

    #[test]
    fn calculate_output() {
        let height_map = read_file("test_input.txt");
        let lowest = search_lowest(height_map);
        let output = calculate_lowest(lowest);
        assert_eq!(15, output);
    }
}
