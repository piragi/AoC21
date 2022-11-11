use std::{borrow::BorrowMut, fs};

struct Player {
    pos: u32,
    score: u32,
}

struct Dice {
    sides: Vec<u32>,
    counter: u32,
}

impl Dice {
    fn new() -> Dice {
        Dice {
            sides: vec![1, 2, 3],
            counter: 0,
        }
    }
    fn roll3(&self) -> Dice {
        Dice {
            sides: self
                .sides
                .iter()
                .map(|side| if *side > 97 { side + 3 - 100 } else { side + 3 })
                .collect(),
            counter: self.counter + 3,
        }
    }
}

impl Player {
    fn play(&mut self, dice: &Dice) {
        let sides_sum = dice.sides.iter().sum::<u32>();
        self.pos += sides_sum;
        while self.pos > 10 {
            self.pos -= 10;
        }

        self.score += self.pos;
    }
}

fn main() {
    let start_time = std::time::Instant::now();

    let (player1, player2) = get_input("input.txt");
    let dice = Dice::new();
    play1000(player1, player2, dice);

    let duration = start_time.elapsed();
    println!("Duration: {:?}\n", duration);
}

fn play1000(mut player1: Player, mut player2: Player, mut dice: Dice) {
    loop {
        player1.play(&dice);
        dice = dice.roll3();
        if player1.score >= 1000 {
            println!("{}", dice.counter * player2.score);
            break;
        }
        println!("{:?}", dice.sides);

        player2.play(&dice);
        dice = dice.roll3();
        if player2.score >= 1000 {
            println!("{}", dice.counter * player1.score);
            break;
        }
        println!("{:?}", dice.sides);
    }
}

fn get_input(path: &str) -> (Player, Player) {
    let string = fs::read_to_string(path).unwrap();
    let split = string.split_once('\n').unwrap();

    (
        Player {
            pos: split.0.chars().last().unwrap().to_digit(10).unwrap(),
            score: 0,
        },
        Player {
            pos: split.1.chars().last().unwrap().to_digit(10).unwrap(),
            score: 0,
        },
    )
}
