use super::Solution;

#[derive(Default)]
pub struct Day8 {}

#[derive(Clone)]
enum Instruction {
    NOP(i32),
    ACC(i32),
    JMP(i32),
}

impl Instruction {
    #[inline]
    pub fn swap(&mut self) {
        *self = match *self {
            Instruction::NOP(x) => Instruction::JMP(x),
            Instruction::ACC(x) => Instruction::ACC(x),
            Instruction::JMP(x) => Instruction::NOP(x),
        }
    }
}

impl Day8 {
    fn parse_line(input: &str) -> Instruction {
        let mut split = input.split(" ");

        match split.next().unwrap() {
            "nop" => Instruction::NOP(split.next().unwrap().parse::<i32>().unwrap()),
            "acc" => Instruction::ACC(split.next().unwrap().parse::<i32>().unwrap()),
            "jmp" => Instruction::JMP(split.next().unwrap().parse::<i32>().unwrap()),
            _ => panic!("Invalid Instruction"),
        }
    }

    fn run_program(ins: &[Instruction]) -> (i32, bool) {
        let mut visited = vec![false; ins.len()];
        let mut acc = 0;
        let mut index = 0;

        let mut loop_found = false;

        loop {
            if index >= visited.len() {
                break;
            } else if visited[index] {
                loop_found = true;
                break;
            } else {
                visited[index] = true;
                match ins[index] {
                    Instruction::NOP(_) => index += 1,
                    Instruction::ACC(x) => {
                        acc += x;
                        index += 1;
                    }
                    Instruction::JMP(x) => {
                        if x.is_negative() {
                            index -= x.abs() as usize;
                        } else {
                            index += x as usize;
                        }
                    }
                }
            }
        }

        (acc, loop_found)
    }
}

impl Solution for Day8 {
    type Answer = i32;
    const PROBLEM_NUMBER: u8 = 8;
    const PROBLEM_INPUT: &'static str = include_str!("../../inputs/problem/day8.txt");
    fn solve(input: &str) -> (Self::Answer, Self::Answer) {
        let mut ins = input
            .lines()
            .map(|x| Self::parse_line(x))
            .collect::<Vec<Instruction>>();

        let (part1_answer, mut loop_found) = Self::run_program(&ins);
        let mut part2_answer = 0;
        let mut change_index = 0;
        while loop_found {
            let ins_clone = &mut ins;

            ins_clone[change_index].swap();

            let simulation = Self::run_program(&ins_clone);
            part2_answer = simulation.0;
            loop_found = simulation.1;
            if loop_found {
                ins_clone[change_index].swap();
            }

            change_index += 1;
        }

        (part1_answer, part2_answer)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        let answer = Day8::solve(include_str!("../../inputs/sample/day8.txt"));
        assert_eq!(answer, (5, 8));
    }
}
