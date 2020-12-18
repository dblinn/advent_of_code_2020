use std::error::Error;

const ROUNDS: usize = 6;
const NEIGHBORS_3D: [(i32, i32, i32); 26] = [
    (-1, -1, -1),
    (-1, -1, 0),
    (-1, -1, 1),
    (-1, 0, -1),
    (-1, 0, 0),
    (-1, 0, 1),
    (-1, 1, -1),
    (-1, 1, 0),
    (-1, 1, 1),
    (0, -1, -1),
    (0, -1, 0),
    (0, -1, 1),
    (0, 0, -1),
    (0, 0, 1),
    (0, 1, -1),
    (0, 1, 0),
    (0, 1, 1),
    (1, -1, -1),
    (1, -1, 0),
    (1, -1, 1),
    (1, 0, -1),
    (1, 0, 0),
    (1, 0, 1),
    (1, 1, -1),
    (1, 1, 0),
    (1, 1, 1)
];

const NEIGHBORS_4D: [(i32, i32, i32, i32); 80] = [
    (-1, -1, -1, -1),
    (-1, -1, -1, 0),
    (-1, -1, -1, 1),
    (-1, -1, 0, -1),
    (-1, -1, 0, 0),
    (-1, -1, 0, 1),
    (-1, -1, 1, -1),
    (-1, -1, 1, 0),
    (-1, -1, 1, 1),
    (-1, 0, -1, -1),
    (-1, 0, -1, 0),
    (-1, 0, -1, 1),
    (-1, 0, 0, -1),
    (-1, 0, 0, 0),
    (-1, 0, 0, 1),
    (-1, 0, 1, -1),
    (-1, 0, 1, 0),
    (-1, 0, 1, 1),
    (-1, 1, -1, -1),
    (-1, 1, -1, 0),
    (-1, 1, -1, 1),
    (-1, 1, 0, -1),
    (-1, 1, 0, 0),
    (-1, 1, 0, 1),
    (-1, 1, 1, -1),
    (-1, 1, 1, 0),
    (-1, 1, 1, 1),
    (0, -1, -1, -1),
    (0, -1, -1, 0),
    (0, -1, -1, 1),
    (0, -1, 0, -1),
    (0, -1, 0, 0),
    (0, -1, 0, 1),
    (0, -1, 1, -1),
    (0, -1, 1, 0),
    (0, -1, 1, 1),
    (0, 0, -1, -1),
    (0, 0, -1, 0),
    (0, 0, -1, 1),
    (0, 0, 0, -1),
    (0, 0, 0, 1),
    (0, 0, 1, -1),
    (0, 0, 1, 0),
    (0, 0, 1, 1),
    (0, 1, -1, -1),
    (0, 1, -1, 0),
    (0, 1, -1, 1),
    (0, 1, 0, -1),
    (0, 1, 0, 0),
    (0, 1, 0, 1),
    (0, 1, 1, -1),
    (0, 1, 1, 0),
    (0, 1, 1, 1),
    (1, -1, -1, -1),
    (1, -1, -1, 0),
    (1, -1, -1, 1),
    (1, -1, 0, -1),
    (1, -1, 0, 0),
    (1, -1, 0, 1),
    (1, -1, 1, -1),
    (1, -1, 1, 0),
    (1, -1, 1, 1),
    (1, 0, -1, -1),
    (1, 0, -1, 0),
    (1, 0, -1, 1),
    (1, 0, 0, -1),
    (1, 0, 0, 0),
    (1, 0, 0, 1),
    (1, 0, 1, -1),
    (1, 0, 1, 0),
    (1, 0, 1, 1),
    (1, 1, -1, -1),
    (1, 1, -1, 0),
    (1, 1, -1, 1),
    (1, 1, 0, -1),
    (1, 1, 0, 0),
    (1, 1, 0, 1),
    (1, 1, 1, -1),
    (1, 1, 1, 0),
    (1, 1, 1, 1)
];

struct Cube {
    contents: Vec<Vec<Vec<bool>>>,
    // x
    cols: i32,
    // y
    height: i32,
    // z
    rows: i32,
}

impl Clone for Cube {
    fn clone(&self) -> Self {
        Cube { contents: self.contents.clone(), cols: self.cols, rows: self.rows, height: self.height }
    }
}

