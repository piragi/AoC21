use std::{collections::HashMap, fs};

const WIN: u64 = 21;

#[derive(Clone, Debug)]
struct Player {
    pos: u64,
    score: u64,
    count: u64,
}

#[derive(Clone, Debug)]
struct State {
    player: Vec<Player>,
    universes: u64,
}
#[derive(Debug)]
struct Game {
    states: Vec<State>,
    score: Vec<u64>,
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

impl Game {
    fn play(&mut self, dice: Dice, index_player: usize) {
        let mut evolved_states = Game {
            states: Vec::new(),
            score: vec![0, 0],
        };

        for state in &self.states {
            for side in &dice.sides {
                let pos = (((state.player[index_player].pos + side.0) - 1) % 10) + 1;
                let score = pos + state.player[index_player].score;

                if score >= WIN {
                    self.score[index_player] +=
                        side.1 * state.universes * state.player[index_player].count;
                } else {
                    let mut evolved = state.clone();

                    evolved.player[index_player].pos =
                        (((state.player[index_player].pos + side.0) - 1) % 10) + 1;
                    evolved.player[index_player].score = score;
                    evolved.player[index_player].count *= evolved.universes * *side.1;
                    evolved.universes = *side.1;

                    evolved_states.states.push(evolved);
                }
            }
        }
        if !evolved_states.states.is_empty() {
            match index_player {
                0 => evolved_states.play(dice, 1),
                _ => evolved_states.play(dice, 0),
            }
        }

        for i in 0..=1 {
            self.score[i] += evolved_states.score[i];
        }
    }
}

fn main() {
    let start_time = std::time::Instant::now();

    let (player1, player2) = get_input("input.txt");
    let dice = Dice::new();
    let state = State {
        player: vec![player1, player2],
        universes: 1,
    };
    let mut game = Game {
        states: vec![state],
        score: vec![0, 0],
    };
    game.play(dice, 0);
    println!("{:?}", game.score[0].max(game.score[1]));

    let duration = start_time.elapsed();
    println!("Duration: {:?}\n", duration);
}

fn get_input(path: &str) -> (Player, Player) {
    let string = fs::read_to_string(path).unwrap();
    let split = string.split_once('\n').unwrap();

    (
        Player {
            pos: split.0.chars().last().unwrap().to_digit(10).unwrap().into(),
            score: 0,
            count: 1,
        },
        Player {
            pos: split.1.chars().last().unwrap().to_digit(10).unwrap().into(),
            score: 0,
            count: 1,
        },
    )
}
