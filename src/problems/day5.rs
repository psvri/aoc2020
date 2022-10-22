use std::collections::BinaryHeap;

use super::Solution;

#[derive(Default)]
pub struct Day5 {}

impl Day5 {
    fn get_seat_id(seat: &str) -> u32 {
        let mut chars = seat.chars();

        let mut row = 0;

        for letter in (&mut chars).take(7) {
            match letter {
                'F' => {
                    row = row << 1;
                }
                'B' => {
                    row = (row << 1) | 1;
                }
                _ => panic!("Invalid letter"),
            }
        }

        let mut column = 0;
        for letter in chars {
            match letter {
                'L' => {
                    column = column << 1;
                }
                'R' => {
                    column = (column << 1) | 1;
                }
                _ => panic!("Invalid letter"),
            }
        }

        Self::get_id(row, column)
    }

    #[inline]
    fn get_id(row: u32, column: u32) -> u32 {
        row * 8 + column
    }
}

impl Solution for Day5 {
    type Answer = u32;
    const PROBLEM_NUMBER: u8 = 5;
    const PROBLEM_INPUT: &'static str = include_str!("../../inputs/problem/day5.txt");
    fn solve(input: &str) -> (Self::Answer, Self::Answer) {
        let mut ids = BinaryHeap::new();

        input.lines().for_each(|x| ids.push(Self::get_seat_id(x)));

        let sorted_ids = ids.into_sorted_vec();

        for i in 0..sorted_ids.len() - 1 {
            if sorted_ids[i + 1] - sorted_ids[i] == 2 {
                return (sorted_ids[sorted_ids.len() - 1], sorted_ids[i + 1] - 1);
            }
        }

        (sorted_ids[sorted_ids.len() - 1], 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        let answer = Day5::solve(include_str!("../../inputs/sample/day5.txt"));
        assert_eq!(answer.0, 820);
    }
}
