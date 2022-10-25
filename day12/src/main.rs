use std::{collections::HashMap, fs};

#[derive(PartialEq)]
struct Cave {
    name: String,
    routes: Vec<Cave>,
}
fn main() {
    let connections = read_input("./input.txt");
    let mut cave_hash = connect_caves(connections);

    let paths = traverse_paths(
        &mut cave_hash,
        "start",
        HashMap::new(),
        vec!["start".to_string()],
        false,
    );
    println!("len: {}", paths.0.len());
}

fn read_input(path: &str) -> Vec<Vec<String>> {
    let input_lines = fs::read_to_string(path).expect("file expected in directory");
    input_lines
        .lines()
        .map(|line| line.split('-').map(|str| str.to_string()).collect())
        .collect()
}

fn connect_caves(connections: Vec<Vec<String>>) -> HashMap<String, Vec<String>> {
    let mut cave_hash: HashMap<String, Vec<String>> = HashMap::new();
    let mut ends = Vec::new();
    for end in connections {
        if end[1] == "end" {
            ends.push(end[0].clone());
            continue;
        } else if end[0] == "end" {
            ends.push(end[1].clone());
            continue;
        }

        let connection = cave_hash.entry(end[0].clone()).or_insert(Vec::new());
        connection.push(end[1].clone());
        let connection = cave_hash.entry(end[1].clone()).or_insert(Vec::new());
        connection.push(end[0].clone());
    }

    for end in ends {
        let connection = cave_hash.get_mut(&end).unwrap();
        connection.push("end".to_string());
    }

    cave_hash
}

fn traverse_paths(
    cave_hash: &mut HashMap<String, Vec<String>>,
    entry: &str,
    mut visited_hash: HashMap<String, i32>,
    mut path: Vec<String>,
    mut visited_twice: bool,
) -> (Vec<Vec<String>>, bool) {
    let mut paths = Vec::new();
    let start = cave_hash.get(entry).unwrap().clone();

    for route in start {
        while path.last().unwrap() != entry {
            let element = path.pop().unwrap();
            let entry = visited_hash.get_mut(&element).unwrap();
            if *entry == 2 && element.chars().next().unwrap().is_lowercase() {
                visited_twice = false;
            }
            *entry -= 1;
        }
        // visited_twice = visited_hash
        //     .iter()
        //     .filter(|(key, _)| key.chars().next().unwrap().is_lowercase())
        //     .any(|(_, value)| *value >= 2);

        let visit = visited_hash.entry(route.to_string()).or_insert(0);

        if route == "end" {
            path.push(route.clone());
            paths.push(path);
            return (paths, visited_twice);
        } else if (route.chars().next().unwrap().is_lowercase() && *visit >= 1 && visited_twice)
            || route == "start"
        {
            continue;
        } else if route.chars().next().unwrap().is_lowercase() && *visit == 1 && !visited_twice {
            visited_twice = true;
        }

        path.push(route.clone());
        *visit += 1;

        paths.append(
            &mut traverse_paths(
                cave_hash,
                &route,
                visited_hash.clone(),
                path.clone(),
                visited_twice,
            )
            .0,
        );
    }
    (paths, visited_twice)
}

#[cfg(test)]
mod tests {
    use std::{collections::HashMap, fs};

    use crate::{connect_caves, read_input, traverse_paths};

    #[test]
    fn test_input() {
        let expected = vec![
            vec!["start", "A"],
            vec!["start", "b"],
            vec!["A", "c"],
            vec!["A", "b"],
            vec!["b", "d"],
            vec!["A", "end"],
            vec!["b", "end"],
        ];
        let output = read_input("test_input_small.txt");
        assert_eq!(expected, output);
    }

