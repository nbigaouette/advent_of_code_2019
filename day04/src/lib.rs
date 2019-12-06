//! # Day 04: Secure Container
//!
//! [GitHub source](https://github.com/nbigaouette/advent_of_code_2019/tree/master/day04)
//!
//! [Benchmarking report](../../../day04/target/criterion/report/index.html)
//!
//! ## Part One
//!
//! You arrive at the Venus fuel depot only to discover it's protected by a password. The Elves had
//! written the password on a sticky note, but someone threw it out.
//!
//! However, they do remember a few key facts about the password:
//!
//! * It is a six-digit number.
//! * The value is within the range given in your puzzle input.
//! * Two adjacent digits are the same (like `22` in `1_22_345`).
//! * Going from left to right, the digits _never decrease_; they only ever increase or
//!   stay the same (like `111123` or `135679`).
//!
//! Other than the range rule, the following are true:
//!
//! * `111111` meets these criteria (double `11`, never decreases).
//! * `2234_50_` does not meet these criteria (decreasing pair of digits `50`).
//! * `123789` does not meet these criteria (no double).
//!
//! _How many different passwords_ within the range given in your puzzle input meet these criteria?
//!
//! ## Part Two
//!
//! An Elf just remembered one more important detail: the two adjacent matching digits _are not part of
//! a larger group of matching digits_.
//!
//! Given this additional criterion, but still ignoring the range rule, the following are now true:
//!
//! * `112233` meets these criteria because the digits never decrease and all repeated digits are
//!   exactly two digits long.
//! * `123_444_` no longer meets the criteria (the repeated `44` is part of a larger group of `444`).
//! * `111122` meets the criteria (even though `1` is repeated more than twice, it still contains a
//!   double `22`).
//!
//! _How many different passwords_ within the range given in your puzzle input meet all of the
//! criteria?
//!

use std::{convert::TryFrom, fmt::Debug, ops::Range};

pub use anyhow::{Context, Result};

pub mod initial;
pub use crate::initial::Day04Initial;

#[derive(Debug, PartialEq)]
pub struct Day04Input {
    start: Password,
    stop_inc: Password,
}

impl Day04Input {
    pub fn range(&self) -> Range<usize> {
        Range {
            start: self.start.value,
            end: self.stop_inc.value + 1,
        }
    }
}

const PASSWORD_LENGTH: usize = 6;

impl TryFrom<&str> for Password {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let digits_vec: Vec<u8> = value
            .chars()
            .take(PASSWORD_LENGTH)
            .map(|c| {
                c.to_digit(10)
                    .ok_or_else(|| anyhow::anyhow!("Bad character: {}", c))
                    .unwrap() as u8
            })
            .collect();
        let mut digits_array: [u8; PASSWORD_LENGTH] = [0; PASSWORD_LENGTH];
        digits_array.copy_from_slice(&digits_vec);
        let password = Password {
            value: value.parse::<usize>()?,
            digits: digits_array,
        };
        Ok(password)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Password {
    value: usize,
    digits: [u8; PASSWORD_LENGTH],
}

impl Password {
    fn is_adjacent_same(&self) -> bool {
        self.digits.windows(2).filter(|dd| dd[0] == dd[1]).count() >= 1
    }

    fn is_adjacent_increase_monotone(&self) -> bool {
        self.digits.windows(2).filter(|dd| dd[0] > dd[1]).count() == 0
    }

    fn is_adjacent_pair(&self) -> bool {
        (0..=9)
            .filter_map(|to_find| {
                if self.digits.iter().filter(|&&d| d == to_find).count() == 2 {
                    Some(())
                } else {
                    None
                }
            })
            .count()
            > 0
    }

    pub fn is_valid_part1(&self) -> bool {
        self.is_adjacent_same() && self.is_adjacent_increase_monotone()
    }
    pub fn is_valid_part2(&self) -> bool {
        self.is_adjacent_pair() && self.is_adjacent_increase_monotone()
    }
}

type Day04SolutionPart1 = usize;
type Day04SolutionPart2 = usize;

pub trait AoC: Debug {
    type SolutionPart1;
    type SolutionPart2;

    fn description(&self) -> &'static str {
        "None"
    }

    fn new(input: &str) -> Self
    where
        Self: Sized;

    fn solution_part1(&self) -> Self::SolutionPart1 {
        unimplemented!()
    }

    fn solution_part2(&self) -> Self::SolutionPart2 {
        unimplemented!()
    }
}

pub fn parse_input(input: &str) -> Result<Day04Input> {
    let values: Vec<&str> = input.trim().split('-').collect();
    let start: &str = &values
        .get(0)
        .ok_or_else(|| anyhow::anyhow!("Failed to take first item"))?;
    let end_inclusive: &str = &values
        .get(1)
        .ok_or_else(|| anyhow::anyhow!("Failed to take second item"))?;
    Ok(Day04Input {
        start: Password::try_from(start)?,
        stop_inc: Password::try_from(end_inclusive)?,
    })
}

pub static PUZZLE_INPUT: &str = include_str!("../input");

pub mod benchmark {
    use super::*;

    pub type BenchmarkVector =
        Vec<Box<dyn AoC<SolutionPart1 = Day04SolutionPart1, SolutionPart2 = Day04SolutionPart2>>>;

    pub fn to_benchmark() -> BenchmarkVector {
        vec![Box::new(Day04Initial::new(PUZZLE_INPUT))]
    }
}

#[cfg(test)]
mod tests {
    use env_logger;
    use std::env;

    use pretty_assertions::assert_eq;

    use crate::*;

    pub fn init_logger() {
        env::var("RUST_LOG")
            .or_else(|_| -> Result<String, ()> {
                let rust_log = "debug".to_string();
                println!("Environment variable 'RUST_LOG' not set.");
                println!("Setting to: {}", rust_log);
                env::set_var("RUST_LOG", &rust_log);
                Ok(rust_log)
            })
            .unwrap();
        let _ = env_logger::try_init();
    }

    #[test]
    fn parse() {
        init_logger();

        let parsed = parse_input(PUZZLE_INPUT).unwrap();
        let expected = Day04Input {
            start: Password {
                value: 178416,
                digits: [1, 7, 8, 4, 1, 6],
            },
            stop_inc: Password {
                value: 676461,
                digits: [6, 7, 6, 4, 6, 1],
            },
        };
        assert_eq!(parsed, expected);
    }

    #[test]
    fn password_range() {
        init_logger();

        let parsed = parse_input(PUZZLE_INPUT).unwrap();
        let range = parsed.range();
        let expected = Range {
            start: 178416,
            end: 676461 + 1,
        };
        assert_eq!(range, expected);
    }
}
