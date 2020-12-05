use std::str::FromStr;
use std::error::Error;

const DAY: u8 = 4;

struct Passport {
    byr: String,
    iyr: String,
    eyr: String,
    hgt: String,
    hcl: String,
    ecl: String,
    pid: String,
    cid: String
}

fn valid_range(s: &str, low: usize, high: usize) -> bool {
    s.parse::<usize>().ok().map(|year| year >= low && year <= high).unwrap_or(false)
}

fn byr_valid(s: &str) -> bool {
    valid_range(s, 1920, 2002)
}

fn iyr_valid(s: &str) -> bool {
    valid_range(s, 2010, 2020)
}

fn eyr_valid(s: &str) -> bool {
    valid_range(s, 2020, 2030)
}

fn hgt_valid(s: &str) -> bool {
    s.strip_suffix("cm")
        .map(|p| valid_range(p, 150, 193))
        .or(s.strip_suffix("in").map(|p| valid_range(p, 59, 76)))
        .unwrap_or(false)

}

fn hcl_valid(s: &str) -> bool {
    s.starts_with('#') && s.len() == 7 && s.chars().skip(1).all(|c| c.is_ascii_hexdigit())
}

fn ecl_valid(s: &str) -> bool {
    match s {
        "amb" => true,
        "blu" => true,
        "brn" => true,
        "gry" => true,
        "grn" => true,
        "hzl" => true,
        "oth" => true,
        _ => false
    }
}

fn pid_valid(s: &str) -> bool {
    s.len() == 9 && s.chars().all(|c| c.is_ascii_digit())
}

impl Passport {
    fn fields_present(&self) -> bool {
        !self.byr.is_empty()
            && !self.iyr.is_empty()
            && !self.eyr.is_empty()
            && !self.hgt.is_empty()
            && !self.hcl.is_empty()
            && !self.ecl.is_empty()
            && !self.pid.is_empty()
    }

    fn fields_valid(&self) -> bool {
        return self.fields_present() &&
            byr_valid(&self.byr) &&
            iyr_valid(&self.iyr) &&
            eyr_valid(&self.eyr) &&
            hgt_valid(&self.hgt) &&
            hcl_valid(&self.hcl) &&
            ecl_valid(&self.ecl) &&
            pid_valid(&self.pid)
    }
}

impl FromStr for Passport {
    type Err = String;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let mut byr = "".to_string();
        let mut iyr = "".to_string();
        let mut eyr = "".to_string();
        let mut hgt = "".to_string();
        let mut hcl = "".to_string();
        let mut ecl = "".to_string();
        let mut pid = "".to_string();
        let mut cid = "".to_string();

        for field in line.trim().split(|c| c == ' ' || c == '\n') {
            let parts = field.split(':').collect::<Vec<&str>>();
            if let [ident, val] = parts[..] {
                match ident {
                    "byr" => byr = val.to_string(),
                    "iyr" => iyr = val.to_string(),
                    "eyr" => eyr = val.to_string(),
                    "hgt" => hgt = val.to_string(),
                    "hcl" => hcl = val.to_string(),
                    "ecl" => ecl = val.to_string(),
                    "pid" => pid = val.to_string(),
                    "cid" => cid = val.to_string(),
                    _ => println!("Unknown field: {:?}", ident)
                }
            } else {
                return Err(format!("Unable to parse ident from {:?} (line={:?})", field, line));
            }
        }

        Ok(Self { byr, iyr, eyr, hgt, hcl, ecl, pid, cid })
    }
}

fn part1(input: &str) -> usize {
    input.split("\n\n")
        .map(|s| s.parse::<Passport>().unwrap())
        .filter(|p| p.fields_present())
        .count()
}

fn part2(input: &str) -> usize {
    input.split("\n\n")
        .map(|s| s.parse::<Passport>().unwrap())
        .filter(|p| p.fields_valid())
        .count()
}

fn main() -> Result<(), Box<dyn Error>> {
    dotenv::dotenv().ok();

    let input = advent_of_code_2020::get_puzzle_input(DAY)?;
    advent_of_code_2020::check_answer(DAY, 1, part1(&input))?;
    advent_of_code_2020::check_answer(DAY, 2, part2(&input))?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in";

    const INPUT2_INVALID: &str = "eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007";

    const INPUT2_VALID: &str = "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719";

    #[test]
    fn example1() {
        assert_eq!(part1(INPUT), 2);
    }

    #[test]
    fn example2_invalid() {
        assert_eq!(part2(INPUT2_INVALID), 0);
    }

    #[test]
    fn example2_valid() {
        assert_eq!(part2(INPUT2_VALID), 4);
    }

    #[test]
    fn test_byr() {
        assert!(byr_valid("2002"));
        assert!(!byr_valid("2003"));
    }

    #[test]
    fn test_hgt() {
        assert!(hgt_valid("60in"));
        assert!(hgt_valid("190cm"));

        assert!(!hgt_valid("190in"));
        assert!(!hgt_valid("190"));
    }

    #[test]
    fn test_hcl() {
        assert!(hcl_valid("#123abc"));

        assert!(!hcl_valid("#123abz"));
        assert!(!hcl_valid("123abc"));
    }

    #[test]
    fn test_ecl() {
        assert!(ecl_valid("brn"));

        assert!(!ecl_valid("wat"));
    }

    #[test]
    fn test_pid() {
        assert!(pid_valid("000000001"));

        assert!(!pid_valid("0123456789"));
    }
}