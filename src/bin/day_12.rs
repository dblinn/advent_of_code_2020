use std::error::Error;
use std::ops::Neg;

#[derive(Copy, Clone, PartialEq, Eq)]
enum Dir {
    N, S, E, W
}

const ALL_DIRS: [Dir; 4] = [Dir::N, Dir::E, Dir::S, Dir::W];

// north and east are positive
struct Travel {
    dir: Dir,
    east_west: i32,
    north_south: i32
}

struct Waypoint {
    east_west: i32,
    north_south: i32
}

impl Dir {
    fn rotate(&self, deg: i32) -> Dir {
        let ndx = ALL_DIRS.iter().position(|&d| d == *self).unwrap() as i32 +  deg / 90;
        let n =  if ndx >= 0 {
            ndx as usize % ALL_DIRS.len()
        } else {
            (ALL_DIRS.len() as i32 + (ndx % ALL_DIRS.len() as i32)) as usize
        };

        ALL_DIRS[n]
    }
}

impl Travel {
    fn movement(&mut self, dir: Dir, units: usize) {
        match dir {
            Dir::E => self.east_west += units as i32,
            Dir::W => self.east_west -= units as i32,
            Dir::N => self.north_south += units as i32,
            Dir::S => self.north_south -= units as i32,
        }
    }

    fn forward(&mut self, units: usize) {
        self.movement(self.dir, units)
    }

    fn to_waypoint(&mut self, waypoint: &Waypoint, units: usize) {
        self.east_west += waypoint.east_west * units as i32;
        self.north_south += waypoint.north_south * units as i32;
    }

    fn turn(&mut self, c: char, deg: usize) {
        self.dir = if c == 'R' {
            self.dir.rotate(deg as i32)
        } else {
            self.dir.rotate((deg as i32).neg())
        }
    }

    fn manhattan_distance(&self) -> usize {
        (self.east_west.abs() + self.north_south.abs()) as usize
    }
}

impl Waypoint {
    fn movement(&mut self, dir: Dir, units: usize) {
        match dir {
            Dir::E => self.east_west += units as i32,
            Dir::W => self.east_west -= units as i32,
            Dir::N => self.north_south += units as i32,
            Dir::S => self.north_south -= units as i32,
        }
    }

    fn turn(&mut self, c: char, deg: usize) {
        let num = deg / 90;
        for _ in 0..num {
            if c == 'R' {
                let tmp = self.north_south;
                self.north_south = -self.east_west;
                self.east_west = tmp;
            } else {
                let tmp = self.north_south;
                self.north_south = self.east_west;
                self.east_west = -tmp;
            }
        }
    }
}

fn num(inp: &str) -> usize {
    inp.parse::<usize>().expect("valid number")
}

fn part1(input: &str) ->usize {
    let mut travel = Travel { dir: Dir::E, east_west: 0, north_south: 0 };

    for l in input.lines() {
        let c = l.chars().next().unwrap();
        match c {
            'N' => travel.movement(Dir::N, num(&l[1..])),
            'S' => travel.movement(Dir::S, num(&l[1..])),
            'E' => travel.movement(Dir::E, num(&l[1..])),
            'W' => travel.movement(Dir::W, num(&l[1..])),
            'F' => travel.forward(num(&l[1..])),
            'R'|'L' => travel.turn(c, num(&l[1..])),
            _ => unreachable!("Unexpected input type {}", c)
        }
    }

    travel.manhattan_distance()
}

fn part2(input: &str) -> usize {
    let mut travel = Travel { dir: Dir::E, east_west: 0, north_south: 0 };
    let mut waypoint = Waypoint { east_west: 10, north_south: 1 };

    for l in input.lines() {
        let c = l.chars().next().unwrap();
        match c {
            'N' => waypoint.movement(Dir::N, num(&l[1..])),
            'S' => waypoint.movement(Dir::S, num(&l[1..])),
            'E' => waypoint.movement(Dir::E, num(&l[1..])),
            'W' => waypoint.movement(Dir::W, num(&l[1..])),
            'F' => travel.to_waypoint(&waypoint, num(&l[1..])),
            'R'|'L' => waypoint.turn(c, num(&l[1..])),
            _ => unreachable!("Unexpected input type {}", c)
        }
    }

    travel.manhattan_distance()
}

const DAY: u8 = 12;

fn main() -> Result<(), Box<dyn Error>> {
    let input = advent_of_code_2020::get_puzzle_input(DAY)?;

    println!("Part1: {}", part1(&input));
    println!("Part2: {}", part2(&input));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "F10
N3
F7
R90
F11";

    #[test]
    fn example1() {
        assert_eq!(25, part1(INPUT));
    }

    #[test]
    fn example2() {
        assert_eq!(286, part2(INPUT));
    }
}