impl Cube {
    fn new(cols: usize, rows: usize, height: usize) -> Cube {
        let contents: Vec<Vec<Vec<bool>>> = vec![vec![vec![false; height]; rows]; cols];
        Cube { contents: contents, cols: cols as i32, rows: rows as i32, height: height as i32 }
    }

    fn offsets(&self, round: usize) -> (i32, i32, i32) {
        // input cols: 3
        // initial cols: 15
        // offset at round 0: 1
        // cols / 2 - ROUNDS + round
        // offset at round 1: 2
        let offset = (ROUNDS - round) as i32;
        (self.cols / 2 - offset, self.rows / 2 - offset, self.height / 2 - offset)
    }

    fn get(&self, x: i32, y: i32, z: i32) -> bool {
        let a = self.cols / 2 + x;
        let b = self.rows / 2 + y;
        let c = self.height / 2 + z;
        if a < 0 || a >= self.cols || b < 0 || b >= self.rows || c < 0 || c >= self.height {
            return false;
        }

        self.contents[a as usize][b as usize][c as usize]
    }

    fn active_neighbors(&self, x: i32, y: i32, z: i32) -> usize {
        NEIGHBORS_3D.iter().map(|&(nx, ny, nz)| self.get(x + nx, y + ny, z + nz))
            .filter(|n| *n)
            .count()
    }

    fn set(&mut self, x: i32, y: i32, z: i32, active: bool) {
        self.contents[(self.cols / 2 + x) as usize]
            [(self.rows / 2 + y) as usize]
            [(self.height / 2 + z) as usize] = active;
    }

    fn num_active(&self) -> usize {
        self.contents.iter().flat_map(|r| r.iter()
            .flat_map(|h| h))
            .filter(|&&active| active)
            .count()
    }

    fn simulate(&self, round: usize) -> Cube {
        let mut output: Cube = self.clone();
        let (xoff, yoff, zoff) = self.offsets(round);
        for x in -xoff..=xoff {
            for y in -yoff..=yoff {
                for z in -zoff..=zoff {
                    let n = self.active_neighbors(x, y, z);
                    if self.get(x, y, z) {
                        if n < 2 || n > 3 {
                            output.set(x, y, z, false);
                        }
                    } else if n == 3 {
                        output.set(x, y, z, true);
                    }
                }
            }
        }

        println!("Round {}: {} -> {}", round, self.num_active(), output.num_active());
        output
    }
}

struct HyperCube {
    contents: Vec<Vec<Vec<Vec<bool>>>>,
    // x
    cols: i32,
    // y
    rows: i32,
    // z
    height: i32,
    // w
    fourth: i32,
}

impl Clone for HyperCube {
    fn clone(&self) -> Self {
        HyperCube { contents: self.contents.clone(),
            cols: self.cols, rows: self.rows, height: self.height, fourth: self.fourth }
    }
}

impl HyperCube {
    fn new(cols: usize, rows: usize, height: usize, fourth: usize) -> HyperCube {
        let contents: Vec<Vec<Vec<Vec<bool>>>> = vec![vec![vec![vec![false; fourth]; height]; rows]; cols];
        HyperCube { contents: contents,
            cols: cols as i32, rows: rows as i32, height: height as i32, fourth: fourth as i32 }
    }

    fn offsets(&self, round: usize) -> (i32, i32, i32, i32) {
        // input cols: 3
        // initial cols: 15
        // offset at round 0: 1
        // cols / 2 - ROUNDS + round
        // offset at round 1: 2
        let offset = (ROUNDS - round) as i32;
        (self.cols / 2 - offset, self.rows / 2 - offset, self.height / 2 - offset, self.fourth / 2 - offset)
    }

    fn get(&self, x: i32, y: i32, z: i32, w: i32) -> bool {
        let a = self.cols / 2 + x;
        let b = self.rows / 2 + y;
        let c = self.height / 2 + z;
        let d = self.fourth / 2 + w;
        if a < 0 || a >= self.cols ||
            b < 0 || b >= self.rows ||
            c < 0 || c >= self.height ||
            d < 0 || d >= self.fourth {
            return false;
        }

        self.contents[a as usize][b as usize][c as usize][d as usize]
    }

