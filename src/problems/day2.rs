use super::Solution;

struct Policy<'a> {
    start: usize,
    end: usize,
    letter: u8,
    password: &'a [u8],
}

impl<'a> Policy<'a> {
    fn new(line: &'a str) -> Self {
        let mut splits = line.split_whitespace();
        let mut indices = splits
            .next()
            .unwrap()
            .split('-')
            .map(|x| x.parse::<usize>().unwrap());
        let start = indices.next().unwrap();
        let end = indices.next().unwrap();
        let letter = splits.next().unwrap().chars().next().unwrap() as u8;
        let password = splits.next().unwrap().as_bytes();

        Self {
            start,
            end,
            letter,
            password,
        }
    }

    fn is_valid_part_1(&self) -> bool {
        let mut count = 0;

        for i in self.password {
            if self.letter == *i {
                count += 1;
            }
        }

        count >= self.start && count <= self.end
    }

    fn is_valid_part_2(&self) -> bool {
        (self.password[self.start - 1] == self.letter)
            ^ (self.password[self.end - 1] == self.letter)
    }
}

#[derive(Default)]
pub struct Day2 {}

impl Solution for Day2 {
    type Answer = u64;
    const PROBLEM_NUMBER: u8 = 2;
    const PROBLEM_INPUT: &'static str = include_str!("../../inputs/problem/day2.txt");
    fn solve(input: &str) -> (Self::Answer, Self::Answer) {
        let mut part1_answer = 0;
        let mut part2_answer = 0;

        input.lines().map(|x| Policy::new(x)).for_each(|x| {
            if x.is_valid_part_1() {
                part1_answer += 1;
            }
            if x.is_valid_part_2() {
                part2_answer += 1;
            }
        });

        (part1_answer, part2_answer)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        let answer = Day2::solve(include_str!("../../inputs/sample/day2.txt"));
        assert_eq!(answer, (2, 1));
    }
}
