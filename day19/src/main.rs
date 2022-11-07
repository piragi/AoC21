use std::fs;

#[derive(Debug, PartialEq, Clone)]
struct Scanner {
    pos: Coord,
    beacons: Vec<Beacon>,
}

#[derive(Debug, PartialEq, Clone)]
struct Coord {
    x: i32,
    y: i32,
    z: i32,
}

#[derive(Debug, PartialEq, Clone)]
struct Beacon {
    pos: Coord,
    adj: Vec<Coord>,
}

fn main() {
    let scanners = get_input("test.txt");
    let scanners = construct_adjacency(scanners);
    println!("{:?}", scanners[1].beacons[0].adj);
    println!("{:?}", scanners[4].beacons[0].adj);
    check_adj(scanners);
}

fn check_adj(scanners: Vec<Scanner>) {
    let mut counter = 0;
    for i in 0..scanners.len() {
        for j in 0..scanners.len() {
            println!("compare {i}.scanner with {j}.scanner");

            if scanners[i] != scanners[j] {
                for i_beacon in 0..scanners[i].beacons.len() {
                    let mut beacon_overlap = 0;

                    for j_beacon in 0..scanners[j].beacons.len() {
                        let beacon = &scanners[i].beacons[i_beacon];
                        let beacon_compare = &scanners[j].beacons[j_beacon];

                        for i_adj in 0..beacon.adj.len() {
                            for j_adj in 0..beacon_compare.adj.len() {
                                let orientation = orientation(beacon.adj[i_adj].clone());
                                let compare = &beacon_compare.adj[j_adj];

                                if orientation.contains(compare) {
                                    beacon_overlap += 1;
                                }
                            }
                        }
                        if beacon_overlap >= 11 {
                            println!("point: {:?}", beacon.pos);
                            counter += 1;
                        }
                        beacon_overlap = 0;
                    }
                }
                println!("-------");
            }
        }
    }
    println!("counter: {counter}");
}

fn orientation(coord: Coord) -> Vec<Coord> {
    let mut coord = coord.clone();
    let mut orientation = Vec::new();

    for _i in 0..2 {
        orientation.append(&mut rotate_face(coord.clone()));
        coord = rotate_x(coord);
    }

    coord = flip_y(coord);

    for _i in 0..2 {
        orientation.append(&mut rotate_face(coord.clone()));
        coord = rotate_y(coord);
    }

    coord = flip_z(coord);

    for _i in 0..2 {
        orientation.append(&mut rotate_face(coord.clone()));
        coord = rotate_z(coord);
    }

    orientation
}

fn rotate_face(mut coord: Coord) -> Vec<Coord> {
    let mut orientation = Vec::new();
    for _i in 0..4 {
        coord = flip_x(coord);
        orientation.push(coord.clone());
    }
    orientation
}

fn rotate_x(coord: Coord) -> Coord {
    Coord {
        x: -coord.x,
        y: coord.y,
        z: -coord.z,
    }
}

fn rotate_y(coord: Coord) -> Coord {
    Coord {
        x: -coord.x,
        y: -coord.y,
        z: coord.z,
    }
}

fn rotate_z(coord: Coord) -> Coord {
    Coord {
        x: coord.x,
        y: -coord.y,
        z: -coord.z,
    }
}

fn flip_x(coord: Coord) -> Coord {
    Coord {
        x: coord.x,
        y: coord.z,
        z: -coord.y,
    }
}
fn flip_y(coord: Coord) -> Coord {
    Coord {
        x: -coord.z,
        y: coord.y,
        z: coord.x,
    }
}
fn flip_z(coord: Coord) -> Coord {
    Coord {
        x: -coord.y,
        y: coord.x,
        z: coord.z,
    }
}

fn construct_adjacency(mut scanners: Vec<Scanner>) -> Vec<Scanner> {
    let mut scanners_adj = Vec::new();
    for scanner in &scanners {
        let mut beacons_adj = Vec::new();
        for beacon in &scanner.beacons {
            let mut adj = Vec::new();
            for adj_beacon in &scanner.beacons {
                if beacon.pos != adj_beacon.pos {
                    adj.push(Coord {
                        x: adj_beacon.pos.x - beacon.pos.x,
                        y: adj_beacon.pos.y - beacon.pos.y,
                        z: adj_beacon.pos.z - beacon.pos.z,
                    })
                }
            }
            beacons_adj.push(adj);
        }
        scanners_adj.push(beacons_adj);
    }

    for i in 0..scanners.len() {
        for j in 0..scanners[i].beacons.len() {
            scanners[i].beacons[j].adj = scanners_adj[i][j].clone();
        }
    }

    scanners
}

fn get_input(path: &str) -> Vec<Scanner> {
    let mut scanners = Vec::new();
    let mut beacons = Vec::new();
    let input = fs::read_to_string(path).unwrap();
    for line in input.lines() {
        if line.starts_with("---") {
            beacons = Vec::new();
        } else if !line.is_empty() {
            let split = line
                .split(',')
                .map(|split| split.parse::<i32>().unwrap())
                .collect::<Vec<i32>>();
            beacons.push(Beacon {
                pos: Coord {
                    x: split[0],
                    y: split[1],
                    z: split[2],
                },
                adj: Vec::new(),
            });
        } else {
            scanners.push(Scanner {
                pos: Coord { x: 0, y: 0, z: 0 },
                beacons: beacons.clone(),
            });
        }
    }

    scanners.push(Scanner {
        pos: Coord { x: 0, y: 0, z: 0 },
        beacons: beacons.clone(),
    });

    scanners
}

#[cfg(test)]
mod tests {
    use crate::{construct_adjacency, get_input, Beacon, Coord, Scanner};

    #[test]
    fn test_input() {
        let scanners = get_input("test.txt");
        let scanner_0 = Scanner {
            pos: Coord { x: 0, y: 0, z: 0 },
            beacons: vec![
                Beacon {
                    pos: Coord { x: -1, y: -1, z: 1 },
                    adj: Vec::new(),
                },
                Beacon {
                    pos: Coord { x: -2, y: -2, z: 2 },
                    adj: Vec::new(),
                },
                Beacon {
                    pos: Coord { x: -3, y: -3, z: 3 },
                    adj: Vec::new(),
                },
                Beacon {
                    pos: Coord { x: -2, y: -3, z: 1 },
                    adj: Vec::new(),
                },
                Beacon {
                    pos: Coord { x: 5, y: 6, z: -4 },
                    adj: Vec::new(),
                },
                Beacon {
                    pos: Coord { x: 8, y: 0, z: 7 },
                    adj: Vec::new(),
                },
            ],
        };
        assert_eq!(scanners[0], scanner_0);
        assert_eq!(scanners[1], scanner_0);
    }
    #[test]
    fn test_adjacency() {
        let scanners = get_input("test.txt");
        let beacon_0 = Beacon {
            pos: Coord { x: -1, y: -1, z: 1 },
            adj: vec![
                Coord { x: -1, y: -1, z: 1 },
                Coord { x: -2, y: -2, z: 2 },
                Coord { x: -1, y: -2, z: 0 },
                Coord { x: 6, y: 7, z: -5 },
                Coord { x: 9, y: 1, z: 6 },
            ],
        };
        let scanners = construct_adjacency(scanners);
        assert_eq!(scanners[0].beacons[0], beacon_0);
    }
}
