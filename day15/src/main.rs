use std::{fs, time::Instant};

const GRID: usize = 100;
const CAVE: usize = 4;

#[derive(Debug)]
struct Vertice {
    pos: (usize, usize),
    weight: u32,
}

fn main() {
    let start_time = Instant::now();
    let mut graph = read_file("input.txt");
    expand_input(&mut graph);
    dijkstra(graph);
    let duration = start_time.elapsed();
    println!("Duration: {:?}\n", duration);
}

fn read_file(path: &str) -> Vec<Vec<(u32, bool)>> {
    fs::read_to_string(path)
        .unwrap()
        .lines()
        .map(|line| {
            line.chars()
                .map(|char| (char.to_digit(10).unwrap(), false))
                .collect::<Vec<(u32, bool)>>()
        })
        .collect::<Vec<Vec<(u32, bool)>>>()
}

fn expand_input(graph: &mut Vec<Vec<(u32, bool)>>) {
    let mut new_graph = Vec::new();

    for i in 1..10 as u32 {
        new_graph.push(
            graph
                .iter()
                .map(|vec| {
                    vec.iter()
                        .map(|(element, _)| {
                            if *element + i > 9 {
                                (*element + i - 9, false)
                            } else {
                                (*element + i, false)
                            }
                        })
                        .collect()
                })
                .collect::<Vec<Vec<(u32, bool)>>>(),
        );
    }

    for i in 0..CAVE {
        let mut vec = new_graph[i].clone();
        graph.append(&mut vec);
        for j in i..i + CAVE {
            for line in i * GRID..i * GRID + GRID {
                let mut vec = new_graph[j][line - (i * GRID)].clone();
                graph[line].append(&mut vec);
            }
        }
    }

    for j in CAVE..CAVE + CAVE {
        for line in CAVE * GRID..4 * GRID + GRID {
            let mut vec = new_graph[j][line - (CAVE * GRID)].clone();
            graph[line].append(&mut vec);
        }
    }
}

fn dijkstra(mut graph: Vec<Vec<(u32, bool)>>) {
    let mut stack = Vec::new();
    let mut weight = 0_u32;
    let (mut i, mut j) = (0_i32, 0_i32);

    loop {
        graph[i as usize][j as usize].1 = true;

        let directions = vec![(i - 1, j), (i + 1, j), (i, j + 1), (i, j - 1)];
        let directions = directions
            .into_iter()
            .filter(|(y, x)| {
                *x >= 0 && *y >= 0 && *x < graph[i as usize].len() as i32 && *y < graph.len() as i32
            })
            .map(|(x, y)| (x as usize, y as usize))
            .collect::<Vec<(usize, usize)>>();

        for (y, x) in directions {
            if !graph[y][x].1 {
                let v = Vertice {
                    pos: (y, x),
                    weight: weight + graph[y][x].0,
                };

                match stack.iter().position(|sv: &Vertice| sv.pos == v.pos) {
                    Some(pos) => {
                        if stack[pos].weight > v.weight {
                            stack.remove(pos);
                            stack.push(v);
                        }
                    }
                    None => stack.push(v),
                }
            }
        }
        stack.sort_by_key(|k| k.weight);
        let v = stack.remove(0);
        weight = v.weight;
        let (y, x) = v.pos;
        if graph[y][x].1 {
            continue;
        }

        if y == graph.len() - 1 && x == graph[graph.len() - 1].len() - 1 {
            break;
        }

        i = y as i32;
        j = x as i32;
    }
    println!("weight: {weight}");
}
