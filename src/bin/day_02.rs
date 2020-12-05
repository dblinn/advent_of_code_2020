use parse_display::{Display as PDisplay, FromStr as PFromStr};
use std::error::Error;

#[derive(PDisplay, PFromStr, Debug)]
#[display("{min}-{max} {letter}: {password}")]
struct TobogganPassword {
    min: usize,
    max: usize,
    letter: char,
    password: String,
}

fn is_valid_pass(pass: &TobogganPassword) -> bool {
    let letter_count = pass.password.chars()
        .filter(|c| *c == pass.letter)
        .count();
    letter_count >= pass.min && letter_count <= pass.max
}

fn is_valid_pass_new_algorithm(pass: & TobogganPassword) -> bool {
    let mut left_match = false;
    let mut right_match = false;
    for (i, c) in pass.password.chars().enumerate() {
        left_match |= (i + 1) == pass.min && c == pass.letter;
        right_match |= (i + 1) == pass.max && c == pass.letter;
    }
    left_match ^ right_match
}

fn parse_password_rules(input: &str) -> Result<Vec<TobogganPassword>, impl Error> {
    input
        .lines()
        .map(|line| line.trim().parse::<TobogganPassword>())
        .collect()
}

const DAY: u8 = 2;
fn main() -> Result<(), Box<dyn Error>> {
    let input = advent_of_code_2020::get_puzzle_input(DAY)?;
    let passes = parse_password_rules(&input)?;
    let first_pass_count = passes.iter()
        .filter(|pass| is_valid_pass(pass))
        .count();
    let second_pass_count = passes.iter()
        .filter(|pass| is_valid_pass_new_algorithm(pass))
        .count();

    println!("Part1: {}", first_pass_count);
    println!("Part2: {}", second_pass_count);

    Ok(())
}
