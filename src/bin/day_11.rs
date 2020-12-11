#![feature(min_const_generics)]

use std::error::Error;
use std::cmp::{min, max};

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
enum Location {
    Seat(bool),
    Floor
}

type FloorPlan = Vec<Vec<Location>>;
type Position = (usize, usize);

fn display(floor_plan: &FloorPlan) -> String {
    let res = floor_plan.iter().map(|row|
        row.iter().map(|l| match l {
            Location::Seat(true) => '#',
            Location::Seat(false) => 'L',
            Location::Floor => '.'
        }).collect::<String>()).collect::<Vec<String>>();
    res.join("\n")
}

fn val(x: usize) -> char {
    match x {
        0 => '0',
        1 => '1',
        2 => '2',
        3 => '3',
        4 => '4',
        5 => '5',
        6 => '6',
        7 => '7',
        8 => '8',
        _ => unreachable!(),
    }
}

fn display_adjacencies(floor_plan: &FloorPlan) -> String {
    let max_pos = (floor_plan.len(), floor_plan[0].len());
    let res = floor_plan.iter().enumerate().map(|row|
        row.1.iter().enumerate().map(|l| match l.1 {
            Location::Seat(_) => val(adjacent_seats_filled(&(row.0, l.0), floor_plan, &max_pos)),
            Location::Floor => '.'
        }).collect::<String>()).collect::<Vec<String>>();
    res.join("\n")
}

fn parse(input: &str) -> FloorPlan {
    input.lines().map(
        |line| line.chars().map(|c| match c {
            '.' => Location::Floor,
            'L' => Location::Seat(false),
            '#' => Location::Seat(true),
            _ => unreachable!(format!("Illegal char {} in line {}", c, line)),
        }).collect::<Vec<Location>>()
    ).collect()
}

fn adjacent_seats_filled(pos: &Position, floor_plan: &FloorPlan, max_pos: &Position) -> usize {
    (max(1, pos.0) - 1 ..= min(pos.0 + 1, max_pos.0 - 1)).map(|i| {
        (max(1, pos.1) - 1 ..= min(pos.1 + 1, max_pos.1 - 1))
            .filter(|j| i != pos.0 || *j != pos.1)
            .map(|j| floor_plan[i][j])
            .filter(|loc| {
                match loc { Location::Seat(occupied) => *occupied, _ => false }
            })
            .count()
    }).sum()
}

fn visible_seat_filled(pos: &Position, floor_plan: &FloorPlan,
                       max_pos: &Position, row_inc: i32, col_inc: i32) -> bool {
    let mut row = pos.0 as i32 + row_inc;
    let mut col = pos.1 as i32 + col_inc;
    while row >= 0 && col >= 0 && row < max_pos.0 as i32 && col < max_pos.1 as i32 {
        match floor_plan[row as usize][col as usize] {
            Location::Seat(true) => return true,
            Location::Seat(false) => return false,
            Location::Floor => {
                row += row_inc;
                col += col_inc;
            }
        }
    }
    false
}

fn visible_seats_filled(pos: &Position, floor_plan: &FloorPlan, max_pos: &Position) -> usize {
    (-1..=1).map(|i| {
        (-1..=1).filter(|j| i != 0 || *j != 0)
            .filter(|j| visible_seat_filled(pos, floor_plan, max_pos, i, *j))
            .count()
    }).sum()
}

fn equal_plans(a: &FloorPlan, b: &FloorPlan) -> bool {
    a.iter().zip(b.iter())
        .all(|(row_a, row_b)| row_a.iter().zip(row_b.iter())
            .all(|(loc_a, loc_b)| *loc_a == *loc_b))
}

fn simulate<const THRESHOLD: usize>(floor_plan: &FloorPlan, max_pos: &Position,
                                    observe: &dyn Fn(&Position, &FloorPlan, &Position) -> usize) -> FloorPlan
{
    let mut next_round = floor_plan.clone();
    for i in 0..max_pos.0 {
        for j in 0..max_pos.1 {
            if let Location::Seat(currently_occupied) = floor_plan[i][j] {
                let num_occupied = observe(&(i, j), floor_plan, max_pos);
                next_round[i][j] = if num_occupied == 0 {
                    Location::Seat(true)
                } else if num_occupied < THRESHOLD {
                    next_round[i][j]
                } else {
                    Location::Seat(false)
                };
            }
        }
    }

    next_round
}

fn part1(input: &str) -> usize {
    let mut floor_plan = parse(input);

    let max_pos = (floor_plan.len(), floor_plan[0].len());
    for i in 0..10000 {
        let next_plan = simulate::<4>(&floor_plan, &max_pos, &adjacent_seats_filled);
        if equal_plans(&floor_plan, &next_plan) {
            break
        } else {
            floor_plan = next_plan;
        }
    }

    // println!("{}\n", display(&floor_plan));
    //
    // for i in 3 ..= 5 {
    //     for j in 0 ..= 2 {
    //         print!("{:?}", match floor_plan[j][i] { Location::Seat(true) => '#', Location::Seat(false) => 'L', _ => '.' });
    //     }
    //     println!();
    // }
    // println!("{}\n", adjacent_seats_filled(&(4, 1), &floor_plan, &max_pos));
    // println!("{}\n", display_adjacencies(&floor_plan));

    floor_plan.iter().map(|row| row.iter()
        .map(|loc| match loc { Location::Seat(true) => 1, _ => 0}).sum::<usize>()).sum()
}

fn part2(input: &str) -> usize {
    let mut floor_plan = parse(input);

    let max_pos = (floor_plan.len(), floor_plan[0].len());
    for i in 0..10000 {
        let next_plan = simulate::<5>(&floor_plan, &max_pos, &visible_seats_filled);
        if equal_plans(&floor_plan, &next_plan) {
            break
        } else {
            floor_plan = next_plan;
        }
    }

    floor_plan.iter().map(|row| row.iter()
        .map(|loc| match loc { Location::Seat(true) => 1, _ => 0}).sum::<usize>()).sum()
}

const DAY: u8 = 11;

fn main() -> Result<(), Box<dyn Error>> {
    let input = advent_of_code_2020::get_puzzle_input(DAY)?;

    println!("Part1: {}", part1(&input));
    println!("Part2: {}", part2(&input));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL";

    /// the empty seat below would see eight occupied seats:
    const EIGHT: &str = ".......#.
...#.....
.#.......
.........
..#L....#
....#....
.........
#........
...#.....";

    /// The leftmost empty seat below would only see one empty seat,
    /// but cannot see any of the occupied ones:
    const NONE: &str = ".##.##.
#.#.#.#
##...##
...L...
##...##
#.#.#.#
.##.##.";

    /// The empty seat below would see no occupied seats:
    const EMPTY: &str = ".............
.L.L.#.#.#.#.
.............";

    #[test]
    fn example1() {
        assert_eq!(37, part1(INPUT));
    }

    #[test]
    fn test_visibility() {
        let eight = parse(EIGHT);
        let empty = parse(EMPTY);
        let none = parse(NONE);

        assert_eq!(8, visible_seats_filled(&(4, 3), &eight, &(eight.len(), eight[0].len())));
        assert_eq!(0, visible_seats_filled(&(3, 3), &none, &(none.len(), none[0].len())));
        assert_eq!(0, visible_seats_filled(&(1, 1), &empty, &(empty.len(), empty[0].len())));
    }

    #[test]
    fn example2() {
        assert_eq!(26, part2(INPUT));
    }
}
