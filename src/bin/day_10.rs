use std::error::Error;
use std::collections::HashMap;

fn part1(input: &str) -> [usize; 3] {
    let mut vec = input.lines()
        .map(|v| v.parse::<usize>().expect(&format!("A valid number. Not {}", v)))
        .collect::<Vec<usize>>();
    vec.sort();

    let mut diffs: [usize; 3] = [0, 0, 1];
    diffs[*vec.first().unwrap() - 1] += 1;

    for diff in vec.windows(2).map(|w| w[1] - w[0]) {
        diffs[diff - 1] += 1;
    }
    diffs
}

// Example using dynamic programming
// (0) 1 2 3 4 (7)
//
// (0) 13
// arrangements for 1 7
// arrangements for 2 4
// arrangements for 3 2
//
// 1 7
// arrangements for 2 4
// arrangements for 3 2
// arrangements for 4 1
//
// 2 —> 4
// arrangements for 3 2
// arrangements for 4 1
// arrangements for (7) 1
//
// 3 —> 2
// arrangements for 4 1
// arrangements for (7) 1
//
// 4 —> 1
// arrangements for (7) 1
//
// (7) —> 1
// 1
fn part2(input: &str) -> usize {
    let mut vec = input.lines()
        .map(|v| v.parse::<usize>().expect(&format!("A valid number. Not {}", v)))
        .collect::<Vec<usize>>();
    vec.push(0);
    vec.sort();

    // Ensure even window length
    for _ in 0..4 {
        vec.push(*vec.last().unwrap() + 3);
    }

    let mut range_cache: HashMap<usize, Vec<usize>> = HashMap::with_capacity(vec.len());
    for vals in vec.windows(4) {
        let val = vals[0];
        range_cache.insert(val, vals[1..].iter()
            .filter(|v| **v - val <= 3)
            .map(|v| *v)
            .collect());
    }

    let mut combination_count_cache: HashMap<usize, usize> = HashMap::new();
    for i in 1..=4 {
        combination_count_cache.insert(vec[vec.len() - i], 1);
    }

    for val in vec.iter().rev().skip(4) {
        let sum = range_cache.get(val).unwrap().iter()
            .map(|r| *combination_count_cache.get(r).unwrap())
            .sum();
        combination_count_cache.insert(*val, sum);
    }
    *combination_count_cache.get(&0).unwrap()
}

const DAY: u8 = 10;

fn main() -> Result<(), Box<dyn Error>> {
    let input = advent_of_code_2020::get_puzzle_input(DAY)?;

    let [one, _two, three] = part1(&input);
    println!("Part1: {}", one * three);
    println!("Part2: {}", part2(&input));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "16
10
15
5
1
11
7
19
6
12
4";

    const INPUT2: &str = "28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3";

    #[test]
    fn example1() {
        let [one, _two, three] = part1(INPUT);
        assert_eq!(7, one);
        assert_eq!(5, three);

        let [one, _two, three] = part1(INPUT2);
        assert_eq!(22, one);
        assert_eq!(10, three);
    }

    #[test]
    fn example2() {
        assert_eq!(part2(INPUT), 8);
        assert_eq!(part2(INPUT2), 19208);
    }
}