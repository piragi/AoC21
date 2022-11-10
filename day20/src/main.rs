use core::fmt;
use std::{fs, time::Instant};

struct Image {
    image: Vec<Vec<char>>,
    surrounded: char,
}

impl Image {
    fn count_lit(&self) -> usize {
        self.image
            .iter()
            .flatten()
            .filter(|&element| *element == '#')
            .count()
    }
    fn calculate_output(&mut self, algo: &str) -> Image {
        let neighbours: Vec<(i32, i32)> = vec![
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 0),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ];
        let mut output_image = Vec::new();
        self.extend_image(self.surrounded);

        for line_pos in 0..self.image.len() {
            let mut output_line = Vec::new();
            for element_pos in 0..self.image[line_pos].len() {
                let code: String = neighbours
                    .iter()
                    .map(|(line, element)| {
                        if line_pos as i32 + line < 0
                            || element_pos as i32 + element < 0
                            || line_pos as i32 + line >= self.image.len() as i32
                            || element_pos as i32 + element >= self.image[line_pos].len() as i32
                        {
                            return convert_pixel(self.surrounded);
                        }
                        let line = (line_pos as i32 + *line) as usize;
                        let element = (element_pos as i32 + *element) as usize;

                        convert_pixel(self.image[line][element])
                    })
                    .collect();
                let code = usize::from_str_radix(&code, 2).unwrap();
                output_line.push(algo.chars().nth(code).unwrap());
            }
            output_image.push(output_line);
        }

        Image {
            image: output_image,
            surrounded: match self.surrounded {
                '#' => algo.chars().last().unwrap(),
                '.' => algo.chars().next().unwrap(),
                _ => panic!("illegal character"),
            },
        }
    }

    fn extend_image(&mut self, pattern: char) {
        for i in 0..self.image.len() {
            self.image[i].insert(0, pattern);
            self.image[i].push(pattern);
        }

        let empty_line = vec![pattern; self.image[0].len()];

        self.image.insert(0, empty_line.clone());
        self.image.push(empty_line);
    }
}

impl fmt::Display for Image {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut output = String::new();
        for line in &self.image {
            for element in line {
                output.push(*element);
            }
            output.push('\n');
        }
        write!(f, "{}", output)
    }
}

fn main() {
    let start_time = Instant::now();
    let (algo, mut input) = get_input("input.txt");
    for _i in 0..50 {
        input = input.calculate_output(&algo);
    }
    println!("{}", input.count_lit());
    let duration = start_time.elapsed();
    println!("Duration: {:?}\n", duration);
}

fn convert_pixel(char: char) -> char {
    match char {
        '.' => '0',
        '#' => '1',
        _ => panic!("illegal character"),
    }
}

fn get_input(path: &str) -> (String, Image) {
    let binding = fs::read_to_string(path).unwrap();
    let input_str = binding.split_once("\n\n").unwrap();

    (
        input_str.0.lines().collect(),
        Image {
            image: input_str
                .1
                .lines()
                .map(|line| line.chars().collect())
                .collect(),
            surrounded: '.',
        },
    )
}
