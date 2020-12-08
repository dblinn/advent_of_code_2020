use std::error::Error;
use std::collections::HashMap;

const DAY: u8 = 7;

struct Bag {
    color: String,
    bag_id: usize
}

struct Contains {
    bag_id: usize,
    count: usize
}

struct Rule {
    bag_id: usize,
    contains: Vec<Contains>,
}

fn parse_bag(i: usize, line: &str) -> Bag {
    let parts = line.split(" bags contain ").collect::<Vec<&str>>();
    Bag { color: parts[0].into(), bag_id: i }
}

fn parse_contains(contains: &str, bag_map: &HashMap<String, usize>) -> Option<Contains> {
    if contains.starts_with("no") {
        None
    } else {
        let parts = contains.split_ascii_whitespace().collect::<Vec<&str>>();
        let c: usize = parts[0].parse::<usize>().expect(&format!("Why could I not parse \"{}\"", contains));
        let bag_name = format!("{} {}", parts[1], parts[2]);
        Some(Contains { bag_id: *bag_map.get(&bag_name).unwrap(), count: c })
    }
}

fn parse_rule(line: &str, bag_map: &HashMap<String, usize>) -> Rule {
    let parts = line.split(" bags contain ").collect::<Vec<&str>>();
    if let [bag_color, contains_description] = parts[..] {
        let c: Vec<Contains> = contains_description.split(", ")
            .filter_map(|contains| parse_contains(contains, bag_map))
            .collect();
        Rule { bag_id: *bag_map.get(bag_color).unwrap(), contains: c }
    } else {
        Rule { bag_id: 0, contains: vec!() }
    }
}

fn contains_gold(rules: &Vec<Rule>, bag_id: usize, shiny_gold_id: usize) -> bool {
    bag_id == shiny_gold_id || rules[bag_id].contains.iter()
        .any(|id| contains_gold(rules, id.bag_id, shiny_gold_id))
}

fn bags_contained(rules: &Vec<Rule>, bag_id: usize) -> usize {
    1 + rules[bag_id].contains.iter()
        .map(|c| c.count * bags_contained(rules, c.bag_id))
        .sum::<usize>()
}

fn  part1(input: &str) -> usize {
    let mut bag_map: HashMap<String, usize> = HashMap::new();

    let bags: Vec<Bag> =  input.lines().enumerate()
        .map(|(i, line)| parse_bag(i, line))
        .collect();
    for bag in bags.iter() {
        bag_map.insert(bag.color.clone(), bag.bag_id);
    }

    let rules: Vec<Rule> = input.lines()
        .map(|line| parse_rule(line, &bag_map))
        .collect();
    let gold_id = *bag_map.get("shiny gold").expect("contains gold");
    rules.iter()
        .filter(|r| contains_gold(&rules, r.bag_id, gold_id))
        .map(|r| { println!("{:?}", bags[r.bag_id].color); r })
        .count() - 1
}

fn part2(input: &str) -> usize {
    let mut bag_map: HashMap<String, usize> = HashMap::new();

    let bags: Vec<Bag> =  input.lines().enumerate()
        .map(|(i, line)| parse_bag(i, line))
        .collect();
    for bag in bags.iter() {
        bag_map.insert(bag.color.clone(), bag.bag_id);
    }

    let rules: Vec<Rule> = input.lines()
        .map(|line| parse_rule(line, &bag_map))
        .collect();
    let gold_id = *bag_map.get("shiny gold").expect("contains gold");
    bags_contained(&rules, gold_id) - 1
}


fn main() -> Result<(), Box<dyn Error>> {
    let input = advent_of_code_2020::get_puzzle_input(DAY)?;

    println!("Part1: {}", part1(&input));
    println!("Part2: {}", part2(&input));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.
";

    #[test]
    fn example1() {
        assert_eq!(part1(INPUT), 4);
    }

    const INPUT2: &str = "shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags.";

    #[test]
    fn example2() {
        assert_eq!(part2(INPUT), 32);
        assert_eq!(part2(INPUT2), 126);
    }
}