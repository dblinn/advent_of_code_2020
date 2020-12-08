use std::error::Error;
use std::collections::{HashSet, HashMap};

fn distinct_answers(group: &str) -> usize {
    let h: HashSet<char> = group.chars()
        .filter(|c| c.is_ascii_alphabetic())
        .collect();
    h.len()
}

fn all_answered(group: &str) -> usize {
    let num_group_members = group.lines().count();
    let counts: HashMap<char, usize> = group.chars()
        .filter(|c| c.is_ascii_alphabetic())
        .fold(HashMap::new(), |mut acc, c| {
            *acc.entry(c).or_insert(0) += 1;
            acc
        });
    counts.values().filter(|count| **count == num_group_members).count()
}

fn  part1(input: &str) -> usize {
    input.split_terminator("\n\n")
        .map(|group| distinct_answers(group))
        .sum()
}

fn part2(input: &str) -> usize {
    input.split_terminator("\n\n")
        .map(|group| all_answered(group))
        .sum()
}

const DAY: u8 = 6;

fn main() -> Result<(), Box<dyn Error>> {
    let input = advent_of_code_2020::get_puzzle_input(DAY)?;

    println!("Part1: {}", part1(&input));
    println!("Part2: {}", part2(&input));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "abc

a
b
c

ab
ac

a
a
a
a

b";

    #[test]
    fn example1() {
        assert_eq!(part1(INPUT), 11);
    }

    #[test]
    fn example2() {
        assert_eq!(part2(INPUT), 6);
    }
}