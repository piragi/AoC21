use std::{collections::HashMap, fs};

struct Board {
    pos: HashMap<u64, HashMap<u64, u64>>,
    universes_wins: u64,
}

struct Dice {
    sides: HashMap<u64, u64>,
}

impl Dice {
    fn new() -> Dice {
        Dice {
            sides: HashMap::from([(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)]),
        }
    }
}

impl Board {
    fn play(&mut self, dice: &mut Dice) -> bool {
        let mut twenty_one = true;
        let mut new_player = self.pos.clone();

        for (pos, scores) in &self.pos {
            for (side, side_counts) in &dice.sides {
                let mut new_pos = pos + side;
                while new_pos > 10 {
                    new_pos -= 10;
                }

                let pos_entry = new_player.entry(new_pos).or_default();
                for (score, players) in scores {
                    if *players == 0 {
                        continue;
                    }
                    let calculated_score = *score + new_pos;

                    if calculated_score >= 21 {
                        self.universes_wins += side_counts * players;
                    } else {
                        twenty_one = false;
                        let players_per_score = pos_entry.entry(calculated_score).or_insert(0);
                        *players_per_score += side_counts * players;
                    }
                }
            }

            let delete_entry = new_player.entry(*pos).or_default();

            for (score, player) in scores {
                delete_entry.entry(*score).and_modify(|e| *e -= player);
            }
        }
        self.pos = new_player;
        twenty_one
    }
}

fn main() {
    let start_time = std::time::Instant::now();

    let (player1, player2) = get_input("input.txt");
    let dice = Dice::new();
    play21(player1, player2, dice);
    let duration = start_time.elapsed();
    println!("Duration: {:?}\n", duration);
}

fn play21(mut player1: Board, mut player2: Board, mut dice: Dice) {
    while !player1.play(&mut dice) || !player2.play(&mut dice) {}
    println!("{:?}", player1.universes_wins);
    println!("{:?}", player2.universes_wins);
}

fn get_input(path: &str) -> (Board, Board) {
    let string = fs::read_to_string(path).unwrap();
    let split = string.split_once('\n').unwrap();

    (
        Board {
            pos: HashMap::from([(
                split.0.chars().last().unwrap().to_digit(10).unwrap().into(),
                HashMap::from([(0, 1)]),
            )]),
            universes_wins: 0,
        },
        Board {
            pos: HashMap::from([(
                split.1.chars().last().unwrap().to_digit(10).unwrap().into(),
                HashMap::from([(0, 1)]),
            )]),
            universes_wins: 0,
        },
    )
}
