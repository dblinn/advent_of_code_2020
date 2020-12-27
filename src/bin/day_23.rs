#![feature(linked_list_cursors)]

use std::error::Error;
use std::collections::LinkedList;

#[derive(Debug)]
struct Circle {
    cups: LinkedList<usize>,
    cup_count: usize
}

impl Circle {
    fn new(cups: LinkedList<usize>) -> Self {
        Self { cup_count: cups.len(), cups: cups }
    }

    fn decrement(&self, destination: usize) -> usize {
        let new = destination as i32 - 1;
        (if new <= 0 { new + self.cup_count as i32 } else { new }) as usize
    }

    fn find_destination(&self, current: usize, pick_ups: &(usize, usize, usize)) -> usize {
        let mut destination = self.decrement(current);
        while destination == pick_ups.0 || destination == pick_ups.1 || destination == pick_ups.2 {
            destination = self.decrement(destination);
        }

        destination
    }

    fn play_round(&mut self) {
        let current = self.cups.pop_front().unwrap();
        let pick_ups = (self.cups.pop_front().unwrap(),
                        self.cups.pop_front().unwrap(),
                        self.cups.pop_front().unwrap());

        self.cups.push_back(current);

        let destination = self.find_destination(current, &pick_ups);
        let mut cursor = self.cups.cursor_front_mut();
        loop {
            if let Some(&mut val) = cursor.current() {
                if val == destination {
                    break;
                }
            }
            cursor.move_next();
        }

        cursor.insert_after(pick_ups.2);
        cursor.insert_after(pick_ups.1);
        cursor.insert_after(pick_ups.0);
    }
}

fn part1(input: &str) -> usize {
    let mut circle = Circle::new(input.chars()
        .map(|c| c.to_digit(10).unwrap() as usize).collect());
    for _ in 0 .. 100 {
        circle.play_round();
    }

    let mut answer = String::with_capacity(circle.cup_count);
    let mut cursor = circle.cups.cursor_front();
    while cursor.current() != Some(&1) {
        cursor.move_next();
    }
    for _ in 0 .. circle.cup_count {
        cursor.move_next();
        if let Some(&val) = cursor.current() {
            answer.push((val as u8 + b'0') as char);
        }
    }

    answer.parse::<usize>().unwrap()
}

fn part2(input: &str) -> usize {
    let mut circle = Circle::new(input.chars()
        .map(|c| c.to_digit(10).unwrap() as usize).collect());
    for i in circle.cup_count .. 1_000_000 {
        circle.cups.push_back(i);
    }

    for i in 0 .. 10_000_000 {
        if i % 1000 == 0 {
            println!("Round: {}", i);
        }
        circle.play_round();
    }

    let mut cursor = circle.cups.cursor_front();
    while cursor.current() != Some(&1) {
        cursor.move_next();
    }

    cursor.move_next();
    let a = *cursor.current().unwrap();
    cursor.move_next();
    let b = *cursor.current().unwrap();
    a * b
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = "586439172".to_string();

    println!("Part1: {}", part1(&input));
    println!("Part2: {}", part2(&input));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "389125467";

    #[test]
    fn example1() {
        assert_eq!(67384529, part1(INPUT));
    }

    #[test]
    fn example2() {
        assert_eq!(149245887792, part2(INPUT));
    }
}