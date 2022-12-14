pub mod problems;
use crate::problems::Solution;
use paste::paste;

macro_rules! import_days {
    { $($days:literal),* } => {
        $(
            paste! {
                use problems::[<day $days>]::[<Day $days>];
            }
        )*
    };
}

import_days! {1, 2, 3, 4, 5, 6, 8, 9}

macro_rules! seq_solve {
    { $($days:literal),* } => {
        $(
            paste! {
                [<Day $days>]::print_solution();
            }
        )*
    };
}

pub fn solve_seq() {
    seq_solve! {1, 2, 3, 4, 5, 6, 8,9 };
}
