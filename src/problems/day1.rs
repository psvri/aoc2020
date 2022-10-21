use super::Solution;

#[derive(Default)]
pub struct Day1 {}

impl Day1 {
    fn get_ints(input: &str) -> Vec<u64> {
        input.lines().map(|x| x.parse::<u64>().unwrap()).collect()
    }
}

impl Solution for Day1 {
    type Answer = u64;
    const PROBLEM_NUMBER: u8 = 1;
    const PROBLEM_INPUT: &'static str = include_str!("../../inputs/problem/day1.txt");
    fn solve(input: &str) -> (Self::Answer, Self::Answer) {
        let numbers = Self::get_ints(input);

        let mut part1_answer = 0;
        let mut part2_answer = 0;

        'outer: for i in 0..numbers.len() - 1 {
            for j in i + 1..numbers.len() {
                if numbers[i] + numbers[j] == 2020 {
                    part1_answer = numbers[i] * numbers[j];
                    break 'outer;
                }
            }
        }

        'outer: for i in 0..numbers.len() - 2 {
            for j in i + 1..numbers.len() - 1 {
                for k in j + 1..numbers.len() {
                    if numbers[i] + numbers[j] + numbers[k] == 2020 {
                        part2_answer = numbers[i] * numbers[j] * numbers[k];
                        break 'outer;
                    }
                }
            }
        }

        (part1_answer, part2_answer)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        let answer = Day1::solve(include_str!("../../inputs/sample/day1.txt"));
        assert_eq!(answer, (514579, 241861950));
    }
}
