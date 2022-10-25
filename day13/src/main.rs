use std::fs;

fn main() {
    let (mut grid, folds) = read_file("input.txt");
    fold(&mut grid, folds);
}

fn fold(grid: &mut Vec<Vec<char>>, folds: Vec<Vec<String>>) {
    for fold in folds {
        match fold[0].as_str() {
            "y" => fold_up(grid, fold[1].parse::<usize>().unwrap()),
            "x" => fold_left2(grid, fold[1].parse::<usize>().unwrap()),
            _ => panic!("unexpected"),
        };
    }

    for line in grid {
        let line: String = line.iter().copied().collect();
        println!("{}", line);
    }
}

fn fold_up(grid: &mut Vec<Vec<char>>, fold: usize) -> &mut Vec<Vec<char>> {
    for i in (fold + 1..grid.len()).rev() {
        //println!("y {}, y: {}", i, i - 2 * (i - fold));

        for j in 0..grid[i].len() {
            if grid[i][j] == '█' {
                grid[i - 2 * (i - fold)][j] = '█';
            }
        }
    }

    grid.truncate(fold);
    grid
}

fn fold_up2(grid: &mut Vec<Vec<char>>, fold: usize) -> &mut Vec<Vec<char>> {
    for i in 0..fold {
        //println!("y {}, y: {}", grid.len() - 1 - i, i);

        for j in 0..grid[i].len() {
            if grid[grid.len() - 1 - i][j] == '█' {
                grid[i][j] = '█';
            }
        }
    }

    grid.truncate(fold);
    grid
}

fn fold_left(grid: &mut Vec<Vec<char>>, fold: usize) -> &mut Vec<Vec<char>> {
    for i in 0..grid.len() {
        for j in (fold + 1..grid[i].len()).rev() {
            if grid[i][j] == '█' {
                grid[i][j - 2 * (j - fold)] = '█';
            }
        }
        grid[i].truncate(fold);
    }
    grid
}

fn fold_left2(grid: &mut Vec<Vec<char>>, fold: usize) -> &mut Vec<Vec<char>> {
    for i in 0..grid.len() {
        for j in 0..fold {
            //println!("x: {}, x: {}", grid[i].len() - 1 - j, j);
            if grid[i][grid[i].len() - 1 - j] == '█' {
                grid[i][j] = '█';
            }
        }
        grid[i].truncate(fold);
    }
    grid
}

fn read_file(path: &str) -> (Vec<Vec<char>>, Vec<Vec<String>>) {
    let input = fs::read_to_string(path).expect("file in directory");
    let (mut max_x, mut max_y) = (0, 0);
    let input = input.split("\n\n").collect::<Vec<&str>>();

    let dots = input[0]
        .lines()
        .map(|line| {
            let dots = line.split(',').collect::<Vec<&str>>();
            let (x, y) = (
                dots[0].parse::<u32>().unwrap(),
                dots[1].parse::<u32>().unwrap(),
            );
            if x > max_x {
                max_x = x;
            }
            if y > max_y {
                max_y = y;
            }
            (x, y)
        })
        .collect::<Vec<(u32, u32)>>();

    println!("x: {},y: {} ", max_x, max_y);

    let mut grid = vec![vec!['.'; (max_x + 1) as usize]; (max_y + 1) as usize];
    for dot in dots {
        grid[dot.1 as usize][dot.0 as usize] = '█';
    }

    let folds = input[1]
        .lines()
        .map(|line| {
            line.trim_start_matches("fold along ")
                .split('=')
                .map(|split| split.to_string())
                .collect()
        })
        .collect::<Vec<Vec<String>>>();

    (grid, folds)
}
