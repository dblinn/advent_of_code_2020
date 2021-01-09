use std::error::Error;
use std::collections::{HashSet, HashMap};

const MOD: usize = 20201227;

fn part1(card_public_key: usize, door_public_key: usize) -> usize {
    let card_loop_size = find_loop_size(card_public_key);
    let door_loop_size = find_loop_size(door_public_key);

    let key_a = encrypt(card_public_key, door_loop_size);
    let key_b = encrypt(door_public_key, card_loop_size);
    assert_eq!(key_a, key_b);
    key_a
}

fn encrypt(subject_number: usize, loop_size: usize) -> usize {
    let mut key: usize = 1;
    for _ in 0 .. loop_size {
        key = (key * subject_number) % MOD;
    }

    key
}

fn find_loop_size(public_key: usize) -> usize {
    let mut key: usize = 1;
    let mut loop_index = 0;
    while key != public_key {
        key = (key * 7) % MOD;

        loop_index += 1;
    }

    loop_index
}

fn part2(input: &str) -> usize {
    0
}

const DAY: u8 = 24;

fn main() -> Result<(), Box<dyn Error>> {
    let input = advent_of_code_2020::get_puzzle_input(DAY)?;

    println!("Part1: {}", part1(16915772, 18447943));
    println!("Part2: {}", part2(&input));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(14897079, part1(5764801, 17807724));
    }

    #[test]
    fn example2() {
        assert_eq!(2208, part2(""));
    }
}