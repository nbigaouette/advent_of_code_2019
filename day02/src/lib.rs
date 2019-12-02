//! # Day 02:
//!
//! [Benchmarking report](../../../day02/target/criterion/report/index.html):
//!
//! * [Part 1](../../../day02/target/criterion/day02_part1/report/index.html)
//! * [Part 2](../../../day02/target/criterion/day02_part2/report/index.html)
//!
//! ## Part One
//!
//!
//!
//! ## Part Two
//!
//!
//!

use std::fmt::Debug;

// pub use anyhow::{Context, Result};
// use shrinkwraprs::Shrinkwrap;

pub mod initial;
pub use crate::initial::Day02Initial;

// pub enum OpCode {
//     Add,      // 1
//     Multiply, // 2
//     Done, // 99
//     Unknown,  // Something went wrong
// }

pub type OpCode = usize;

pub const OPCODE_ADD: usize = 1;
pub const OPCODE_MUL: usize = 2;
pub const OPCODE_DONE: usize = 99;
// Anything else is 'unknown'
pub const OPCODE_JUMP: usize = 4;

// #[derive(Debug, Shrinkwrap, PartialEq)]
// pub struct Day02Entry(usize);

type Day02SolutionPart1 = usize;
type Day02SolutionPart2 = usize;

pub trait AoC<'a>: Debug {
    type SolutionPart1;
    type SolutionPart2;

    fn description(&self) -> &'static str {
        "None"
    }

    fn new(input: &'a str) -> Self
    where
        Self: Sized;

    fn solution_part1(&mut self) -> Self::SolutionPart1 {
        unimplemented!()
    }

    fn solution_part2(&mut self) -> Self::SolutionPart2 {
        unimplemented!()
    }
}

pub fn parse_input<'a>(input: &'a str) -> impl Iterator<Item = OpCode> + 'a {
    input
        .trim()
        .split(',')
        .map(|opcode| opcode.parse::<OpCode>().unwrap())
}

pub static PUZZLE_INPUT: &str = include_str!("../input");

pub mod benchmark {
    use super::*;

    pub type BenchmarkVector<'a> = Vec<
        Box<
            dyn AoC<'a, SolutionPart1 = Day02SolutionPart1, SolutionPart2 = Day02SolutionPart2>
                + 'a,
        >,
    >;

    pub fn to_benchmark<'a>() -> BenchmarkVector<'a> {
        unimplemented!()
        // vec![
        //     Box::new(Day02Initial::new(PUZZLE_INPUT)),
        // ]
    }
}

#[cfg(test)]
mod tests {
    use env_logger;
    use std::env;

    use super::*;

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

        let parsed: Vec<OpCode> = parse_input(PUZZLE_INPUT).collect();
        assert_eq!(parsed.len(), 153);
        assert_eq!(&parsed[0..5], &[1, 0, 0, 3, 1]);
        assert_eq!(&parsed[150..153], &[14, 0, 0]);
    }
}
