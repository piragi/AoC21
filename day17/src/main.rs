use std::cmp::Ordering;

struct Probe {
    pos: Coord,
    vel: Coord,
}

struct Target {
    x: (i32, i32),
    y: (i32, i32),
}

#[derive(PartialEq, Debug)]
struct Coord {
    x: i32,
    y: i32,
}

fn main() {
    let target = get_input("target area: x=102..157, y=-146..-90");
    let hits = get_hits(target);
    println!("{}", hits.len());
}

fn get_input(input: &str) -> Target {
    let input = input.trim_start_matches("target area: x=");
    let input = input.split(", y=").collect::<Vec<&str>>();
    let mut xrange = input[0]
        .split("..")
        .map(|range| range.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
    let mut yrange = input[1]
        .split("..")
        .map(|range| range.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    yrange.sort();
    xrange.sort();

    Target {
        x: (xrange[0], xrange[1]),
        y: (yrange[0], yrange[1]),
    }
}

fn get_max(ranges: Target) -> i32 {
    let counter = gauss(ranges.x.0);

    println!("{counter}");

    let mut max_y = 0;

    for y in 0..ranges.y.0 * ranges.y.0 {
        let (lands, curr_y) = probe_evolution(&ranges, (counter, y));
        if !lands {
            continue;
        }
        if curr_y > max_y {
            max_y = curr_y
        }
    }

    max_y
}

fn gauss(range: i32) -> i32 {
    let mut counter = 0;
    let mut x = 0;
    while x < range {
        counter += 1;
        x = (counter * (counter + 1)) / 2;
    }

    counter
}

fn get_hits(ranges: Target) -> Vec<Coord> {
    let x_min = gauss(ranges.x.0);
    let mut hits = Vec::new();

    for x in x_min..x_min * x_min {
        for y in ranges.y.0..ranges.y.0 * ranges.y.0 {
            let (lands, _) = probe_evolution(&ranges, (x, y));
            if lands {
                hits.push(Coord { x, y });
            }
        }
    }

    hits
}

fn probe_evolution(ranges: &Target, probe: (i32, i32)) -> (bool, i32) {
    let mut probe = Probe {
        pos: Coord { x: 0, y: 0 },
        vel: Coord {
            x: probe.0,
            y: probe.1,
        },
    };
    let mut max_y = 0;

    while probe.pos.x < ranges.x.1 && probe.pos.y > ranges.y.0 {
        probe.pos.x += probe.vel.x;
        probe.pos.y += probe.vel.y;

        if probe.pos.y > max_y {
            max_y = probe.pos.y;
        }

        if (probe.pos.x >= ranges.x.0 && probe.pos.x <= ranges.x.1)
            && (probe.pos.y >= ranges.y.0 && probe.pos.y <= ranges.y.1)
        {
            return (true, max_y);
        }

        probe.vel.y -= 1;
        match probe.vel.x.cmp(&0) {
            Ordering::Less => probe.vel.x += 1,
            Ordering::Greater => probe.vel.x -= 1,
            _ => continue,
        }
    }
    (false, 0)
}

#[cfg(test)]
mod tests {
    use crate::{get_hits, get_input, get_max, probe_evolution, Coord};

    #[test]
    fn test_input() {
        let target = get_input("target area: x=20..30, y=-10..-5");
        assert_eq!(
            ((20, 30), (-10, -5)),
            ((target.x.0, target.x.1), (target.y.0, target.y.1))
        );
    }

    #[test]
    fn test_evolution() {
        let target = get_input("target area: x=20..30, y=-10..-5");
        let coord = probe_evolution(&target, (7, 2));

        println!("-----");

        let target = get_input("target area: x=20..30, y=-10..-5");
        let coord = probe_evolution(&target, (6, 3));

        println!("-----");

        let target = get_input("target area: x=20..30, y=-10..-5");
        let coord = probe_evolution(&target, (9, 0));
        println!("-----");

        let target = get_input("target area: x=20..30, y=-10..-5");
        let coord = probe_evolution(&target, (17, -4));
    }

    #[test]
    fn test_max() {
        let target = get_input("target area: x=20..30, y=-10..-5");
        let max = get_max(target);
        assert_eq!(45, max);
    }

    #[test]
    fn test_hits() {
        let target = get_input("target area: x=20..30, y=-10..-5");
        let hits = get_hits(target);
        println!("{:?}", hits);
        assert_eq!(112, hits.len());
    }
}
