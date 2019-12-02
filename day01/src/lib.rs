//! # Day 01: The Tyranny of the Rocket Equation
//!
//! [Benchmarking report](../../../day01/target/criterion/report/index.html):
//!
//! * [Part 1](../../../day01/target/criterion/day01_part1/report/index.html)
//! * [Part 2](../../../day01/target/criterion/day01_part2/report/index.html)
//!
//! ## Part One
//!
//! Santa has become stranded at the edge of the Solar System while delivering presents
//! to other planets! To accurately calculate his position in space, safely align his
//! warp drive, and return to Earth in time to save Christmas, he needs you to bring him
//! measurements from _fifty stars_.
//!
//! Collect stars by solving puzzles. Two puzzles will be made available on each day in
//! the Advent calendar; the second puzzle is unlocked when you complete the first. Each
//! puzzle grants _one star_. Good luck!
//!
//! The Elves quickly load you into a spacecraft and prepare to launch.
//!
//! At the first Go / No Go poll, every Elf is Go until the Fuel Counter-Upper. They
//! haven't determined the amount of fuel required yet.
//!
//! Fuel required to launch a given _module_ is based on its _mass_. Specifically, to
//! find the fuel required for a module, take its mass, divide by three, round down,
//! and subtract 2.
//!
//! For example:
//!
//! *   For a mass of `12`, divide by 3 and round down to get `4`, then subtract 2 to get `2`.
//! *   For a mass of `14`, dividing by 3 and rounding down still yields `4`, so the fuel required is also `2`.
//! *   For a mass of `1969`, the fuel required is `654`.
//! *   For a mass of `100756`, the fuel required is `33583`.
//!
//! The Fuel Counter-Upper needs to know the total fuel requirement. To find it, individually
//! calculate the fuel needed for the mass of each module (your puzzle input), then add together
//! all the fuel values.
//!
//! _What is the sum of the fuel requirements_ for all of the modules on your spacecraft?
//!
//!
//! ## Part Two
//!
//! During the second Go / No Go poll, the Elf in charge of the Rocket Equation Double-Checker
//! stops the launch sequence. Apparently, you forgot to include additional fuel for the fuel
//! you just added.
//!
//! Fuel itself requires fuel just like a module - take its mass, divide by three, round down,
//! and subtract 2. However, that fuel _also_ requires fuel, and _that_ fuel requires fuel, and
//! so on. Any mass that would require _negative fuel_ should instead be treated as if it
//! requires _zero fuel_; the remaining mass, if any, is instead handled by _wishing really
//! hard_, which has no mass and is outside the scope of this calculation.
//!
//! So, for each module mass, calculate its fuel and add it to the total. Then, treat the fuel
//! amount you just calculated as the input mass and repeat the process, continuing until a
//! fuel requirement is zero or negative. For example:
//!
//! *   A module of mass `14` requires `2` fuel. This fuel requires no further fuel (2 divided
//! by 3 and rounded down is `0`, which would call for a negative fuel), so the total fuel required is still just `2`.
//! *   At first, a module of mass `1969` requires `654` fuel. Then, this fuel requires `216`
//! more fuel (`654 / 3 - 2`). `216` then requires `70` more fuel, which requires `21` fuel, which
//! requires `5` fuel, which requires no further fuel. So, the total fuel required for a module of
//! mass `1969` is `654 + 216 + 70 + 21 + 5 = 966`.
//! *   The fuel required by a module of mass `100756` and its fuel is:
//! `33583 + 11192 + 3728 + 1240 + 411 + 135 + 43 + 12 + 2 = 50346`.
//!
//! _What is the sum of the fuel requirements_ for all of the modules on your spacecraft when
//! also taking into account the mass of the added fuel? (Calculate the fuel requirements for
//! each module separately, then add them all up at the end.)
//!

use std::fmt::Debug;

pub use anyhow::{Context, Result};
use shrinkwraprs::Shrinkwrap;

pub mod initial;
pub use crate::initial::Day01Initial;

type Day01SolutionPart1 = usize;
type Day01SolutionPart2 = usize;

#[derive(Debug, Shrinkwrap, PartialEq)]
pub struct Day01Entry(usize);

impl Day01Entry {
    pub fn weight(&self) -> usize {
        weight(self.0)
    }
}

pub fn weight(module: usize) -> usize {
    (module / 3).saturating_sub(2)
}

pub trait AoC<'a>: Debug {
    type SolutionPart1;
    type SolutionPart2;

    fn description(&self) -> &'static str {
        "None"
    }

    fn new(input: &'a str) -> Self
    where
        Self: Sized;

    fn solution_part1(&self) -> Self::SolutionPart1 {
        unimplemented!()
    }

    fn solution_part2(&self) -> Self::SolutionPart2 {
        unimplemented!()
    }
}

pub fn parse_input<'a>(input: &'a str) -> impl Iterator<Item = Day01Entry> + 'a {
    input
        .lines()
        .map(|line| Day01Entry(line.trim().parse().expect("Invalid entry")))
}

pub static PUZZLE_INPUT: &str = include_str!("../input");

pub mod benchmark {
    use super::*;

    pub type BenchmarkVector<'a> = Vec<
        Box<
            dyn AoC<'a, SolutionPart1 = Day01SolutionPart1, SolutionPart2 = Day01SolutionPart2>
                + 'a,
        >,
    >;

    pub fn to_benchmark<'a>() -> BenchmarkVector<'a> {
        vec![Box::new(Day01Initial::new(PUZZLE_INPUT))]
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
    fn modules_weight() {
        init_logger();

        let to_check = Day01Entry(12).weight();
        let expected = 2;
        assert_eq!(to_check, expected);

        let to_check = Day01Entry(14).weight();
        let expected = 2;
        assert_eq!(to_check, expected);

        let to_check = Day01Entry(1969).weight();
        let expected = 654;
        assert_eq!(to_check, expected);

        let to_check = Day01Entry(100756).weight();
        let expected = 33583;
        assert_eq!(to_check, expected);
    }

    #[test]
    fn parse() {
        init_logger();

        let parsed: Vec<Day01Entry> = parse_input(PUZZLE_INPUT).collect();
        assert_eq!(parsed.len(), 100);
        assert_eq!(
            &parsed[0..5],
            &[
                Day01Entry(130541),
                Day01Entry(69856),
                Day01Entry(104618),
                Day01Entry(149406),
                Day01Entry(64500)
            ]
        );
        assert_eq!(
            &parsed[95..100],
            &[
                Day01Entry(89177),
                Day01Entry(82419),
                Day01Entry(98712),
                Day01Entry(148947),
                Day01Entry(50931)
            ]
        );
    }
}
