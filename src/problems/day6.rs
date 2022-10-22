use ahash::AHashSet as HashSet;

use super::Solution;

#[derive(Default)]
pub struct Day6 {}

impl Day6 {}

impl Solution for Day6 {
    type Answer = u32;
    const PROBLEM_NUMBER: u8 = 6;
    const PROBLEM_INPUT: &'static str = include_str!("../../inputs/problem/day6.txt");
    fn solve(input: &str) -> (Self::Answer, Self::Answer) {
        let mut set_part_1 = HashSet::new();
        let mut set_part_2: Option<HashSet<char>> = None;

        let mut part1_answer = 0;
        let mut part2_answer = 0;

        for line in input.lines() {
            if !line.is_empty() {
                set_part_1.extend(line.chars());
                match set_part_2 {
                    None => set_part_2 = Some(HashSet::from_iter(line.chars())),
                    Some(old_set) => {
                        set_part_2 = Some(
                            old_set
                                .intersection(&line.chars().collect())
                                .map(|x| *x)
                                .collect(),
                        );
                    }
                }
            } else {
                part1_answer += set_part_1.len() as u32;
                part2_answer += set_part_2.unwrap().len() as u32;
                set_part_1.clear();
                set_part_2 = None;
            }
        }

        part1_answer += set_part_1.len() as u32;
        part2_answer += set_part_2.unwrap().len() as u32;

        (part1_answer, part2_answer)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        let answer = Day6::solve(include_str!("../../inputs/sample/day6.txt"));
        assert_eq!(answer, (11, 6));
    }
}
