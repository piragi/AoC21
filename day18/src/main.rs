use std::fs;

#[derive(PartialEq, Debug)]
enum Content {
    Number(u32),
    Pair(Option<Box<SnailPair>>),
}

#[derive(PartialEq, Debug)]
struct SnailPair {
    pub x: Content,
    pub y: Content,
}

fn main() {
    let input = get_input("test_input.txt");
    let vec = get_pairs(input);
}

fn get_pairs(input: Vec<String>) -> Vec<SnailPair> {
    let mut snail_vec = Vec::new();
    for line in input {
        snail_vec.push(find_Pair(&line[1..]).0);
    }
    snail_vec
}

fn find_Pair(input: &str) -> (SnailPair, usize) {
    let mut x = Content::Number(0);
    let mut y = Content::Number(0);
    let mut chars = input.chars();
    let mut counter = 0;
    let mut char = chars.next().unwrap();

    while char != ',' {
        if char == '[' {
            counter += 1;
            let (nested, new_counter) = find_Pair(&input[counter..]);
            x = Content::Pair(Some(Box::new(nested)));
            counter += new_counter;
            chars = input[counter + 1..].chars();
        }
        if char.is_ascii_digit() {
            x = Content::Number(char.to_digit(10).unwrap());
        }
        char = chars.next().unwrap();
        counter += 1;
    }

    char = chars.next().unwrap();
    counter += 1;

    if char == '[' {
        counter += 1;
        let (nested, new_counter) = find_Pair(&input[counter..]);
        y = Content::Pair(Some(Box::new(nested)));
        counter += new_counter;
    }
    if char.is_ascii_digit() {
        y = Content::Number(char.to_digit(10).unwrap());
    }

    (SnailPair { x, y }, counter)
}

fn get_input(path: &str) -> Vec<String> {
    fs::read_to_string(path)
        .unwrap()
        .lines()
        .map(str::to_string)
        .collect()
}

#[cfg(test)]
mod test {
    use crate::{get_input, get_pairs, Content, SnailPair};

    #[test]
    fn test_input() {
        let input_vec = get_input("test_input.txt");
        assert_eq!(7, input_vec.len());
    }
    #[test]
    fn test_output() {
        let input = get_input("test_input.txt");
        let vec = get_pairs(input);
        assert_eq!(
            vec[0],
            SnailPair {
                x: Content::Number(1),
                y: Content::Number(2),
            }
        );

        let complicated = SnailPair {
            x: Content::Pair(Some(Box::new(SnailPair {
                x: Content::Pair(Some(Box::new(SnailPair {
                    x: Content::Pair(Some(Box::new(SnailPair {
                        x: Content::Number(1),
                        y: Content::Number(3),
                    }))),
                    y: Content::Pair(Some(Box::new(SnailPair {
                        x: Content::Number(5),
                        y: Content::Number(3),
                    }))),
                }))),
                y: Content::Pair(Some(Box::new(SnailPair {
                    x: Content::Pair(Some(Box::new(SnailPair {
                        x: Content::Number(1),
                        y: Content::Number(3),
                    }))),
                    y: Content::Pair(Some(Box::new(SnailPair {
                        x: Content::Number(8),
                        y: Content::Number(7),
                    }))),
                }))),
            }))),
            y: Content::Pair(Some(Box::new(SnailPair {
                x: Content::Pair(Some(Box::new(SnailPair {
                    x: Content::Pair(Some(Box::new(SnailPair {
                        x: Content::Number(4),
                        y: Content::Number(9),
                    }))),
                    y: Content::Pair(Some(Box::new(SnailPair {
                        x: Content::Number(6),
                        y: Content::Number(9),
                    }))),
                }))),
                y: Content::Pair(Some(Box::new(SnailPair {
                    x: Content::Pair(Some(Box::new(SnailPair {
                        x: Content::Number(8),
                        y: Content::Number(2),
                    }))),
                    y: Content::Pair(Some(Box::new(SnailPair {
                        x: Content::Number(7),
                        y: Content::Number(3),
                    }))),
                }))),
            }))),
        };

        assert_eq!(vec[6], complicated);
    }
}
