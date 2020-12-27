use std::error::Error;
use std::ops::Index;
use itertools::Itertools;

#[derive(Debug)]
struct Circle {
    cups: Vec<usize>,
    cup_count: usize
}

impl Index<usize> for Circle {
    type Output = usize;

    fn index(&self, index: usize) -> &Self::Output {
        &self.cups[index % self.cups.len()]
    }
}

impl Circle {
    fn new(cups: Vec<usize>) -> Self {
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
        let current = self.cups[0];
        let pick_ups = (self.cups[1], self.cups[2], self.cups[3]);

        self.cups.push(current);
        self.cups.drain(0..4);

        let destination = self.find_destination(current, &pick_ups);
        let (index, _) = self.cups.iter()
            .find_position(|&&cup| cup == destination)
            .unwrap();
        self.cups.splice((index + 1) .. (index + 1),
                         [pick_ups.0, pick_ups.1, pick_ups.2].iter().cloned());
    }
}

fn part1(input: &str) -> usize {
    let mut circle = Circle::new(input.chars()
        .map(|c| c.to_digit(10).unwrap() as usize).collect());
    for _ in 0 .. 100 {
        circle.play_round();
    }

    let index1 = circle.cups.iter().find_position(|&&cup| cup == 1).unwrap().0;
    let val = (index1 + 1 .. index1 + circle.cup_count).into_iter()
        .map(|i| (circle[i] as u8 + b'0') as char)
        .collect::<String>();
    val.parse::<usize>().unwrap()
}

fn part2(input: &str) -> usize {
    let mut circle = Circle::new(input.chars()
        .map(|c| c.to_digit(10).unwrap() as usize).collect());
    for i in circle.cup_count .. 1_000_000 {
        circle.cups.push(i);
    }

    for i in 0 .. 10_000_000 {
        if i % 1000 == 0 {
            println!("Round: {}", i);
        }
        circle.play_round();
    }

    let index1 = circle.cups.iter().find_position(|&&cup| cup == 1).unwrap().0;
    let a = circle[index1 + 1];
    let b = circle[index1 + 2];
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