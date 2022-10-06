use std::collections::HashMap;
use std::fs;

fn main() {
    let mut lantern_hash = read_file("test_input.txt");
    evolution(lantern_hash);
}

fn read_file(path: &str) -> HashMap<i32, i32> {
    let input_string = fs::read_to_string(path).expect("file in directory");
    let input_vector: Vec<&str> = input_string.lines().next().unwrap().split(",").collect();
    let mut lantern_hash = (0..8).map(|key| (key, 0_i32)).collect::<HashMap<_, _>>();

    for entry in input_vector {
        let value = lantern_hash
            .entry(entry.parse::<i32>().unwrap())
            .or_insert(0);
        *value += 1;
    }
    lantern_hash
}

fn evolution(mut lantern_hash: HashMap<i32, i32>) {
    let mut cache = 0;

    for i in (1..=8).rev() {
        match i {
            1 => {
                lantern_hash.entry(8) = lantern_hash.get(&i).unwrap();
                lantern_hash.entry(6).unwrap() = &cache;
            }
            8 => {
                cache = *lantern_hash.entry(i - 1).unwrap();
                lantern_hash.entry(i - 1).unwrap() = lantern_hash.get(&i).unwrap();
            }
            _ => {
                lantern_hash.entry(i - 1).unwrap() = &cache;
                cache = *lantern_hash.entry(i - 1).unwrap();
            }
        }
    }

    println!("{:#?}", lantern_hash);
}

#[cfg(test)]
mod tests {
    use crate::read_file;

    #[test]
    fn test_input() {
        let test_output = read_file("./test_input.txt");
    }
}
