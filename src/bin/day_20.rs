use std::error::Error;
use itertools::Itertools;
use std::collections::HashSet;
use crate::Orientation::{Rot90FlipV, Rot270, Rot270FlipV};

struct Tile {
    num: usize,
    pic: Vec<Vec<char>>,

    /// codes for each side and their flipped variant
    // left and flipped left
    l: usize,
    lflip: usize,

    // right and flipped right
    r: usize,
    rflip: usize,

    // top and flipped top
    t: usize,
    tflip: usize,

    // bottom and flipped bottom
    b: usize,
    bflip: usize
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Orientation {
    Original,
    FlipV,          // Note that FlipV is the same as ROT_180, then flip horizontally
    FlipH,
    FlipVH,         // Note that FlipVH is the same as ROT_180

    Rot90,         // Rotate 90 degrees clockwise
    Rot90FlipV,    // Rotate 90 degrees clockwise, then flip vertically

    Rot270,         // Rotate 270 degrees clockwise
    Rot270FlipV,    // Rotate 270 degrees clockwise, then flip vertically
}

const ORIENTATIONS: [Orientation; 8] = [Orientation::Original, Orientation::FlipV,
    Orientation::FlipH, Orientation::FlipVH,
    Orientation::Rot90, Rot90FlipV, Rot270, Rot270FlipV];

impl Tile {
    fn new(num: usize, pic: Vec<Vec<char>>) -> Tile {
        let (l, lflip) = compute_codes(&pic.iter()
            .map(|c| *c.first().unwrap())
            .collect::<Vec<char>>());
        let (r, rflip) = compute_codes(&pic.iter()
            .map(|c| *c.last().unwrap())
            .collect::<Vec<char>>());
        let (t, tflip) = compute_codes(&pic[0]);
        let (b, bflip) = compute_codes(&pic[pic.len() - 1]);

        Tile {
            num,
            pic,
            l, lflip,
            r, rflip,
            t, tflip,
            b, bflip
        }
    }

    fn left(&self, o: Orientation) -> usize {
        match o {
            Orientation::Original => self.l,
            Orientation::FlipV => self.lflip,

            Orientation::FlipH => self.r,
            Orientation::FlipVH => self.rflip,

            Orientation::Rot90 => self.b,
            Orientation::Rot90FlipV => self.bflip,

            Orientation::Rot270 => self.tflip,
            Orientation::Rot270FlipV => self.t,
        }
    }

    fn right(&self, o: Orientation) -> usize {
        match o {
            Orientation::Original => self.r,
            Orientation::FlipV => self.rflip,

            Orientation::FlipH => self.l,
            Orientation::FlipVH => self.lflip,

            Orientation::Rot90 => self.t,
            Orientation::Rot90FlipV => self.tflip,

            Orientation::Rot270 => self.bflip,
            Orientation::Rot270FlipV => self.b,
        }
    }

    fn top(&self, o: Orientation) -> usize {
        match o {
            Orientation::Original => self.t,
            Orientation::FlipH => self.tflip,

            Orientation::FlipV => self.b,
            Orientation::FlipVH => self.bflip,

            Orientation::Rot90 => self.lflip,
            Orientation::Rot90FlipV => self.rflip,

            Orientation::Rot270 => self.r,
            Orientation::Rot270FlipV => self.l,
        }
    }

    fn bottom(&self, o: Orientation) -> usize {
        match o {
            Orientation::Original => self.b,
            Orientation::FlipH => self.bflip,

            Orientation::FlipV => self.t,
            Orientation::FlipVH => self.tflip,

            Orientation::Rot90 => self.rflip,
            Orientation::Rot90FlipV => self.lflip,

            Orientation::Rot270 => self.l,
            Orientation::Rot270FlipV => self.r,
        }
    }

