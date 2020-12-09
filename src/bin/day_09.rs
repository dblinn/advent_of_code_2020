use std::error::Error;
use std::iter::FromIterator;
use std::collections::{VecDeque, HashSet};

fn find_invalid(input: &str, preamble_length: usize) -> Option<(usize, usize)> {
    let mut previous: VecDeque<usize> = input.lines().take(preamble_length)
        .map(|v| v.parse::<usize>().expect(&format!("A valid number. Not {}", v)))
        .collect::<VecDeque<usize>>();
    let mut valid_values: HashSet<usize> = HashSet::from_iter(previous.iter().cloned());


    for (i, v) in input.lines().skip(preamble_length).enumerate() {
        let num = v.parse::<usize>().expect(&format!("A valid number. Not {}", v));
        if !has_two_sum(&valid_values, num) {
            return Some((num, i + preamble_length));
        }

        valid_values.remove(&previous.pop_front().unwrap());
        previous.push_back(num);
        valid_values.insert(num);
    }

    None
}

fn has_two_sum(valid_values: &HashSet<usize>, num: usize) -> bool {
    valid_values.iter()
        .any(|v| num > *v && num - v != *v && valid_values.contains(&(num - v)))
}

fn find_invalid_range(input: &str, invalid_num: usize) -> usize {
    let possible = input.lines()
        .map(|v|v.parse::<usize>().expect(&format!("A valid number. Not {}", v)))
        .collect::<Vec<usize>>();

    let (mut i, mut j, mut sum) = (0, 0, 0);
    while sum != invalid_num {
        if sum < invalid_num {
            sum += possible[j];
            j += 1;
        } else if sum > invalid_num {
            sum -= possible[i];
            i += 1;
        } else {
            break;
        }
    }

    possible[i..j].iter().min().unwrap () + possible[i..j].iter().max().unwrap()
}

fn part1(input: &str) -> usize {
    find_invalid(input, 25).expect("Has invalid value").0
}

fn part2(input: &str, preamble_length: usize) -> usize {
    if let Some((invalid_num, invalid_index)) = find_invalid(input,  preamble_length) {
        find_invalid_range(input, invalid_num)
    } else {
        panic!("Expected an invalid number");
    }
}

const DAY: u8 = 9;

fn main() -> Result<(), Box<dyn Error>> {
    let input = advent_of_code_2020::get_puzzle_input(DAY)?;

    println!("Part1: {}", part1(&input));
    println!("Part2: {}", part2(&input, 25));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576";

    #[test]
    fn example1() {
        assert_eq!(find_invalid(INPUT, 5).unwrap().0, 127);
    }

    #[test]
    fn example2() {
        assert_eq!(part2(INPUT, 5), 62);
    }
}