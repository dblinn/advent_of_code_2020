use std::error::Error;

fn evaluate(input: &Vec<char>, start: usize, end: usize) -> (Option<usize>, usize) {
    let mut value = None;
    let mut last_op: Option<char> = None;
    let mut i = start;
    while i < end {
        let c = input[i];
        match c {
            '(' => {
                let (res, until) = evaluate(input, i + 1, end);
                i = until;
                if let Some(n) = res {
                    value = match last_op {
                        Some('+') => value.map(|v| v + n).or(Some(n)),
                        Some('*') => value.map(|v| v * n).or(Some(n)),
                        _ => Some(n)
                    }
                }
            },
            ')' => {
                return (value, i);
            },
            '+'|'*' => {
                last_op = Some(c)
            },
            _ => {
                // Must be a number if properly formatted
                let n = c.to_string().parse::<usize>().expect(
                    &format!("a number, not '{}' on input \"{:?}\"", c, input));
                value = match last_op {
                    Some('+') => value.map(|v| v + n).or(Some(n)),
                    Some('*') => value.map(|v| v * n).or(Some(n)),
                    _ => Some(n)
                }
            }
        }
        i += 1;
    }

    (value, end)
}

fn part1(input: &str) -> usize {
    input.lines().map(|l| {
        let chars = l.chars()
            .filter(|c| !c.is_whitespace())
            .collect::<Vec<char>>();
        evaluate(&chars, 0, chars.len()).0.unwrap()
    }).sum()
}

#[derive(Debug)]
enum Expr {
    Num(usize),
    Multiply
}

fn apply_expressions(expressions: &mut Vec<Expr>, v: Option<usize>) -> Option<usize> {
    let mut value = v;
    if let Some(n) = value {
        expressions.push(Expr::Num(n));
    }
    value = None;
    for expr in expressions {
        value = match expr {
            Expr::Num(n) => value.map(|v| *n * v).or(Some(*n)),
            Expr::Multiply => value
        }
    }

    value
}

fn evaluate2(input: &Vec<char>, start: usize, end: usize) -> (Option<usize>, usize) {
    let mut value = None;
    let mut last_op: Option<char> = None;
    let mut i = start;
    let mut expressions: Vec<Expr> = vec!();

    while i < end {
        let c = input[i];
        match c {
            '(' => {
                let (res, until) = evaluate2(input, i + 1, end);
                i = until;
                if let Some(n) = res {
                    value = match last_op {
                        Some('+') => value.map(|v| v + n).or(Some(n)),
                        Some('*') => {
                            if let Some(x) = value {
                                expressions.push(Expr::Num(x));
                            }
                            expressions.push(Expr::Multiply);
                            Some(n)
                        },
                        _ => Some(n)
                    }
                }
            },
            ')' => {
                value = apply_expressions(&mut expressions, value);
                return (value, i);
            },
            '+'|'*' => {
                last_op = Some(c)
            },
            _ => {
                // Must be a number if properly formatted
                let n = c.to_string().parse::<usize>().expect(
                    &format!("a number, not '{}' on input \"{:?}\"", c, input));
                value = match last_op {
                    Some('+') => value.map(|v| v + n).or(Some(n)),
                    Some('*') => {
                        if let Some(x) = value {
                            expressions.push(Expr::Num(x));
                        }
                        expressions.push(Expr::Multiply);
                        Some(n)
                    },
                    _ => Some(n)
                }
            }
        }
        i += 1;
    }

    // Now do a second pass on all the expressions.
    value = apply_expressions(&mut expressions, value);
    (value, end)
}

fn part2(input: &str) -> usize {
    input.lines().map(|l| {
        let chars = l.chars()
            .filter(|c| !c.is_whitespace())
            .collect::<Vec<char>>();
        evaluate2(&chars, 0, chars.len()).0.unwrap()
    }).sum()
}

const DAY: u8 = 18;

fn main() -> Result<(), Box<dyn Error>> {
    let input = advent_of_code_2020::get_puzzle_input(DAY)?;

    println!("Part1: {}", part1(&input));
    println!("Part2: {}", part2(&input));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(71, part1("1 + 2 * 3 + 4 * 5 + 6"));
        assert_eq!(51, part1("1 + (2 * 3) + (4 * (5 + 6))"));
        assert_eq!(26, part1("2 * 3 + (4 * 5)"));
        assert_eq!(437, part1("5 + (8 * 3 + 9 + 3 * 4 * 3)"));
        assert_eq!(12240, part1("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))"));
        assert_eq!(13632, part1("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"));
    }

    #[test]
    fn example2() {
        assert_eq!(231, part2("1 + 2 * 3 + 4 * 5 + 6"));
        assert_eq!(51, part2("1 + (2 * 3) + (4 * (5 + 6))"));
        assert_eq!(46, part2("2 * 3 + (4 * 5)"));
        assert_eq!(1445, part2("5 + (8 * 3 + 9 + 3 * 4 * 3)"));
        assert_eq!(669_060, part2("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))"));
        assert_eq!(23_340, part2("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"));
    }
}