    fn all_codes(&self) -> HashSet<usize> {
        [self.l, self.lflip, self.r, self.rflip, self.t, self.tflip, self.b, self.bflip]
            .iter().cloned().collect()
    }
}

/// For each tile, find all other tiles that could possibly be
/// a neighbor in any pair of orientations.
fn compute_potential_neighbors(tiles: &Vec<Tile>) -> Vec<HashSet<usize>> {
    let mut neighbors: Vec<HashSet<usize>> = vec![HashSet::new(); tiles.len()];
    let codes: Vec<HashSet<usize>> = tiles.iter()
        .map(|t| t.all_codes())
        .collect();

    for (i, codes_i) in codes.iter().enumerate() {
        for (j, codes_j) in codes[(i+1)..].iter().enumerate() {
            let j = j + i + 1;
            if !codes_i.is_disjoint(codes_j) {
                neighbors[i].insert(j);
                neighbors[j].insert(i);
            }
        }
    }

    neighbors
}

// Given the tiles already placed, what orientations are possible for the next tile.
fn possible_orientations(tiles: &Vec<Tile>, placed: &Vec<(usize, Orientation)>,
                         tile: &Tile, next: usize, dim: usize) -> Vec<Orientation> {
    let row = next / dim;
    let col = next % dim;

    let above_bottom = if row != 0 {
        let (above_tile, above_o) = placed[(row - 1) * dim + col];
        let above = &tiles[above_tile];
        Some(above.bottom(above_o))
    } else {
        None
    };

    let left_right = if col != 0 {
        let (left_tile, left_o) = placed[next - 1];
        let left = &tiles[left_tile];
        Some(left.right(left_o))
    } else {
        None
    };

    ORIENTATIONS.iter().filter(|&&o|
        above_bottom.map_or(true, |ab| tile.top(o) == ab)
    ).filter(|&&o|
        left_right.map_or(true, |lr| tile.left(o) == lr)
    ).map(|o| *o)
        .collect::<Vec<Orientation>>()
}

fn compute_codes(data: &Vec<char>) -> (usize, usize) {
    let mut v0: usize = 0;
    let mut v1: usize = 0;
    for (i, &c) in data.iter().enumerate() {
        v0 = (v0 << 1) + if c == '#' { 0 } else { 1 };
        v1 = ((if c == '#' { 0 } else { 1 }) << i) + v1;
    }

    (v0, v1)
}

fn parse_tile(input: &str) -> Tile {
    let mut l = input.lines();
    let x = l.next().unwrap();
    let num = x[x.find(char::is_whitespace).unwrap() + 1 .. x.len() - 1]
        .parse::<usize>().expect("a number");
    let pic = l.map(|row| row.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    Tile::new(num, pic)
}

fn find_placements(tiles: &Vec<Tile>, dim: usize) -> Vec<(usize, Orientation)> {
    let mut arrangements = (0 .. tiles.len())
        .permutations(tiles.len())
        .collect::<Vec<Vec<usize>>>();
    loop {
        let arrangement = arrangements.first().expect("Some arrangement worked");
        let result = check_arrangement(tiles, &mut vec!(), dim, &arrangement);

        if let Some(orientation) = result.0 {
            return arrangement.iter().zip(orientation)
                .map(|(&p, o)| (p, o))
                .collect();
        } else {
            // Remove all arrangements that contain the pattern that failed.
            arrangements.retain(|arr|
                !arrangement_contains(arr, &result.1, dim));
            println!("Remaining: {}", arrangements.len());
        }
    }
}

// Test if the arrangement matches the pattern where the main
// tile has the above tile above it and the left tile to the left of it
//
// this should return true:
// _   _   _
// _   abv _
// lft mn  _
//
// while this should return false:
// _   abv _
// mn  _   _
// _   _   lft
fn arrangement_contains(arrangement: &Vec<usize>, pattern: &(usize, usize, usize), dim: usize) -> bool {
    let &(main, above, left) = pattern;
    let row = main / dim;
    let col = main % dim;
    for i in 1 .. arrangement.len() {
        if arrangement[i] == main {
            let col_match = if col > 0 { arrangement[i - 1] == left } else { true };
            let row_match = if row > 0 && i > dim { arrangement[i - dim] == above } else { true };
            return col_match && row_match;
        }
    }

    false
}

fn check_arrangement(tiles: &Vec<Tile>, placed: &mut Vec<(usize, Orientation)>,
                     dim: usize, arrangement: &Vec<usize>)
    -> (Option<Vec<Orientation>>, (usize, usize, usize)) {
    let next = arrangement[placed.len()];
    let o = if placed.is_empty() {
        ORIENTATIONS.to_vec()
    } else {
        possible_orientations(tiles, placed, &tiles[next], next, dim)
    };

    // TODO: This is not actually correct because the above and left may be locked
    // FIXME: out of some valid orientations which we strip out even though they might work.
    // If no orientations are possible, the arrangement fails.
    // Return the main, above, and left that had no orientations that worked.
    //
    if o.is_empty() {
        return (None, (next, if next > dim { next - dim } else { 0 }, next - 1));
    }

    for orientation in o {
        placed.push((next, orientation));
        if placed.len() == tiles.len() {
            return (Some(placed.iter().map(|(_, orient)| *orient).collect()), (0,0,0));
        }

        let res = check_arrangement(tiles, placed, dim, arrangement);
        if res.0.is_some() {
            return res;
        }
    }

    (None, (next, if next > dim { next - dim } else { 0 }, if next > 0 { next - 1 } else { 0 }))
}

fn part1(input: &str) -> usize {
    let tiles = input.trim().split("\n\n")
        .map(|t| parse_tile(t))
        .collect::<Vec<Tile>>();

    println!("{:?}", compute_potential_neighbors(&tiles));

    // let len = ((tiles.len() as f64).sqrt() + 0.0001) as usize;
    // let orientations = find_placements(&tiles, len);

    // let (tl, tr, bl, br) =
    //     (orientation[0], orientation[len-1], orientation[len * (len - 1)], orientation[len * len - 1]);
    // tiles[tl.0 * len + tl.1].num *
    //     tiles[tr.0 * len + tr.1].num *
    //     tiles[bl.0 * len + bl.1].num *
    //     tiles[br.0 * len + br.1].num
    0
}

fn part2(input: &str) -> usize {
    0
}

const DAY: u8 = 20;

fn main() -> Result<(), Box<dyn Error>> {
    let input = advent_of_code_2020::get_puzzle_input(DAY)?;
    // let input = INPUT;

    println!("Part1: {}", part1(&input));
    println!("Part2: {}", part2(&input));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "Tile 2311:
..##.#..#.
##..#.....
#...##..#.
####.#...#
##.##.###.
##...#.###
.#.#.#..##
..#....#..
###...#.#.
..###..###

Tile 1951:
#.##...##.
#.####...#
.....#..##
#...######
.##.#....#
.###.#####
###.##.##.
.###....#.
..#.#..#.#
#...##.#..

Tile 1171:
####...##.
#..##.#..#
##.#..#.#.
.###.####.
..###.####
.##....##.
.#...####.
#.##.####.
####..#...
.....##...

Tile 1427:
###.##.#..
.#..#.##..
.#.##.#..#
#.#.#.##.#
....#...##
...##..##.
...#.#####
.#.####.#.
..#..###.#
..##.#..#.

Tile 1489:
##.#.#....
..##...#..
.##..##...
..#...#...
#####...#.
#..#.#.#.#
...#.#.#..
##.#...##.
..##.##.##
###.##.#..

Tile 2473:
#....####.
#..#.##...
#.##..#...
######.#.#
.#...#.#.#
.#########
.###.#..#.
########.#
##...##.#.
..###.#.#.

Tile 2971:
..#.#....#
#...###...
#.#.###...
##.##..#..
.#####..##
.#..####.#
#..#.#..#.
..####.###
..#.#.###.
...#.#.#.#

Tile 2729:
...#.#.#.#
####.#....
..#.#.....
....#..#.#
.##..##.#.
.#.####...
####.#.#..
##.####...
##..#.##..
#.##...##.

Tile 3079:
#.#.#####.
.#..######
..#.......
######....
####.#..#.
.#...#.##.
#.#####.##
..#.###...
..#.......
..#.###...";

    #[test]
    fn example1() {
        assert_eq!(20899048083289, part1(INPUT));
    }

    #[test]
    fn example2() {
        assert_eq!(286, part2(INPUT));
    }

    #[test]
    fn test_foo() {
        assert_eq!((16, 1), compute_codes(&".####".chars().collect()));
        assert_eq!((1, 16), compute_codes(&"####.".chars().collect()));
    }

    #[test]
    fn test_parse_tile() {
        let t = parse_tile("Tile 555:
.####
.####
.####
.####
.###.");
        assert_eq!(555, t.num);
        assert_eq!(16, t.t);
        assert_eq!(1, t.tflip);
        assert_eq!(31, t.l);
        assert_eq!(31, t.lflip);
        assert_eq!(16, t.rflip);
        assert_eq!(1, t.r);
        assert_eq!(17, t.b);
        assert_eq!(17, t.bflip);

        assert_eq!(16, t.top(Orientation::Original));
        assert_eq!(17, t.top(Orientation::FlipV));
        assert_eq!(1, t.top(Orientation::FlipH));
        assert_eq!(17, t.top(Orientation::FlipVH));

        assert_eq!(17, t.bottom(Orientation::Original));
        assert_eq!(16, t.bottom(Orientation::FlipV));
        assert_eq!(17, t.bottom(Orientation::FlipH));
        assert_eq!(1, t.bottom(Orientation::FlipVH));

        assert_eq!(31, t.left(Orientation::Original));
        assert_eq!(31, t.left(Orientation::FlipV));
        assert_eq!(1, t.left(Orientation::FlipH));
        assert_eq!(16, t.left(Orientation::FlipVH));

        assert_eq!(1, t.right(Orientation::Original));
        assert_eq!(16, t.right(Orientation::FlipV));
        assert_eq!(31, t.right(Orientation::FlipH));
        assert_eq!(31, t.right(Orientation::FlipVH));
    }

    #[test]
    fn test_arrangement_contains() {
        let v = (0..9).into_iter().collect();
        assert_eq!(true, arrangement_contains(&v, &(2, 0, 1), 3));
        assert_eq!(true, arrangement_contains(&v, &(5, 2, 4), 3));
        assert_eq!(true, arrangement_contains(&v, &(3, 0, 0), 3));
    }

    #[test]
    fn test_possible_orientations() {
        let t1 = parse_tile("Tile 555:
.####
.####
.####
.####
.###.");
        let t2 = parse_tile("Tile 556:
####.
####.
####.
####.
####.");
        let t3 = parse_tile("Tile 557:
.####
.####
.####
.####
.####");
        let t4 = parse_tile("Tile 558:
.####
.####
.####
.####
.####");

        let v = vec!(t1, t2, t3, t4);
        assert_eq!(vec!(Orientation::FlipH, Orientation::FlipVH),
                   possible_orientations(&v,
                                         &vec!((0, Orientation::Original),
                                               (1, Orientation::Original),
                                               (2, Orientation::Original)),
                                         &v[3], 3, 2));
    }
}