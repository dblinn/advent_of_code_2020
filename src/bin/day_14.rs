use parse_display::{Display as PDisplay, FromStr as PFromStr};
use std::error::Error;
use std::collections::HashMap;

#[derive(PDisplay, PFromStr, Debug)]
#[display("mem[{register}] = {value}")]
struct AssignmentOp {
    register: usize,
    value: usize
}

impl AssignmentOp {
    fn execute(&self, registers: &mut HashMap<usize, usize>, mask: &Vec<(usize, bool)>) {
        let mut v = self.value;
        for m in mask {
            v = Self::toggle(v, m);
        }

        registers.insert(self.register, v);
    }

    fn toggle(v: usize, mask: &(usize, bool)) -> usize {
        let &(offset, setting) = mask;
        if setting {
            v | 1 << offset
        } else {
            v & !(1 << offset)
        }
    }

    fn addresses_v2(&self, addr: usize, mask: &str) -> Vec<usize> {
        let mut v = addr;
        for (offset, set) in parse_mask(mask) {
            if set {
                v |= 1 << offset;
            }
        }

        let mut addresses: Vec<usize> = vec!(v);
        for i in mask.chars().rev().enumerate()
            .filter(|&(_i, c)| c == 'X')
            .map(|(i,_c)| i) {
            let num_addresses = addresses.len();
            for j in 0..num_addresses {
                let v = addresses[j];
                addresses[j] = Self::toggle(v, &(i, true));
                addresses.push(Self::toggle(v, &(i, false)));
            }
        }

        addresses
    }

    fn execute_v2(&self, registers: &mut HashMap<usize, usize>, mask: &str) {
        for addr in self.addresses_v2(self.register, mask) {
            registers.insert(addr, self.value);
        }
    }
}

fn parse_mask(mask: &str) -> Vec<(usize, bool)> {
    mask.chars().rev().enumerate()
        .filter(|&(_i, c)| c != 'X')
        .map(|(i, c)| (i, c == '1'))
        .collect::<Vec<(usize, bool)>>()
}

fn part1(input: &str) -> usize {
    let mut registers: HashMap<usize, usize> = HashMap::new();
    let mut mask: Vec<(usize, bool)> = vec!();
    for l in input.lines() {
        if l.starts_with("mem") {
            // Instruction
            let op = l.parse::<AssignmentOp>().expect("valid assignment");
            op.execute(&mut registers, &mask);
        } else {
            // Mask
            mask = parse_mask(&l[7..]);
        }
    };

    registers.values().sum()
}

fn part2(input: &str) -> usize {
    let mut registers: HashMap<usize, usize> = HashMap::new();
    let mut mask: String = "".to_string();
    for l in input.lines() {
        if l.starts_with("mem") {
            // Instruction
            let op = l.parse::<AssignmentOp>().expect("valid assignment");
            op.execute_v2(&mut registers, &mask);
        } else {
            // Mask
            mask = l[7..].to_string();
        }
    };

    registers.values().sum()
}

const DAY: u8 = 14;

fn main() -> Result<(), Box<dyn Error>> {
    let input = advent_of_code_2020::get_puzzle_input(DAY)?;

    println!("Part1: {}", part1(&input));
    println!("Part2: {}", part2(&input));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0";

    const INPUT2: &str = "mask = 000000000000000000000000000000X1001X
mem[42] = 100
mask = 00000000000000000000000000000000X0XX
mem[26] = 1";

    #[test]
    fn example1() {
        assert_eq!(165, part1(INPUT));
    }

    #[test]
    fn example2() {
        assert_eq!(208, part2(INPUT2));
    }
}