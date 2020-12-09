use std::error::Error;
use std::collections::HashSet;

#[derive(Copy, Clone, Debug)]
enum Instruction {
    Nop(i32),
    Acc(i32),
    Jmp(i32),
}

#[derive(Copy, Clone, PartialEq, Eq)]
enum Termination {
    InfiniteLoop,
    Terminated
}

fn parse_instruction(line: &str) -> Option<Instruction> {
    line.get(..3).and_then(|instr| {
        match instr {
            "nop" => line.get(4..).map(|val| Instruction::Nop(val.parse::<i32>().expect(line))),
            "acc" => line.get(4..).map(|val| Instruction::Acc(val.parse::<i32>().expect(line))),
            "jmp" => line.get(4..).map(|val| Instruction::Jmp(val.parse::<i32>().expect(line))),
            _ => None,
        }
    })
}

fn run_instructions(instructions: &Vec<Instruction>) -> (i32, Termination) {
    let mut ndx: i32 = 0;
    let mut accum: i32 = 0;
    let mut seen: HashSet<i32> = HashSet::new();
    while ndx < instructions.len() as i32 {
        if !seen.insert(ndx) {
            return (accum, Termination::InfiniteLoop);
        }

        match instructions[ndx as usize] {
            Instruction::Nop(_) => (),
            Instruction::Acc(val) => {
                accum = accum + val;
            }
            Instruction::Jmp(val) => {
                ndx = ndx + val - 1;
            }
        }
        ndx = ndx + 1;
    }

    (accum, Termination::Terminated)
}

fn part1(input: &str) -> i32 {
    let instructions = input.lines()
        .flat_map(|line| parse_instruction(line))
        .collect::<Vec<Instruction>>();

    run_instructions(&instructions).0
}

fn part2(input: &str) -> i32 {
    let mut instructions = input.lines()
        .flat_map(|line| parse_instruction(line))
        .collect::<Vec<Instruction>>();

    let mut status = Termination::InfiniteLoop;
    let mut result = 0;
    let mut line_to_check: usize = 0;
    let mut last_checked = 0;
    let mut last_instruction = instructions[0];
    while status == Termination::InfiniteLoop {
        loop {
            match instructions[line_to_check] {
                Instruction::Nop(val) => {
                    instructions[last_checked] = last_instruction;
                    last_checked = line_to_check;
                    last_instruction = instructions[line_to_check];
                    instructions[line_to_check] = Instruction::Jmp(val);
                    break;
                },
                Instruction::Jmp(val) => {
                    instructions[last_checked] = last_instruction;
                    last_checked = line_to_check;
                    last_instruction = instructions[line_to_check];
                    instructions[line_to_check] = Instruction::Nop(val);
                    break;
                }
                _ => line_to_check = line_to_check + 1
            }
        }

        line_to_check = line_to_check + 1;
        let x = run_instructions(&instructions);
        result = x.0;
        status = x.1
    }

    result
}

const DAY: u8 = 8;

fn main() -> Result<(), Box<dyn Error>> {
    let input = advent_of_code_2020::get_puzzle_input(DAY)?;

    println!("Part1: {}", part1(&input));
    println!("Part2: {}", part2(&input));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6";

    #[test]
    fn example1() {
        assert_eq!(part1(INPUT), 5);
    }

    #[test]
    fn example2() {
        assert_eq!(part2(INPUT), 8);
    }
}