use paste::paste;
use std::fmt::Debug;

macro_rules! mod_days {
    { $($days:literal),* } => {
        $(
            paste! {
                pub mod [<day $days>];
            }
        )*
    };
}

mod_days! {1, 2}

pub trait Solution: Sync {
    type Answer: Debug;
    const PROBLEM_NUMBER: u8;
    const PROBLEM_INPUT: &'static str;
    fn solve(input: &str) -> (Self::Answer, Self::Answer);
    fn print_solution() {
        print_answer(Self::PROBLEM_NUMBER, Self::solve(Self::PROBLEM_INPUT))
    }
}

pub fn print_answer<T: Debug>(day: u8, answer: (T, T)) {
    #[cfg(not(feature = "print_err"))]
    println!("Answer for day {:?} is {:?}", day, answer);

    #[cfg(feature = "print_err")]
    eprintln!("Answer for day {:?} is {:?}", day, answer);
}
