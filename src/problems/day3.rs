use super::Solution;

const TREE: u8 = '#' as u8;
const OPEN_SQUARE: u8 = '.' as u8;

#[derive(Default)]
pub struct Day3 {}

impl Day3 {
    fn compute_slope(map: &Vec<&[u8]>, incr_x: usize, incr_y: usize) -> u64 {
        let mut answer = 0;
        let x_len: usize = map[0].len();
        let mut start = (0, 0);
        while start.1 < map.len() - incr_y {
            start.0 = (start.0 + incr_x) % x_len;
            start.1 += incr_y;

            if map[start.1][start.0] == TREE {
                answer += 1;
            }
        }

        answer
    }
}

impl Solution for Day3 {
    type Answer = u64;
    const PROBLEM_NUMBER: u8 = 3;
    const PROBLEM_INPUT: &'static str = include_str!("../../inputs/problem/day3.txt");
    fn solve(input: &str) -> (Self::Answer, Self::Answer) {
        let map = input.lines().map(|x| x.as_bytes()).collect::<Vec<&[u8]>>();

        const SLOPES: [(usize, usize); 5] = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];

        let part1_answer = Self::compute_slope(&map, SLOPES[1].0, SLOPES[1].1);

        let mut part2_answer = 1;

        for (x, y) in SLOPES {
            part2_answer *= Self::compute_slope(&map, x, y);
        }

        (part1_answer, part2_answer)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        let answer = Day3::solve(include_str!("../../inputs/sample/day3.txt"));
        assert_eq!(answer, (7, 336));
    }
}