    fn active_neighbors(&self, x: i32, y: i32, z: i32, w: i32) -> usize {
        NEIGHBORS_4D.iter()
            .map(|&(nx, ny, nz, nw)| self.get(x + nx, y + ny, z + nz, w + nw))
            .filter(|n| *n)
            .count()
    }

    fn set(&mut self, x: i32, y: i32, z: i32, w: i32, active: bool) {
        self.contents[(self.cols / 2 + x) as usize]
            [(self.rows / 2 + y) as usize]
            [(self.height / 2 + z) as usize]
            [(self.fourth / 2 + w) as usize] = active;
    }

    fn num_active(&self) -> usize {
        self.contents.iter().flat_map(|r| r.iter()
            .flat_map(|h| h))
            .flat_map(|fourth| fourth)
            .filter(|&&active| active)
            .count()
    }

    fn simulate(&self, round: usize) -> HyperCube {
        let mut output: HyperCube = self.clone();
        let (xoff, yoff, zoff, woff) = self.offsets(round);
        for x in -xoff..=xoff {
            for y in -yoff..=yoff {
                for z in -zoff..=zoff {
                    for w in -woff..=woff {
                        let n = self.active_neighbors(x, y, z, w);
                        if self.get(x, y, z, w) {
                            if n < 2 || n > 3 {
                                output.set(x, y, z, w, false);
                            }
                        } else if n == 3 {
                            output.set(x, y, z, w, true);
                        }
                    }
                }
            }
        }

        println!("Round {}: {} -> {}", round, self.num_active(), output.num_active());
        output
    }
}


fn initialize_cube(cube: &mut Cube, input: &Vec<Vec<bool>>) {
    let (xoff, yoff, zoff) = cube.offsets(0);
    for (y, row) in input.iter().enumerate() {
        for (x, &active) in row.iter().enumerate() {
            if active {
                cube.set(x as i32 - xoff, y as i32 - yoff, zoff, active);
            }
        }
    }
}

fn initialize_hypercube(cube: &mut HyperCube, input: &Vec<Vec<bool>>) {
    let (xoff, yoff, zoff, woff) = cube.offsets(0);
    for (y, row) in input.iter().enumerate() {
        for (x, &active) in row.iter().enumerate() {
            if active {
                cube.set(x as i32 - xoff, y as i32 - yoff, zoff, woff, active);
            }
        }
    }
}

fn part1(input: &str) -> usize {
    let input_cube = input.lines()
        .map(|l| l.chars().map(|c| c == '#').collect::<Vec<bool>>())
        .collect::<Vec<Vec<bool>>>();

    let extra: usize = ROUNDS * 2;
    let (cols, rows, height) = (input_cube[0].len() + extra, input_cube.len() + extra, 1 + extra);
    let mut cube = Cube::new(cols, rows, height);

    initialize_cube(&mut cube, &input_cube);
    for i in 1..=ROUNDS {
        cube = cube.simulate(i);
    }

    cube.num_active()
}

fn part2(input: &str) -> usize {
    let input_cube = input.lines()
        .map(|l| l.chars().map(|c| c == '#').collect::<Vec<bool>>())
        .collect::<Vec<Vec<bool>>>();

    let extra: usize = ROUNDS * 2;
    let (cols, rows, height, fourth) =
        (input_cube[0].len() + extra, input_cube.len() + extra, 1 + extra, 1 + extra);
    let mut cube = HyperCube::new(cols, rows, height, fourth);

    initialize_hypercube(&mut cube, &input_cube);
    for i in 1..=ROUNDS {
        cube = cube.simulate(i);
    }

    cube.num_active()
}

const DAY: u8 = 17;

fn main() -> Result<(), Box<dyn Error>> {
    let input = advent_of_code_2020::get_puzzle_input(DAY)?;

    println!("Part1: {}", part1(&input));
    println!("Part2: {}", part2(&input));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = ".#.
..#
###";

    #[test]
    fn example1() {
        assert_eq!(112, part1(INPUT));
    }

    #[test]
    fn example2() {
        assert_eq!(848, part2(INPUT));
    }
}