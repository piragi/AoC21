use std::{collections::HashMap, fs, hash::Hash};

const INIT_RANGE: std::ops::RangeInclusive<i64> = -50..=50;

struct ReactorCore {
    grid: HashMap<Coord, bool>,
}
impl ReactorCore {
    fn new() -> ReactorCore {
        ReactorCore {
            grid: HashMap::new(),
        }
    }

    fn count(&self) -> usize {
        self.grid
            .values()
            .into_iter()
            .filter(|value| *value == &true)
            .count()
    }
    fn turn_off(self) -> ReactorCore {
        ReactorCore {
            grid: self
                .grid
                .into_iter()
                .map(|(coord, _)| (coord, false))
                .collect::<HashMap<Coord, bool>>(),
        }
    }

    fn traverse_step(&mut self, cuboid: &Cuboid) {
        for y in cuboid.coord.y.0..=cuboid.coord.y.1 {
            for z in cuboid.coord.z.0..=cuboid.coord.z.1 {
                for x in cuboid.coord.x.0..=cuboid.coord.x.1 {
                    let status = self
                        .grid
                        .entry(Coord::new(x, y, z))
                        .or_insert(cuboid.status);
                    *status = cuboid.status;
                }
            }
        }
    }
}

#[derive(Debug)]
struct Cuboid {
    status: bool,
    coord: CoordRange,
}

#[derive(Debug)]
struct RebootSteps {
    steps: Vec<Cuboid>,
}
impl RebootSteps {
    fn initialization(&self) -> ReactorCore {
        let mut core = ReactorCore::new();
        for cuboid in &self.steps {
            if cuboid.coord.inside_init_range() {
                core.traverse_step(cuboid);
            }
        }
        core
    }

    fn reboot(&self, core: &mut ReactorCore) {
        for cuboid in &self.steps {
            core.traverse_step(cuboid);
        }
    }
}

#[derive(Debug)]
struct CoordRange {
    x: (i64, i64),
    y: (i64, i64),
    z: (i64, i64),
}
impl CoordRange {
    fn new(x: (i64, i64), y: (i64, i64), z: (i64, i64)) -> CoordRange {
        CoordRange { x, y, z }
    }

    fn inside_init_range(&self) -> bool {
        INIT_RANGE.contains(&self.x.0)
            && INIT_RANGE.contains(&self.x.1)
            && INIT_RANGE.contains(&self.y.0)
            && INIT_RANGE.contains(&self.y.1)
            && INIT_RANGE.contains(&self.z.0)
            && INIT_RANGE.contains(&self.z.1)
    }
}

#[derive(Eq, Hash, PartialEq, Debug)]
struct Coord {
    x: i64,
    y: i64,
    z: i64,
}
impl Coord {
    fn new(x: i64, y: i64, z: i64) -> Coord {
        Coord { x, y, z }
    }
}

fn main() {
    let start_time = std::time::Instant::now();

    let reboot_steps = get_input("test.txt");
    let core = reboot_steps.initialization();
    let mut reboot_core = core.turn_off();
    reboot_steps.reboot(&mut reboot_core);
    println!("{}", reboot_core.count());

    let duration = start_time.elapsed();
    println!("Duration: {:?}\n", duration);
}

fn get_input(path: &str) -> RebootSteps {
    let input = fs::read_to_string(path).unwrap();
    let steps = input
        .lines()
        .map(|line| {
            let split = line.split_once(' ').unwrap();
            let status = matches!(split.0, "on");

            let coord: Vec<(i64, i64)> = split
                .1
                .split(',')
                .map(|coord| {
                    let range = coord[2..].split_once("..").unwrap();
                    (
                        range.0.parse::<i64>().unwrap(),
                        range.1.parse::<i64>().unwrap(),
                    )
                })
                .collect();

            Cuboid {
                status,
                coord: CoordRange::new(coord[0], coord[1], coord[2]),
            }
        })
        .collect::<Vec<Cuboid>>();

    RebootSteps { steps }
}
