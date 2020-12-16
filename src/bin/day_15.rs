use std::error::Error;
use std::collections::HashMap;
use std::collections::hash_map::Entry::{Occupied, Vacant};

fn compute_guess_number(initial_guesses: &Vec<usize>, turn: usize) -> usize {
    let mut guess_map: HashMap<usize, usize> = HashMap::with_capacity(turn);
    for (i, n) in initial_guesses.iter().enumerate() {
        guess_map.insert(*n, i);
    }

    let mut to_speak = 0;
    let mut last_spoken = 0;
    for i in initial_guesses.len() .. turn {
        last_spoken = to_speak;
        match guess_map.entry(to_speak) {
            Occupied(entry) => {
                to_speak = i - *entry.get();
                *entry.into_mut() = i;
            },
            Vacant(entry) => {
                to_speak = 0;
                entry.insert(i);
            }
        }
    }

    last_spoken
}

fn part1(input: &str) -> usize {
    compute_guess_number(&input.split(",")
                             .map(|val| val.parse::<usize>().expect("a number"))
                             .collect::<Vec<usize>>(), 2020)
}

fn part2(input: &str) -> usize {
    compute_guess_number(&input.split(",")
        .map(|val| val.parse::<usize>().expect("a number"))
        .collect::<Vec<usize>>(), 30_000_000)
}

const DAY: u8 = 15;

fn main() -> Result<(), Box<dyn Error>> {
    let input = advent_of_code_2020::get_puzzle_input(DAY)?;

    println!("Part1: {}", part1(&input));
    println!("Part2: {}", part2(&input));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "0,3,6";

    #[test]
    fn example1() {
        assert_eq!(436, part1(INPUT));
        assert_eq!(1, part1("1,3,2"));
        assert_eq!(10, part1("2,1,3"));
        assert_eq!(27, part1("1,2,3"));
        assert_eq!(78, part1("2,3,1"));
        assert_eq!(438, part1("3,2,1"));
        assert_eq!(1836, part1("3,1,2"));
    }

    #[test]
    fn example2() {
        assert_eq!(175594, part2(INPUT));
        assert_eq!(2578, part2("1,3,2"));
        assert_eq!(3544142, part2("2,1,3"));
        assert_eq!(261214, part2("1,2,3"));
        assert_eq!(6895259, part2("2,3,1"));
        assert_eq!(18, part2("3,2,1"));
        assert_eq!(362, part2("3,1,2"));
    }
}