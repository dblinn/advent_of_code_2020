use std::error::Error;
use std::collections::VecDeque;
use std::collections::HashSet;

fn part1(input: &str) -> usize {
    let mut players = input.split("\n\n");
    let mut player1 = players.next().expect("Player 1").lines()
        .skip(1)
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<VecDeque<usize>>();

    let mut player2 = players.next().expect("Player 2").lines()
        .skip(1)
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<VecDeque<usize>>();

    loop {
        if player1.is_empty() || player2.is_empty() {
            break;
        }

        let p1 = player1.pop_front().unwrap();
        let p2 = player2.pop_front().unwrap();
        if p1 > p2 {
            player1.push_back(p1);
            player1.push_back(p2);
        } else {
            player2.push_back(p2);
            player2.push_back(p1);
        }
    }

    let winner = if player1.is_empty() { player2 } else { player1 };
    winner.iter().rev().enumerate()
        .map(|(i, j)| (i + 1) * *j)
        .sum()
}

#[derive(PartialEq, Eq, Debug)]
enum Winner {
    PLAYER1,
    PLAYER2
}

fn recursive_combat(mut player1: VecDeque<usize>,
                    mut player2: VecDeque<usize>, depth: usize) -> (Winner, usize) {
    let mut p1_inputs = HashSet::new();
    let mut p2_inputs = HashSet::new();

    loop {
        if player1.is_empty() || player2.is_empty() {
            break;
        } else if p1_inputs.contains(&player1) && p2_inputs.contains(&player2) {
            return (Winner::PLAYER1, 0);
        }

        p1_inputs.insert(player1.clone());
        p2_inputs.insert(player2.clone());

        let p1 = player1.pop_front().unwrap();
        let p2 = player2.pop_front().unwrap();

        if player1.len() >= p1 &&  player2.len() >= p2 {
            match recursive_combat(player1.iter().take(p1).cloned().collect(),
            player2.iter().take(p2).cloned().collect(), depth + 1) {
                (Winner::PLAYER1, _) => {
                    player1.push_back(p1);
                    player1.push_back(p2);
                },
                (Winner::PLAYER2, _) => {
                    player2.push_back(p2);
                    player2.push_back(p1);
                }
            }
        } else if p1 > p2 {
            player1.push_back(p1);
            player1.push_back(p2);
        } else {
            player2.push_back(p2);
            player2.push_back(p1);
        }
    }

    if player1.is_empty() {
        (Winner::PLAYER2, player2.iter().rev().enumerate()
            .map(|(i, j)| (i + 1) * *j)
            .sum())
    } else {
        (Winner::PLAYER1, player1.iter().rev().enumerate()
            .map(|(i, j)| (i + 1) * *j)
            .sum())
    }
}

fn part2(input: &str) -> usize {
    let mut players = input.split("\n\n");
    let player1 = players.next().expect("Player 1").lines()
        .skip(1)
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<VecDeque<usize>>();

    let player2 = players.next().expect("Player 2").lines()
        .skip(1)
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<VecDeque<usize>>();

    let (_, score) = recursive_combat(player1, player2, 0);
     score
}

const DAY: u8 = 22;

fn main() -> Result<(), Box<dyn Error>> {
    let input = advent_of_code_2020::get_puzzle_input(DAY)?;

    println!("Part1: {}", part1(&input));
    println!("Part2: {}", part2(&input));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "Player 1:
9
2
6
3
1

Player 2:
5
8
4
7
10";

    #[test]
    fn example1() {
        assert_eq!(306, part1(INPUT));
    }

    #[test]
    fn example2() {
        assert_eq!(291, part2(INPUT));
    }

    #[test]
    fn test_not_infinite() {
        let (winner, _) = recursive_combat(VecDeque::from(vec!(43, 19)),
        VecDeque::from(vec!(2, 29, 14)), 0);
        assert_eq!(Winner::PLAYER1, winner);
    }
}