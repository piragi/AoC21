use std::fs;

fn main() {
    let mut input = get_input("input.txt");
    get_biggest(&mut input);
    reduce(&mut input);
}

fn get_biggest(input: &mut Vec<Vec<String>>) {
    let mut max_mag = 0;
    for counter in 0..input.len() {
        let mut line = input[counter].clone();
        while explode(&mut line) || split(&mut line) {}

        for element in input[counter + 1..].as_mut() {
            while explode(element) || split(element) {}
            let mut constructed = construct(&element.to_vec(), &line);
            while explode(&mut constructed) || split(&mut constructed) {}

            let mag = eval_pair(&constructed[1..].to_vec()).0;
            if max_mag < mag {
                max_mag = mag
            }
            let mut constructed = construct(&line, &element.clone());
            while explode(&mut constructed) || split(&mut constructed) {}

            let mag = eval_pair(&constructed[1..].to_vec()).0;
            if max_mag < mag {
                max_mag = mag
            }
        }
    }

    println!("max_mag: {}", max_mag);
}

fn construct(first: &Vec<String>, second: &Vec<String>) -> Vec<String> {
    let mut constructed = Vec::new();

    constructed.push("[".to_string());
    constructed.append(&mut first.clone());
    constructed.push(",".to_string());
    constructed.append(&mut second.clone());
    constructed.push("]".to_string());
    constructed.to_vec()
}

fn reduce(input: &mut Vec<Vec<String>>) {
    let mut add = Vec::new();
    for line in input {
        {
            while explode(line) || split(line) {}
            match add.is_empty() {
                true => add = line.to_vec(),
                false => {
                    add.insert(0, "[".to_string());
                    add.push(",".to_string());
                    add.append(line);
                    add.push("]".to_string());
                }
            }
            while explode(&mut add) || split(&mut add) {}
        }
    }
    println!("{}", eval_pair(&add[1..].to_vec()).0);
}

fn eval_pair(add: &Vec<String>) -> (i32, usize) {
    let mut lhs: i32 = -1;
    let mut rhs: i32 = -1;
    let mut index = 0;
    let mut element = String::new();

    while index < add.len() {
        element = add[index].clone();
        if element != "[" && element != "]" && element != "," {
            match lhs.is_negative() {
                true => {
                    lhs = element.parse::<i32>().unwrap();
                    index += 1;
                }
                false => {
                    rhs = element.parse::<i32>().unwrap();
                    index += 1;
                    break;
                }
            }
        } else if element == "[" {
            match lhs.is_negative() {
                true => {
                    let (mag, counter) = eval_pair(&add[index + 1..].to_vec());
                    lhs = mag;
                    index = counter;
                }
                false => {
                    let (mag, counter) = eval_pair(&add[index + 1..].to_vec());
                    rhs = mag;
                    index += counter;
                    break;
                }
            }
        }
        index += 1;
    }
    (3 * lhs + 2 * rhs, index + 1)
}

fn split(line: &mut Vec<String>) -> bool {
    let mut index = 0;
    let mut split = Vec::new();
    for (i, element) in line.iter().enumerate() {
        if element != "[" && element != "]" && element != "," && element.len() > 1 {
            let number = element.parse::<u8>().unwrap();
            match number % 2 {
                0 => {
                    split = vec![
                        "[".to_string(),
                        (number / 2).to_string(),
                        ",".to_string(),
                        (number / 2).to_string(),
                        "]".to_string(),
                    ]
                }
                1 => {
                    split = vec![
                        "[".to_string(),
                        (number / 2).to_string(),
                        ",".to_string(),
                        (number / 2 + 1).to_string(),
                        "]".to_string(),
                    ]
                }
                _ => panic!("incorrect input"),
            }
            index = i;
            break;
        }
    }

    match split.is_empty() {
        true => false,
        false => {
            //println!("---split!---");

            line.splice(index..index + 1, split);
            true
        }
    }
}

fn explode(line: &mut Vec<String>) -> bool {
    let mut level = 0;
    let mut exploded_line = Vec::new();
    for (index, char) in line.iter().enumerate() {
        if char == "[" {
            level += 1;
        } else if char == "]" {
            level -= 1;
        }
        if level > 4 && (char != "[" && char != "]" && char != ",") {
            exploded_line = line.clone();
            for i in (0..index - 1).rev() {
                if line[i] != "[" && line[i] != "]" && line[i] != "," {
                    exploded_line[i] = (line[i].parse::<u8>().unwrap()
                        + line[index].parse::<u8>().unwrap())
                    .to_string();
                    break;
                }
            }
            for i in index + 3..line.len() {
                if line[i] != "[" && line[i] != "]" && line[i] != "," {
                    exploded_line[i] = (line[i].parse::<u8>().unwrap()
                        + line[index + 2].parse::<u8>().unwrap())
                    .to_string();
                    break;
                }
            }
            exploded_line.insert(index + 4, String::from("0"));
            exploded_line.drain(index - 1..index + 4);

            break;
        }
    }

    match exploded_line.is_empty() {
        true => false,
        false => {
            //println!("-----explode!-----");
            line.clone_from(&exploded_line);
            true
        }
    }
}

fn get_input(path: &str) -> Vec<Vec<String>> {
    fs::read_to_string(path)
        .unwrap()
        .lines()
        .map(|line| {
            line.chars()
                .map(|char| char.to_string())
                .collect::<Vec<String>>()
        })
        .collect::<Vec<Vec<String>>>()
}

#[cfg(test)]
mod tests {
    use crate::{get_input, reduce};

    #[test]
    fn test_len() {
        let mut input = get_input("test_input.txt");
        reduce(&mut input);
        assert_eq!(33, input[0].len());
        assert_eq!(17, input[1].len());
        assert_eq!(17, input[2].len());
        assert_eq!(17, input[3].len());
    }
}
