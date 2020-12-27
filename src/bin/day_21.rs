use std::error::Error;
use std::collections::{HashSet, HashMap};
use itertools::Itertools;

#[derive(Debug)]
struct IngredientList {
    allergens: HashSet<String>,
    ingredients: HashSet<String>
}

fn parse_ingredients(line: &str) -> IngredientList {
    let mut split = line.split(" (contains ");
    let ingredients = split.next().unwrap().split(" ")
        .map(|s| s.to_string())
        .collect::<HashSet<String>>();
    let allergens = split.next().unwrap().split(" ")
        .map(|s| s[..s.len() - 1].to_string())
        .collect::<HashSet<String>>();
    IngredientList { ingredients, allergens }
}

fn possible_ingredients(allergen: &String, ing: &Vec<IngredientList>) -> HashSet<String> {
    let mut containing = ing.iter()
        .filter(|i| i.allergens.contains(allergen))
        .map(|i| &i.ingredients);

    let mut p = HashSet::<String>::new();
    if let Some(first) = containing.next() {
        p.extend(first.iter().cloned());

        for next in containing {
            p = p.intersection(next).cloned().collect::<HashSet<String>>();
        }
    }

    println!("{}: {:?}", allergen, p);
    p
}

fn find_cannot_be_allergen(ing: &Vec<IngredientList>) -> HashSet<String> {
    let mut all_allergens = HashSet::<String>::new();
    for i in ing {
        for j in &i.allergens {
            all_allergens.insert(j.clone());
        }
    }

    let ingredients_that_might_be_allergens = all_allergens.iter()
        .flat_map(|a| possible_ingredients(a, ing))
        .map(|a| a.clone())
        .collect::<HashSet<String>>();

    ing.iter().flat_map(|i| i.ingredients.iter())
        .filter(|&i| !ingredients_that_might_be_allergens.contains(i))
        .cloned()
        .collect::<HashSet<String>>()
}

fn part1(input: &str) -> usize {
    let ing = input.lines().map(|l| parse_ingredients(l))
        .collect::<Vec<IngredientList>>();
    let non_allergen = find_cannot_be_allergen(&ing);

    let mut sum: usize = 0;
    for i in ing {
        sum += i.ingredients.intersection(&non_allergen).count();
    }

    sum
}

fn part2(input: &str) -> String {
    let ing = input.lines().map(|l| parse_ingredients(l))
        .collect::<Vec<IngredientList>>();

    let mut all_allergens = HashSet::<String>::new();
    for i in &ing {
        for j in &i.allergens {
            all_allergens.insert(j.clone());
        }
    }

    let mut allergen_possibilities: Vec<(String, HashSet<String>)> = all_allergens.iter()
        .map(|a| (a.clone(), possible_ingredients(a, &ing)))
        .collect::<Vec<_>>();
    let mut allergens: HashMap<String, String> = HashMap::with_capacity(allergen_possibilities.len());

    while allergen_possibilities.len() > 0 {
        let x = allergen_possibilities.iter()
            .find_position(|(_a, p)| p.len() == 1)
            .map(|(ndx, (a, p))| (ndx, a.clone(), p.iter().next().unwrap().clone()));
        if let Some((ndx, a, p)) = x {
            allergen_possibilities.remove(ndx);
            let ingredient = p;
            allergens.insert(a.clone(), ingredient.clone());
            for (_a, p) in allergen_possibilities.iter_mut() {
                p.remove(&ingredient);
            }
        }
    }

    allergens.iter()
        .sorted_by_key(|&(k, _v)| k.as_str())
        .map(|(_k, v)| v.as_str())
        .join(",")
}

const DAY: u8 = 21;

fn main() -> Result<(), Box<dyn Error>> {
    let input = advent_of_code_2020::get_puzzle_input(DAY)?;

    println!("Part1: {}", part1(&input));
    println!("Part2: {}", part2(&input));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
trh fvjkl sbzzf mxmxvkd (contains dairy)
sqjhc fvjkl (contains soy)
sqjhc mxmxvkd sbzzf (contains fish)";

    #[test]
    fn example1() {
        assert_eq!(5, part1(INPUT));
    }

    #[test]
    fn example2() {
        assert_eq!("mxmxvkd,sqjhc,fvjkl".to_string(), part2(INPUT));
    }
}