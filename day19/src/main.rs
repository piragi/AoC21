use core::fmt;
use std::{
    borrow::Borrow,
    collections::{HashMap, HashSet},
    fs,
};

#[derive(Debug, PartialEq, Clone)]
struct Scanner {
    pos: Coord,
    beacons: Vec<Beacon>,
    id: usize,
}

struct Scanners {
    scanner: Vec<Scanner>,
}

#[derive(Eq, Hash, Debug, PartialEq, Clone)]
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

impl Scanners {
    fn compare_scanners1(&mut self, start_id: usize, done: &mut HashSet<usize>) {
        done.insert(start_id);

        for i in 0..self.scanner.len() {
            if done.contains(&i) {
                continue;
            }
            let scanner = &self.scanner[start_id];
            match scanner.overlap_rotate(&self.scanner[i]) {
                Some(x) => {
                    self.scanner[i] = x;
                    self.compare_scanners1(self.scanner[i].id, done)
                }
                None => continue,
            }
        }
    }

    fn compare_scanners(&mut self) {
        let mut done = HashSet::new();
        self.compare_scanners1(0, &mut done);
    }

    fn count_beacons(&self) -> usize {
        let mut beacons = HashSet::new();
        for s in &self.scanner {
            let scanner = s.borrow();
            for beacon in &scanner.beacons {
                beacons.insert(beacon.pos.add(&scanner.pos));
            }
        }
        beacons.len()
    }

    fn manhatten_distances(&mut self) {
        let mut vec = Vec::new();
        for scanner in &self.scanner {
            for scanner_compare in &self.scanner {
                let manhatten = scanner.pos.manhattan(&scanner_compare.pos);
                if vec.is_empty() || (*vec.last().unwrap() < manhatten) {
                    vec.push(manhatten);
                }
            }
        }
        println!("{}", vec.last().unwrap());
    }
}

impl Scanner {
    fn overlap_rotate(&self, other: &Scanner) -> Option<Scanner> {
        let mut scanner2 = other.clone();
        let mut m = HashMap::<Coord, (usize, u32)>::new();

        for beacon in &self.beacons {
            for beacon_compared in &other.beacons {
                for rotation in 0..24 {
                    let rotated = &beacon_compared.pos.rotate(rotation);
                    let dist = beacon.pos.subtract(rotated);

                    m.entry(dist)
                        .and_modify(|e| e.1 += 1)
                        .or_insert((rotation.into(), 1u32));
                }
            }
        }

        for (pos, (rotation, beacon_overlap)) in m {
            if beacon_overlap >= 12 {
                for i in 0..scanner2.beacons.len() {
                    scanner2.beacons[i].pos =
                        scanner2.beacons[i].pos.rotate(rotation.try_into().unwrap());
                }
                scanner2.pos = self.pos.add(&pos);
                return Some(scanner2);
            }
        }
        None
    }
}

impl Coord {
    fn rotate(&self, rotating: u8) -> Coord {
        let (x, y, z) = (self.x, self.y, self.z);

        let (x, y, z) = match rotating {
            0 => (x, y, z),
            1 => (-y, x, z),
            2 => (-x, -y, z),
            3 => (y, -x, z),
            4 => (x, z, -y),
            5 => (-z, x, -y),
            6 => (-x, -z, -y),
            7 => (z, -x, -y),
            8 => (y, x, -z),
            9 => (-x, y, -z),
            10 => (-y, -x, -z),
            11 => (x, -y, -z),
            12 => (z, x, y),
            13 => (-x, z, y),
            14 => (-z, -x, y),
            15 => (x, -z, y),
            16 => (z, y, -x),
            17 => (-y, z, -x),
            18 => (-z, -y, -x),
            19 => (y, -z, -x),
            20 => (y, z, x),
            21 => (-z, y, x),
            22 => (-y, -z, x),
            23 => (z, -y, x),
            _ => unreachable!(),
        };
        Coord { x, y, z }
    }

    fn manhattan(&self, other: &Coord) -> u32 {
        let d = (other.x - self.x).abs() + (other.y - self.y).abs() + (other.z - self.z).abs();
        d as u32
    }

    fn subtract(&self, other: &Coord) -> Coord {
        Coord {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }

    fn add(&self, other: &Coord) -> Coord {
        Coord {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}
impl fmt::Display for Coord {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

fn main() {
    let mut scanners = Scanners {
        scanner: get_input("test.txt"),
    };

    scanners.compare_scanners();
    println!("number of beacons: {}", scanners.count_beacons());
    scanners.manhatten_distances();
}

fn get_input(path: &str) -> Vec<Scanner> {
    let mut scanners = Vec::new();
    let mut beacons = Vec::new();
    let input = fs::read_to_string(path).unwrap();
    let mut count = 0;
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
                id: count,
            });
            count += 1;
        }
    }

    scanners.push(Scanner {
        pos: Coord { x: 0, y: 0, z: 0 },
        beacons: beacons.clone(),
        id: count,
    });

    scanners
}
