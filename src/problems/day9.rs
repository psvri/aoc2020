use ahash::AHashSet as HashSet;

use super::Solution;

#[derive(Default)]
pub struct Day9 {}

impl Day9 {
    pub fn validate_xmas(input: &[u64], preamble: usize) -> (u64, u64) {
        let mut invalid_index = 0;

        'outer: for i in preamble..input.len() - 1 {
            for j in (i - preamble)..(i - 1) {
                for k in (j + 1)..i {
                    if input[j] + input[k] == input[i] {
                        continue 'outer;
                    }
                }
            }
            invalid_index = i;
            break 'outer;
        }

        let mut window_size = 2;
        let mut min = u64::MAX;
        let mut max = 0;

        'outer: loop {
            for i in 0..invalid_index - (window_size - 1) {
                let mut count = 0;
                let mut sum = 0;
                while count < window_size {
                    sum += input[i + count];
                    count += 1;
                }

                if sum == input[invalid_index] {
                    for j in i..i + window_size {
                        if input[j] >= max {
                            max = input[j];
                        }
                        if input[j] <= min {
                            min = input[j];
                        }
                    }
                    break 'outer;
                }
            }
            window_size += 1;
        }

        (input[invalid_index], min + max)
    }
}

impl Solution for Day9 {
    type Answer = u64;
    const PROBLEM_NUMBER: u8 = 9;
    const PROBLEM_INPUT: &'static str = include_str!("../../inputs/problem/day9.txt");
    fn solve(input: &str) -> (Self::Answer, Self::Answer) {
        let numbers = input
            .lines()
            .map(|x| x.parse().unwrap())
            .collect::<Vec<_>>();

        Self::validate_xmas(&numbers, 25)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        let input = include_str!("../../inputs/sample/day9.txt")
            .lines()
            .map(|x| x.parse().unwrap())
            .collect::<Vec<_>>();
        let answer = Day9::validate_xmas(&input, 5);
        assert_eq!(answer, (127, 62));
    }
}
