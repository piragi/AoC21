use std::collections::HashMap;
use std::fs;
use std::hash::Hash;

fn main() {
    let input_string = fs::read_to_string("./input.txt").expect("input is in directory");

    let (mut all_boards, guesses) = read_file(&input_string);

    check_guess(&mut all_boards, guesses)
}

fn calc_result(board: &Vec<Vec<&str>>, guess: u32) -> u32 {
    let mut result = 0 as u32;

    for row in board {
        for element in row {
            if *element != "x" {
                result += element.parse::<u32>().unwrap();
            }
        }
    }
    result * guess
}

fn read_file(input_string: &String) -> (Vec<Vec<Vec<&str>>>, Vec<&str>) {
    let mut guesses = Vec::new();
    let mut board = Vec::new();
    let mut all_boards: Vec<Vec<Vec<&str>>> = Vec::new();

    let mut counter = 0;

    for lines in input_string.lines() {
        if lines.len() > 14 {
            guesses = lines.split(',').collect();
            continue;
        } else if lines.is_empty() {
            continue;
        }

        board.push(lines.split_ascii_whitespace().collect());
        counter += 1;

        if counter == 5 {
            all_boards.push(board);
            board = Vec::new();
            counter = 0;
        }
    }

    (all_boards, guesses)
}

fn check_guess(all_boards: &mut Vec<Vec<Vec<&str>>>, guesses: Vec<&str>) {
    let mut bingos_found = HashMap::new();
    let mut last_guess = "0";
    let mut last_board = 0;
    for guess in guesses {
        for i in 0..all_boards.len() {
            for j in 0..all_boards[i].len() {
                for k in 0..all_boards[i][j].len() {
                    if guess.eq(all_boards[i][j][k]) {
                        all_boards[i][j][k] = "x";
                        //check for bingo
                        if !bingos_found.contains_key(&i) && check_bingo(&all_boards[i]) {
                            //return Some((i, guess.parse::<u32>().unwrap()));
                            bingos_found.insert(
                                i,
                                calc_result(&all_boards[i], guess.parse::<u32>().unwrap()),
                            );
                            last_board = i;
                        }
                    }
                }
            }
        }
    }
    println!("last won board{:#?}", bingos_found.get(&last_board));
}

fn check_bingo(board: &Vec<Vec<&str>>) -> bool {
    let mut counter_row = 0;
    let mut counter_dia1 = 0;
    let mut counter_dia2 = 0;
    for i in 0..5 {
        if board[i] == ["x", "x", "x", "x", "x"] {
            return true;
        }
        if board[i][i] == "x" {
            counter_dia1 += 1;
        }
        if board[i][4 - i] == "x" {
            counter_dia2 += 1;
        }

        for j in 0..5 {
            if board[j][i] == "x" {
                counter_row += 1;
            }
        }
        if counter_row == 5 || counter_dia2 == 5 || counter_dia1 == 5 {
            return true;
        }

        counter_row = 0;
    }
    false
}
