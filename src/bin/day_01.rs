use std::error::Error;
use std::collections::{HashSet};

fn find_summing_pair(target: i32, input: &Vec<usize>) -> Option<i32> {
    let mut set: HashSet<i32> = HashSet::new();
    for i in input.iter().map(|x| *x as i32) {
        let diff : i32 = target - i;
        if set.contains(&diff) {
            return Some(diff * i);
        } else {
            set.insert(i);
        }
    }
    None
}

fn find_summing_triplet(target: i32, input: &Vec<usize>) -> Option<i32> {
    let mut set: HashSet<i32> = HashSet::new();
    for (i, a_usize) in input.iter().enumerate() {
        let a = *a_usize as i32;
        set.insert(a);

        for b in input[i..].iter().map(|x| *x as i32) {
            let diff = target - a - b;
            if set.contains(&diff) {
                return Some(a * b * diff);
            } else {
                set.insert(b);
            }
        }
    }
    None
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = advent_of_code_2020::get_puzzle_input(1)?;
    let input: Vec<usize> = input
        .lines()
        .map(|l| l.trim().parse())
        .collect::<Result<_, _>>()?;

    let answer = find_summing_pair(2020, &input)
        .ok_or("answer not found")?;
    println!("Part1: {}", answer);

    let answer = find_summing_triplet(2020, &input)
        .ok_or("answer not found")?;
    println!("Part2: {}", answer);

    Ok(())
}