    #[test]
    fn get_hash() {
        let output = read_input("test_input_small.txt");
        let hash = connect_caves(output);
        let expected: HashMap<String, Vec<String>> = HashMap::from([
            (String::from("c"), vec![String::from("A")]),
            (
                String::from("start"),
                vec![String::from("A"), String::from("b")],
            ),
            (
                String::from("end"),
                vec![String::from("A"), String::from("b")],
            ),
            (
                String::from("A"),
                vec![
                    String::from("start"),
                    String::from("c"),
                    String::from("b"),
                    String::from("end"),
                ],
            ),
            (
                String::from("b"),
                vec![
                    String::from("start"),
                    String::from("A"),
                    String::from("d"),
                    String::from("end"),
                ],
            ),
            (String::from("d"), vec![String::from("b")]),
        ]);

        assert_eq!(expected, hash);
    }

    #[test]
    fn get_routes() {
        let connections = read_input("./test_input_small.txt");
        let mut cave_hash = connect_caves(connections);
        let (paths, _) = traverse_paths(
            &mut cave_hash,
            "start",
            HashMap::new(),
            vec!["start".to_string()],
            false,
        );
        let expected = vec![
            vec![
                "start".to_string(),
                "A".to_string(),
                "c".to_string(),
                "A".to_string(),
                "b".to_string(),
                "A".to_string(),
                "end".to_string(),
            ],
            vec![
                "start".to_string(),
                "A".to_string(),
                "c".to_string(),
                "A".to_string(),
                "b".to_string(),
                "end".to_string(),
            ],
            vec![
                "start".to_string(),
                "A".to_string(),
                "c".to_string(),
                "A".to_string(),
                "end".to_string(),
            ],
            vec![
                "start".to_string(),
                "A".to_string(),
                "b".to_string(),
                "A".to_string(),
                "c".to_string(),
                "A".to_string(),
                "end".to_string(),
            ],
            vec![
                "start".to_string(),
                "A".to_string(),
                "b".to_string(),
                "A".to_string(),
                "end".to_string(),
            ],
            vec![
                "start".to_string(),
                "A".to_string(),
                "b".to_string(),
                "end".to_string(),
            ],
            vec!["start".to_string(), "A".to_string(), "end".to_string()],
            vec![
                "start".to_string(),
                "b".to_string(),
                "A".to_string(),
                "c".to_string(),
                "A".to_string(),
                "end".to_string(),
            ],
            vec![
                "start".to_string(),
                "b".to_string(),
                "A".to_string(),
                "end".to_string(),
            ],
            vec!["start".to_string(), "b".to_string(), "end".to_string()],
        ];

        assert!(expected
            .into_iter()
            .all(|expected_line| paths.contains(&expected_line)));
    }

    #[test]
    fn get_length() {
        let connections = read_input("./test_input.txt");
        let mut cave_hash = connect_caves(connections);
        let (paths, _) = traverse_paths(
            &mut cave_hash,
            "start",
            HashMap::new(),
            vec!["start".to_string()],
            false,
        );
        println!("{:?}", paths);
        assert_eq!(paths.len(), 103);
    }

    #[test]
    fn get_length_larger() {
        let connections = read_input("./test_input_large.txt");
        let mut cave_hash = connect_caves(connections);
        let (paths, _) = traverse_paths(
            &mut cave_hash,
            "start",
            HashMap::new(),
            vec!["start".to_string()],
            false,
        );
        println!("{:?}", paths);
        assert_eq!(paths.len(), 3509);
    }

    #[test]
    fn get_route_bigger() {
        let connections = read_input("./test_input.txt");
        let mut cave_hash = connect_caves(connections);
        let (paths, _) = traverse_paths(
            &mut cave_hash,
            "start",
            HashMap::new(),
            vec!["start".to_string()],
            false,
        );
        let expected = get_input("test_input_valid.txt");

        assert!(expected
            .into_iter()
            .all(|expected_line| paths.contains(&expected_line)));
    }

    fn get_input(path: &str) -> Vec<Vec<String>> {
        let input_string = fs::read_to_string(path).unwrap();
        input_string
            .lines()
            .map(|line| line.to_string().split(',').map(str::to_string).collect())
            .collect()
    }
}
