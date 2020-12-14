use std::error::Error;

fn part1(input: &str) -> usize {
    let mut lines = input.lines();
    let earliest_departure = lines.next().unwrap().parse::<usize>().expect("valid number");
    let soonest = lines.next().unwrap().split(",")
        .filter(|t| t != &"x")
        .map(|t| t.parse::<usize>().expect("valid number"))
        .map(|t| {
            (t - (earliest_departure % t), t)
        })
        .min_by_key(|(wait, _t)| *wait)
        .expect("has at least one");

    soonest.0 * soonest.1
}

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn least_common_multiple(x: usize, y: usize) -> usize {
    (x * y) / gcd(x, y)
}

// Find the smallest number n such that x * n + remainder == y * n
fn smallest_with_remainder(x: usize, y: usize, remainder: usize) -> usize {
    for i in 1.. {
        if ((i * x) + remainder) % y == 0 {
            println!("{}, {}, {}, {}, {}", i * x, i * x + remainder, x, y, remainder);
            return i * x + remainder;
        }
    }

    unreachable!();
}

fn part2(input: &str) -> usize {
    let vals = input.lines().skip(1).next().unwrap().split(",").collect::<Vec<&str>>();
    let mut prev: Vec<(usize, usize)> = vec!();
    let mut key = vals[0].parse::<usize>().expect("first value is number");
    for (i, x) in vals[1..].iter().enumerate() {
        if x == &"x" {
            // nothing to do. All valid.
            continue;
        } else {
            let num = x.parse::<usize>().expect("a number");
            for z in 1..5 {
                let possible = smallest_with_remainder(key * z, num, i + 1);
                if all_divisible(possible, &prev) {
                    key = possible;
                    println!("Found key for {}: {}", num, key);
                    break;
                }
            }

            prev.push((num, i + 1));
        }
    }

    key
}

// You are actually supposed to use some esoteric arithmetic called "Chinese Remainder Theorem"
// to solve this but I am not familiar with it.
fn part2_again(input: &str) -> usize {
    let mut vals = input.lines().skip(1).next().unwrap().split(",")
        .enumerate()
        .filter(|&(i, t)| t != "x")
        .map(|(i, t)| (t.parse::<usize>().expect("a number"), i))
        .collect::<Vec<(usize,usize)>>();
    println!("{:?}", vals);

    let max = *vals.iter().max_by_key(|&&(t, i)| t).unwrap();
    let first = max.0 - max.1;
    if max.0 != vals[0].0 {
        vals.remove(vals.iter().position(|&x| x.0 == max.0).unwrap());
    }

    let evenly_divis = vals.first().unwrap().0;
    vals.remove(0);

    // for i in 2_250_000_000.. {
    // for i in   77_897_700_000.. {
    for i in (156862000000 * 3).. {
    // for i in 0.. {
        if i % 1_000_000 == 0 {
            println!("Iteration: {}", i);
        }

        let v = first + max.0 * i;
        if (v % evenly_divis) == 0 {
            if all_divisible(v, &vals) {
                println!("{}, {}, {}, {}", v, i, i % evenly_divis, evenly_divis);
                return v;
            }
        }
    }

    0
}

fn all_divisible(key: usize, prev: &Vec<(usize, usize)>) -> bool {
    prev.iter().all(|&(val, remainder)| {
        (key + remainder) % val == 0
    })
}

const DAY: u8 = 13;

fn main() -> Result<(), Box<dyn Error>> {
    let input = advent_of_code_2020::get_puzzle_input(DAY)?;

    println!("Part1: {}", part1(&input));
    println!("Part2: {}", part2_again(&input));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "939
7,13,x,x,59,x,31,19";

    #[test]
    fn example1() {
        assert_eq!(295, part1(INPUT));
    }

    #[test]
    fn example2() {
        assert_eq!(1068781, part2_again(INPUT));
        assert_eq!(3417, part2_again("\n17,x,13,19"));
        assert_eq!(754018, part2_again("\n67,7,59,61"));
        assert_eq!(779210, part2_again("\n67,x,7,59,61"));
        assert_eq!(1261476, part2_again("\n67,7,x,59,61"));
        assert_eq!(1202161486, part2_again("\n1789,37,47,1889"));
    }

    #[test]
    fn example3() {
        println!("{}, {}, {}", 7, 13, least_common_multiple(7, 13));
    }
}