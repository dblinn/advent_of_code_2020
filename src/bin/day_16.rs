use std::error::Error;
use parse_display::{Display as PDisplay, FromStr as PFromStr};

#[derive(PDisplay, PFromStr, Debug)]
#[display("{name}: {ll}-{lh} or {hl}-{hh}")]
struct Constraint {
    name: String,
    ll: usize,
    lh: usize,
    hl: usize,
    hh: usize
}

impl Constraint {
    fn meets(&self, n: usize) -> bool {
        (n >= self.ll && n <= self.lh) || (n >= self.hl && n <= self.hh)
    }

    fn possible_fields(&self, tickets: &Vec<Vec<usize>>, num_fields: usize) -> Vec<usize> {
        (0 .. num_fields).into_iter()
            .filter(|&i| tickets.iter().all(|t| self.meets(t[i])))
            .collect::<Vec<usize>>()
    }
}

fn part1(input: &str) -> usize {
    let parts = input.split("\n\n").collect::<Vec<&str>>();
    let constraint_lines = parts[0];
    let other_tickets = parts[2];

    let constraints = constraint_lines.trim().lines()
        .map(|l| l.parse::<Constraint>().expect("a constraint"))
        .collect::<Vec<Constraint>>();

    other_tickets.lines().skip(1)
        .flat_map(|l| l.split(",").map(|w| w.parse::<usize>().expect("a number")))
        .filter(|&n| !constraints.iter().any(|c| c.meets(n)))
        .sum()
}

fn part2(input: &str) -> usize {
    let parts = input.split("\n\n").collect::<Vec<&str>>();
    let constraint_lines = parts[0];
    let other_tickets = parts[2];
    let your_ticket = parts[1].lines().skip(1).next().unwrap()
        .split(",").map(|w| w.parse::<usize>().expect("a number"))
        .collect::<Vec<usize>>();

    let constraints = constraint_lines.trim().lines()
        .map(|l| l.parse::<Constraint>().expect("a constraint"))
        .collect::<Vec<Constraint>>();

    let mut valid_tickets = other_tickets.lines().skip(1)
        .map(|l| l.split(",")
            .map(|w| w.parse::<usize>().expect("a number"))
            .collect::<Vec<usize>>())
        .filter(|ticket| ticket.iter().all(|&n| constraints.iter().any(|c| c.meets(n))))
        .collect::<Vec<Vec<usize>>>();

    valid_tickets.push(your_ticket.clone());
    let num_fields = valid_tickets[0].len();
    assert_eq!(num_fields, constraints.len());

    let assigned_constraints = assign_constraints(&constraints, &valid_tickets);
    constraints.iter().enumerate()
        .filter(|&(_i, c)| c.name.starts_with("departure"))
        .map(|(i, _c)| assigned_constraints[i])
        .map(|i| your_ticket[i])
        .product()
}

// Return constraint ids
fn assign_constraints(constraints: &Vec<Constraint>, tickets: &Vec<Vec<usize>>) -> Vec<usize> {
    let num_fields = tickets[0].len();
    let mut eligible = constraints.iter().enumerate()
        .map(|(i, c)| (i, c.possible_fields(tickets, num_fields)))
        .collect::<Vec<(usize, Vec<usize>)>>();

    let mut assignments = vec![0; num_fields];
    loop {
        let &(ndx, e) = &eligible.iter().enumerate()
            .find(|&(_i, e)| e.1.len() == 1)
            .expect("Eligible with one value");

        let field_num = *e.1.first().unwrap();
        assignments[e.0] = field_num;
        println!("Assigning {} to field number {}", constraints[e.0].name, field_num);
        eligible.remove(ndx);
        for e in eligible.iter_mut() {
            e.1.remove(e.1.iter().position(|x| *x == field_num).expect("needle not found"));
        }

        if eligible.len() == 0 {
            break;
        }
    }

    assignments
}

const DAY: u8 = 16;

fn main() -> Result<(), Box<dyn Error>> {
    let input = advent_of_code_2020::get_puzzle_input(DAY)?;

    println!("Part1: {}", part1(&input));
    println!("Part2: {}", part2(&input));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12";

    const INPUT2: &str = "class: 0-1 or 4-19
row: 0-5 or 8-19
seat: 0-13 or 16-19

your ticket:
11,12,13

nearby tickets:
3,9,18
15,1,5
5,14,9
";

    #[test]
    fn example1() {
        assert_eq!(71, part1(INPUT));
    }

    #[test]
    fn example2() {
        assert_eq!(175594, part2(INPUT2));
    }
}