use std::error::Error;

fn seat_id(boarding_pass: &str) -> usize {
    boarding_pass.chars().fold(0, |seat_id, c| match c {
        'F' | 'L' => (seat_id << 1) | 0,
        'B' | 'R' => (seat_id << 1) | 1,
        _ => unreachable!()
    })
}

const DAY: u8 = 5;

fn main() -> Result<(), Box<dyn Error>> {
    dotenv::dotenv().ok();

    let input = advent_of_code_2020::get_puzzle_input(DAY)?;
    let max_seat_id = input.lines().map(|l| seat_id(l.trim())).max().unwrap();

    let mut all_seat_ids = input.lines()
        .map(|l| seat_id(l.trim()))
        .collect::<Vec<usize>>();
    all_seat_ids.sort();
    let my_seat = all_seat_ids.windows(2)
        .find(|arr| arr[1] - arr[0] == 2)
        .expect("I have a seat")[0] + 1;

    println!("Part1: {}", max_seat_id);
    println!("Part2: {}", my_seat);
    advent_of_code_2020::check_answer(DAY, 1, max_seat_id)?;
    advent_of_code_2020::check_answer(DAY, 2, my_seat)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(seat_id("BFFFBBFRRR"), 567);
        assert_eq!(seat_id("FFFBBBFRRR"), 119);
        assert_eq!(seat_id("BBFFBBFRLL"), 820);
    }

    #[test]
    fn example2() {

    }
}