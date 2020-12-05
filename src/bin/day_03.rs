#![feature(min_const_generics)]

use std::str::FromStr;
use std::error::Error;

struct GridRow {
    row : Vec<bool>
}

impl FromStr for GridRow {
    type Err = std::num::ParseIntError;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        Ok(GridRow { row: line.chars().map(|c| {
            match c {
                '.' => false,
                '#' => true,
                _ => true
            }
        }).collect() })
    }
}

struct Toboggan<const MOVEX: usize, const MOVEY: usize> {

}

// Use const generics!!!!!!!! Cool :)
impl <const MOVEX: usize, const MOVEY: usize> Toboggan<MOVEX, MOVEY> {
    fn count_trees(&self, grid: &Vec<GridRow>) -> usize {
        let mut x_pos = MOVEX;
        let mut y_pos = MOVEY;
        let mut tree_count = 0;
        while y_pos < grid.len() {
            tree_count += if grid[y_pos].row[x_pos] { 1 } else { 0 };
            x_pos = (x_pos + MOVEX) % grid[y_pos].row.len();
            y_pos += MOVEY;
        }

        tree_count
    }
}

fn part1(input: &str) -> usize {
    let grid_rows = input.lines()
        .map(|line| line.parse::<GridRow>().unwrap())
        .collect();

    Toboggan::<3, 1> {}.count_trees(&grid_rows)
}

fn part2(input: &str) -> usize {
    let grid_rows = input.lines()
        .map(|line| line.parse::<GridRow>().unwrap())
        .collect();

    let mut product = 1;
    product *= Toboggan::<1, 1> {}.count_trees(&grid_rows);
    product *= Toboggan::<3, 1> {}.count_trees(&grid_rows);
    product *= Toboggan::<5, 1> {}.count_trees(&grid_rows);
    product *= Toboggan::<7, 1> {}.count_trees(&grid_rows);
    product * Toboggan::<1, 2> {}.count_trees(&grid_rows)
}

const DAY: u8 = 3;

fn main() -> Result<(), Box<dyn Error>> {
    let input = advent_of_code_2020::get_puzzle_input(DAY)?;
    println!("Part1: {}", part1(&input));
    println!("Part2: {}", part2(&input));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#";

    #[test]
    fn example1() {
        assert_eq!(part1(INPUT), 7);
    }

    #[test]
    fn example2() {
        assert_eq!(part2(INPUT), 336)
    }
}
