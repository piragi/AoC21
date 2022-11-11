use std::{borrow::BorrowMut, collections::HashMap, fs, hash::Hash};

struct Player {
    pos: HashMap<u64, HashMap<u64, u64>>,
}

struct Dice {
    sides: HashMap<u64, u64>,
    counter: u64,
}

impl Dice {
    fn new() -> Dice {
        Dice {
            sides: HashMap::from([(3, 1), (4, 3), (5, 5), (6, 7), (7, 5), (8, 4), (9, 1)]),
            counter: 0,
        }
    }
}

impl Player {
    fn play(&mut self, dice: &mut Dice) -> bool {
        let mut twenty_one = false;
        let mut new_player = self.pos.clone();

        dice.counter += 1;

        for (pos, scores) in &self.pos {
            for (side, side_counts) in &dice.sides {
                let mut new_pos = pos + side;
                while new_pos > 10 {
                    new_pos -= 10;
                }

                let pos_entry = new_player.entry(new_pos).or_default();
                for (score, players) in scores {
                    let calculated_score = *score + new_pos;

                    if calculated_score >= 21 {
                        twenty_one = true;
                    }

                    let players_per_score = pos_entry.entry(calculated_score).or_insert(0);
                    *players_per_score += side_counts * players;
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

    let (player1, player2) = get_input("test.txt");
    let dice = Dice::new();
    play21(player1, player2, dice);
    let duration = start_time.elapsed();
    println!("Duration: {:?}\n", duration);
}

fn play21(mut player1: Player, mut player2: Player, mut dice: Dice) {
    while !player1.play(&mut dice) || !player2.play(&mut dice) {}

    println!("{:?}", player1.pos);
    let score_player1 = calculate_universes(player1);
    println!("{:?}", score_player1);
    println!("{:?}", dice.counter);

    let score_player2 = calculate_universes(player2);
}

fn calculate_universes(player: Player) -> u64 {
    let mut counter = 0;
    for scores in player.pos.values() {
        for (score, players) in scores {
            if *score >= 21 {
                counter += players;
            }
        }
    }
    counter
}

fn get_input(path: &str) -> (Player, Player) {
    let string = fs::read_to_string(path).unwrap();
    let split = string.split_once('\n').unwrap();

    (
        Player {
            pos: HashMap::from([(
                split.0.chars().last().unwrap().to_digit(10).unwrap().into(),
                HashMap::from([(1, 1)]),
            )]),
        },
        Player {
            pos: HashMap::from([(
                split.1.chars().last().unwrap().to_digit(10).unwrap().into(),
                HashMap::from([(1, 1)]),
            )]),
        },
    )
}
