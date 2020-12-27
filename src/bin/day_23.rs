use std::error::Error;
use itertools::Itertools;

struct Cups {
    /// Each index corresponds to the cup_number. The
    /// value stored corresponds to the index of the
    /// next cup in the sequence
    vec: Vec<usize>,
    /// This is the index into the vec of the current
    /// cup.
    current_cup: usize
}

impl Cups {
    /// Given a starting arrangement, give back some Cups. Starting order must
    /// contain every number from 1 to starting_order.len(); the rest will be
    /// filled in order.
    pub fn new(len: usize, starting_order: Vec<usize>) -> Cups {
        let mut v = vec![0; len+1];
        let padding_start = starting_order.iter().copied().max().unwrap_or(0) + 1;
        let order = starting_order.iter().copied().chain(padding_start..len+1);
        for (n, next_n) in order.clone().zip(order.cycle().skip(1)).take(len) {
            v[n] = next_n;
        }
        Cups {
            vec: v,
            current_cup: starting_order[0]
        }
    }
    /// Take one turn from the current position:
    pub fn step(&mut self) {
        // Take 3 cups clockwise of current:
        let (t1, t2, t3) = {
            let mut ts = self.next_after(self.current_cup);
            (ts.next().unwrap(),ts.next().unwrap(),ts.next().unwrap())
        };
        // Find idx of cup to put them in front of:
        let mut next_cup = self.minus_one_cup(self.current_cup);
        while t1 == next_cup || t2 == next_cup || t3 == next_cup {
            next_cup = self.minus_one_cup(next_cup);
        }
        // The current index now points to the thing after the last taken cup:
        self.vec[self.current_cup] = self.vec[t3];
        // Last taken index now points to what the next_index used to:
        self.vec[t3] = self.vec[next_cup];
        // Next index now points to the first taken cup:
        self.vec[next_cup] = t1;
        // Current index is now the next cup around:
        self.current_cup = self.vec[self.current_cup];
    }
    /// Return an iterator over the next cups in line from the number given:
    pub fn next_after(&self, cup: usize) -> impl Iterator<Item=usize> + '_ {
        std::iter::successors(Some(cup), move |cup| Some(self.vec[*cup])).skip(1)
    }
    /// Minus one from the cup number to get the previous one.
    fn minus_one_cup(&self, n: usize) -> usize {
        let num_cups = self.vec.len() - 1;
        (n + (num_cups - 1) - 1) % num_cups + 1
    }
}

fn part1(input: &str) -> usize {
    let cup_numbers = input.chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect::<Vec<usize>>();
    let mut cups = Cups::new(cup_numbers.len(), cup_numbers.clone());

    for _ in 0..100 { cups.step() }
    let val = cups.next_after(1).take(8).join("");
    val.parse::<usize>().unwrap()
}

fn part2(input: &str) -> usize {
    let cup_numbers = input.chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect::<Vec<usize>>();
    let mut cups = Cups::new(1_000_000, cup_numbers.clone());

    for _ in 0..10_000_000 { cups.step() }
    for x in cups.next_after(1).take(2) {
        println!("{}", x);
    }
    cups.next_after(1).take(2).product::<usize>()
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