use std::{fs, path::Path};

const Q1_PATH: &str = "src/q1.txt";

const SCORE_ROCK: u32 = 1;
const SCORE_PAPER: u32 = 2;
const SCORE_SCISSOR: u32 = 3;
const SCORE_LOSS: u32 = 0;
const SCORE_DRAW: u32 = 3;
const SCORE_WIN: u32 = 6;

fn format_input_row(row: &str) -> Option<(char, char)> {
    let mut chars = row.chars();
    match (chars.next(), chars.nth(1)) {
        (Some(a), Some(b)) => Some((a, b)),
        _ => None,
    }
}

// next time use modular arithmetic
fn calculate_score_1(round: Option<(char, char)>) -> u32 {
    match round {
        None => 0,
        Some(r) => match r {
            (a, 'X') => {
                SCORE_ROCK
                    + match a {
                        'A' => SCORE_DRAW,
                        'B' => SCORE_LOSS,
                        'C' => SCORE_WIN,
                        _ => 0,
                    }
            }
            (a, 'Y') => {
                SCORE_PAPER
                    + match a {
                        'A' => SCORE_WIN,
                        'B' => SCORE_DRAW,
                        'C' => SCORE_LOSS,
                        _ => 0,
                    }
            }
            (a, 'Z') => {
                SCORE_SCISSOR
                    + match a {
                        'A' => SCORE_LOSS,
                        'B' => SCORE_WIN,
                        'C' => SCORE_DRAW,
                        _ => 0,
                    }
            }
            _ => 0,
        },
    }
}

fn calculate_score_2(round: Option<(char, char)>) -> u32 {
    match round {
        None => 0,
        Some(r) => match r {
            (a, 'X') => {
                SCORE_LOSS
                    + match a {
                        'A' => SCORE_SCISSOR,
                        'B' => SCORE_ROCK,
                        'C' => SCORE_PAPER,
                        _ => 0,
                    }
            }
            (a, 'Y') => {
                SCORE_DRAW
                    + match a {
                        'A' => SCORE_ROCK,
                        'B' => SCORE_PAPER,
                        'C' => SCORE_SCISSOR,
                        _ => 0,
                    }
            }
            (a, 'Z') => {
                SCORE_WIN
                    + match a {
                        'A' => SCORE_PAPER,
                        'B' => SCORE_SCISSOR,
                        'C' => SCORE_ROCK,
                        _ => 0,
                    }
            }
            _ => 0,
        },
    }
}
fn main() {
    let path = Path::new(Q1_PATH);
    let contents = fs::read_to_string(path).expect("Error reading file");
    let res = contents
        .split('\n')
        .map(format_input_row)
        .fold(0, |accum, c| accum + calculate_score_2(c));
    println!("{res}");
}
