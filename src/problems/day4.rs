use super::Solution;

const BYR: u8 = 0b0000_0001;
const IYR: u8 = 0b0000_0010;
const EYR: u8 = 0b0000_0100;
const HGT: u8 = 0b0000_1000;
const HCL: u8 = 0b0001_0000;
const ECL: u8 = 0b0010_0000;
const PID: u8 = 0b0100_0000;
const CID: u8 = 0b1000_0000;

const PART1_VALID_1: u8 = 0b1111_1111;
const PART1_VALID_2: u8 = 0b0111_1111;

#[derive(Default)]
pub struct Day4 {}

impl Day4 {
    fn is_valid(input: &[&str]) -> (bool, bool) {
        let mut result_part1 = 0;
        let mut result_part2 = true;

        for item in input {
            let (key_encoded, validity) = Self::encode_and_validate(item);
            result_part1 = result_part1 | key_encoded;
            result_part2 = validity & result_part2;
        }

        let part_1_result = result_part1 == PART1_VALID_1 || result_part1 == PART1_VALID_2;

        (part_1_result, (part_1_result && result_part2))
    }

    fn encode_and_validate(key_pair: &str) -> (u8, bool) {
        let mut split = key_pair.split(":");
        let key = split.next().unwrap();
        let value = split.next().unwrap();
        match key {
            "byr" => (BYR, Self::is_valid_byr(value)),
            "iyr" => (IYR, Self::is_valid_iyr(value)),
            "eyr" => (EYR, Self::is_valid_eyr(value)),
            "hgt" => (HGT, Self::is_valid_hgt(value)),
            "hcl" => (HCL, Self::is_valid_hcl(value)),
            "ecl" => (ECL, Self::is_valid_ecl(value)),
            "pid" => (PID, Self::is_valid_pid(value)),
            "cid" => (CID, Self::is_valid_cid()),
            _ => panic!("Invalid string"),
        }
    }

    #[inline]
    fn is_valid_byr(byr: &str) -> bool {
        (byr.len() == 4)
            & ({
                let byr = byr.parse::<u16>().unwrap();
                (byr >= 1920) & (byr <= 2002)
            })
    }

    #[inline]
    fn is_valid_iyr(iyr: &str) -> bool {
        (iyr.len() == 4)
            & ({
                let byr = iyr.parse::<u16>().unwrap();
                (byr >= 2010) & (byr <= 2020)
            })
    }

    #[inline]
    fn is_valid_eyr(eyr: &str) -> bool {
        (eyr.len() == 4)
            & ({
                let byr = eyr.parse::<u16>().unwrap();
                (byr >= 2020) & (byr <= 2030)
            })
    }

    #[inline]
    fn is_valid_hgt(hgt: &str) -> bool {
        return if hgt.ends_with("in") {
            let height = hgt.replace("in", "").parse::<u32>().unwrap();
            height >= 59 && height <= 76
        } else if hgt.ends_with("cm") {
            let height = hgt.replace("cm", "").parse::<u32>().unwrap();
            height >= 150 && height <= 193
        } else {
            false
        };
    }

    #[inline]
    fn is_valid_hcl(hcl: &str) -> bool {
        let mut iter = hcl.chars();

        if iter.next() != Some('#') {
            return false;
        }

        for letter in iter {
            match letter.to_digit(16) {
                None => return false,
                _ => {}
            }
        }

        true
    }

    #[inline]
    fn is_valid_ecl(ecl: &str) -> bool {
        match ecl {
            "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => true,
            _ => false,
        }
    }

    #[inline]
    fn is_valid_pid(pid: &str) -> bool {
        pid.len() == 9 && pid.chars().all(|x| x.is_ascii_digit())
    }

    #[inline]
    fn is_valid_cid() -> bool {
        true
    }
}

impl Solution for Day4 {
    type Answer = u64;
    const PROBLEM_NUMBER: u8 = 4;
    const PROBLEM_INPUT: &'static str = include_str!("../../inputs/problem/day4.txt");
    fn solve(input: &str) -> (Self::Answer, Self::Answer) {
        let mut passport_vec: Vec<Vec<&str>> = vec![];
        let mut index = 0;
        let mut start = true;

        for line in input.lines() {
            if !line.is_empty() {
                if start {
                    passport_vec.push(line.split_ascii_whitespace().collect::<Vec<&str>>());
                    start = false;
                } else {
                    passport_vec[index].extend(line.split_ascii_whitespace());
                }
            } else {
                start = true;
                index += 1;
            }
        }

        passport_vec
            .iter()
            .map(|x| Self::is_valid(x))
            .fold((0, 0), |mut acc, x| {
                acc.0 += x.0 as u64;
                acc.1 += x.1 as u64;
                acc
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        let answer = Day4::solve(include_str!("../../inputs/sample/day4.txt"));
        assert_eq!(answer, (8, 4));
    }
}
