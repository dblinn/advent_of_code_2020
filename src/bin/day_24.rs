use std::error::Error;
use std::collections::{HashSet, HashMap};

fn calculate_pos(line: &str) -> (i32, i32) {
    let mut x = 0;
    let mut y = 0;
    let mut modified = false;
    for c in line.chars() {
        match c {
            's' => {
                y -= 1;
                modified = true;
            },
            'n' => {
                y += 1;
                modified = true;
            },
            'e' => {
                x += if modified { 1 } else { 2 };
                modified = false;
            },
            'w' => {
                x -= if modified { 1 } else { 2 };
                modified = false;
            }
            _ => unreachable!(format!("Unexpected char {}", c))
        }
    }

    (x, y)
}

fn compute_black_tiles(input: &str) -> HashSet<(i32, i32)> {
    let mut black_tiles: HashSet<(i32, i32)> = HashSet::new();
    for l in input.lines() {
        let pos = calculate_pos(l);
        if black_tiles.contains(&pos) {
            black_tiles.remove(&pos);
        } else {
            black_tiles.insert(pos);
        }
    }
    black_tiles
}

fn part1(input: &str) -> usize {
    let black_tiles = compute_black_tiles(input);

    black_tiles.len()
}

const NEIGHBORS: [(i32, i32); 6] = [
    (2, 0), // e
    (1, -1), // se
    (-1, -1), // sw
    (-2, 0), // w
    (-1, 1), // nw
    (1, 1), // ne
];

struct Tile {
    black: bool,
    black_neighbors: usize
}

fn will_be_black(tile: &Tile) -> bool {
    if tile.black {
        tile.black_neighbors == 1 || tile.black_neighbors == 2
    } else {
        tile.black_neighbors == 2
    }
}

fn simulate(black_tiles: HashSet<(i32, i32)>) -> HashSet<(i32, i32)> {
    // Start off by creating black tiles for all black tile positions.
    let mut tiles: HashMap<(i32, i32), Tile> = black_tiles.iter()
        .map(|&pos| (pos, Tile { black: true, black_neighbors: 0 }))
        .collect();

    // Increment neighbors for all black tiles.
    for &pos in black_tiles.iter() {
        for n in NEIGHBORS.iter() {
            let mut tile = tiles.entry((pos.0 + n.0, pos.1 + n.1))
                .or_insert_with(|| Tile { black: false, black_neighbors: 0 });
            tile.black_neighbors += 1;
        }
    }

    // Collect the tiles that will be black in the next round.
    tiles.iter()
        .filter_map(|(&pos, tile)| if will_be_black(tile) { Some(pos) } else { None })
        .collect()
}

fn part2(input: &str) -> usize {
    let mut black_tiles = compute_black_tiles(input);

    for _ in 0 .. 100 {
        black_tiles = simulate(black_tiles);
    }

    black_tiles.len()
}

const DAY: u8 = 24;

fn main() -> Result<(), Box<dyn Error>> {
    let input = advent_of_code_2020::get_puzzle_input(DAY)?;

    println!("Part1: {}", part1(&input));
    println!("Part2: {}", part2(&input));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "sesenwnenenewseeswwswswwnenewsewsw
neeenesenwnwwswnenewnwwsewnenwseswesw
seswneswswsenwwnwse
nwnwneseeswswnenewneswwnewseswneseene
swweswneswnenwsewnwneneseenw
eesenwseswswnenwswnwnwsewwnwsene
sewnenenenesenwsewnenwwwse
wenwwweseeeweswwwnwwe
wsweesenenewnwwnwsenewsenwwsesesenwne
neeswseenwwswnwswswnw
nenwswwsewswnenenewsenwsenwnesesenew
enewnwewneswsewnwswenweswnenwsenwsw
sweneswneswneneenwnewenewwneswswnese
swwesenesewenwneswnwwneseswwne
enesenwswwswneneswsenwnewswseenwsese
wnwnesenesenenwwnenwsewesewsesesew
nenewswnwewswnenesenwnesewesw
eneswnwswnwsenenwnwnwwseeswneewsenese
neswnwewnwnwseenwseesewsenwsweewe
wseweeenwnesenwwwswnew";

    #[test]
    fn example1() {
        assert_eq!(10, part1(INPUT));
    }

    #[test]
    fn example2() {
        assert_eq!(2208, part2(INPUT));
    }
}