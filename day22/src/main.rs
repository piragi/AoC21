use std::fs;

struct ReactorCore {
    grid: Vec<Cuboid>,
}
impl ReactorCore {
    fn new() -> ReactorCore {
        ReactorCore { grid: Vec::new() }
    }
    fn count(&self) -> i64 {
        let mut total_area = 0;
        for cuboid in &self.grid {
            match cuboid.status {
                true => total_area += cuboid.area(),
                false => total_area -= cuboid.area(),
            }
        }
        total_area
    }
}

#[derive(PartialEq, Debug, Clone)]
struct Cuboid {
    status: bool,
    coord: CoordRange,
}
impl Cuboid {
    fn intersects(&self, other: &Cuboid) -> Option<Cuboid> {
        let x = intersect_axis(self.coord.x, other.coord.x)?;
        let y = intersect_axis(self.coord.y, other.coord.y)?;
        let z = intersect_axis(self.coord.z, other.coord.z)?;

        let status = match (self.status, other.status) {
            (false, false) => true,
            (true, true) => false,
            (false, true) => false,
            (true, false) => true,
        };

        Some(Cuboid {
            coord: CoordRange { x, y, z },
            status,
        })
    }

    fn area(&self) -> i64 {
        (i64::abs(self.coord.x.1 - self.coord.x.0) + 1)
            * (i64::abs(self.coord.y.1 - self.coord.y.0) + 1)
            * (i64::abs(self.coord.z.1 - self.coord.z.0) + 1)
    }
}

fn intersect_axis(first: (i64, i64), second: (i64, i64)) -> Option<(i64, i64)> {
    let intersection: Vec<i64> = (second.0..=second.1)
        .into_iter()
        .filter(|coord| coord >= &first.0 && coord <= &first.1)
        .collect();

    match !intersection.is_empty() {
        true => Some((intersection[0], intersection[intersection.len() - 1])),
        false => None,
    }
}

#[derive(Debug)]
struct RebootSteps {
    steps: Vec<Cuboid>,
}
impl RebootSteps {
    fn reboot(&self) -> ReactorCore {
        let mut core = ReactorCore::new();
        for step in &self.steps {
            if core.grid.is_empty() {
                core.grid.push(step.clone());
            } else {
                let mut intersection_stack = Vec::new();
                for cuboid in &core.grid {
                    match step.intersects(cuboid) {
                        Some(intersection) => intersection_stack.push(intersection),
                        None => continue,
                    }
                }
                core.grid.append(&mut intersection_stack);

                if step.status {
                    core.grid.push(step.clone());
                }
            }
        }
        core
    }
}

#[derive(PartialEq, Debug, Clone)]
struct CoordRange {
    x: (i64, i64),
    y: (i64, i64),
    z: (i64, i64),
}
impl CoordRange {
    fn new(x: (i64, i64), y: (i64, i64), z: (i64, i64)) -> CoordRange {
        CoordRange { x, y, z }
    }
}

fn main() {
    let start_time = std::time::Instant::now();

    let reboot_steps = get_input("input.txt");
    let core = reboot_steps.reboot();
    println!("count: {}", core.count());

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
