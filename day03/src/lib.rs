//! # Day 03: Crossed Wires
//!
//! [GitHub source](https://github.com/nbigaouette/advent_of_code_2019/tree/master/day03)
//!
//! [Benchmarking report](../../../day03/target/criterion/report/index.html)
//!
//! ## Part One
//!
//! The gravity assist was successful, and you're well on your way to the Venus refuelling station.
//! During the rush back on Earth, the fuel management system wasn't completely installed, so that's
//! next on the priority list.
//!
//! Opening the front panel reveals a jumble of wires. Specifically, _two wires_ are connected to a
//! central port and extend outward on a grid. You trace the path each wire takes as it leaves the
//! central port, one wire per line of text (your puzzle input).
//!
//! The wires twist and turn, but the two wires occasionally cross paths. To fix the circuit, you need
//! to _find the intersection point closest to the central port_. Because the wires are on a grid, use
//! the [Manhattan distance](https://en.wikipedia.org/wiki/Taxicab_geometry) for this measurement.
//! While the wires do technically cross right at the central port where they both start, this point
//! does not count, nor does a wire count as crossing with itself.
//!
//! For example, if the first wire's path is `R8,U5,L5,D3`, then starting from the central port (`o`),
//! it goes right `8`, up `5`, left `5`, and finally down `3`:
//!
//! ```text
//! ...........
//! ...........
//! ...........
//! ....+----+.
//! ....|....|.
//! ....|....|.
//! ....|....|.
//! .........|.
//! .o-------+.
//! ...........
//! ```
//!
//! Then, if the second wire's path is `U7,R6,D4,L4`, it goes up `7`, right `6`, down `4`, and left
//! `4`:
//!
//! ```text
//! ...........
//! .+-----+...
//! .|.....|...
//! .|..+--X-+.
//! .|..|..|.|.
//! .|.-X--+.|.
//! .|..|....|.
//! .|.......|.
//! .o-------+.
//! ...........
//! ```
//!
//! These wires cross at two locations (marked `X`), but the lower-left one is closer to the central
//! port: its distance is `3 + 3 = 6`.
//!
//! Here are a few more examples:
//!
//! * distance `159`:
//!     ```text
//!     R75,D30,R83,U83,L12,D49,R71,U7,L72  
//!     U62,R66,U55,R34,D71,R55,D58,R83
//!     ```
//! * distance `135`:
//!     ```text
//!     R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51  
//!     U98,R91,D20,R16,D67,R40,U7,R15,U6,R7
//!     ```
//!
//! _What is the Manhattan distance_ from the central port to the closest intersection?
//!
//! ## Part Two
//!
//! It turns out that this circuit is very timing-sensitive; you actually need to _minimize the signal delay_.
//!
//! To do this, calculate the _number of steps_ each wire takes to reach each intersection; choose the intersection where the _sum of both wires' steps_ is lowest. If a wire visits a position on the grid multiple times, use the steps value from the _first_ time it visits that position when calculating the total value of a specific intersection.
//!
//! The number of steps a wire takes is the total number of grid squares the wire has entered to get to that location, including the intersection being considered. Again consider the example from above:
//!
//! ```text
//! ...........
//! .+-----+...
//! .|.....|...
//! .|..+--X-+.
//! .|..|..|.|.
//! .|.-X--+.|.
//! .|..|....|.
//! .|.......|.
//! .o-------+.
//! ...........
//! ```
//!
//! In the above example, the intersection closest to the central port is reached after `8+5+5+2 = _20_` steps by the first wire and `7+6+4+3 = _20_` steps by the second wire for a total of `20+20 = _40_` steps.
//!
//! However, the top-right intersection is better: the first wire takes only `8+5+2 = _15_` and the second wire takes only `7+6+2 = _15_`, a total of `15+15 = _30_` steps.
//!
//! Here are the best steps for the extra examples from above:
//!
//! * `610` steps:
//!     ```text
//!     R75,D30,R83,U83,L12,D49,R71,U7,L72  
//!     U62,R66,U55,R34,D71,R55,D58,R83
//!     ```
//! * `410` steps:
//!     ```text
//!     R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51  
//!     U98,R91,D20,R16,D67,R40,U7,R15,U6,R7
//!     ```
//!
//! _What is the fewest combined steps the wires must take to reach an intersection?_
//!

use std::{fmt::Debug, str::FromStr};

pub use anyhow::{Context, Result};

pub mod initial;
pub use crate::initial::Day03Initial;

pub type Length = isize;

#[derive(Debug, PartialEq)]
pub enum Day03Entry {
    Up(Length),
    Down(Length),
    Left(Length),
    Right(Length),
}

impl FromStr for Day03Entry {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (direction_str, length_str) = s.trim().split_at(1);
        let length: Length = length_str.parse()?;
        let direction = match direction_str {
            "R" => Day03Entry::Right(length),
            "L" => Day03Entry::Left(length),
            "U" => Day03Entry::Up(length),
            "D" => Day03Entry::Down(length),
            unknown => anyhow::bail!("Unknown direction: {:?}", unknown),
        };
        Ok(direction)
    }
}

type Day03SolutionPart1 = isize;
type Day03SolutionPart2 = isize;

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

pub fn parse_line<'a>(line: &'a str) -> impl Iterator<Item = Day03Entry> + 'a {
    line.trim()
        .split(',')
        .filter_map(|entry| entry.parse::<Day03Entry>().ok())
}

pub fn parse_input(input: &str) -> Vec<Vec<Day03Entry>> {
    input
        .trim()
        .lines()
        .map(|line: &str| parse_line(line).collect())
        .collect()
}

pub static PUZZLE_INPUT: &str = include_str!("../input");

pub mod benchmark {
    use super::*;

    pub type BenchmarkVector<'a> = Vec<
        Box<
            dyn AoC<'a, SolutionPart1 = Day03SolutionPart1, SolutionPart2 = Day03SolutionPart2>
                + 'a,
        >,
    >;

    pub fn to_benchmark<'a>() -> BenchmarkVector<'a> {
        vec![Box::new(Day03Initial::new(PUZZLE_INPUT))]
    }
}

#[cfg(test)]
mod tests {
    use env_logger;

    use super::*;

    pub fn init_logger() {
        let _ = env_logger::try_init();
    }

    #[test]
    fn parse() {
        init_logger();

        let input = "R75,D30,R83,U83,L12,D49,R71,U7,L72
        U62,R66,U55,R34,D71,R55,D58,R83";
        let parsed = parse_input(input);

        let expected = vec![
            vec![
                Day03Entry::Right(75),
                Day03Entry::Down(30),
                Day03Entry::Right(83),
                Day03Entry::Up(83),
                Day03Entry::Left(12),
                Day03Entry::Down(49),
                Day03Entry::Right(71),
                Day03Entry::Up(7),
                Day03Entry::Left(72),
            ],
            vec![
                Day03Entry::Up(62),
                Day03Entry::Right(66),
                Day03Entry::Up(55),
                Day03Entry::Right(34),
                Day03Entry::Down(71),
                Day03Entry::Right(55),
                Day03Entry::Down(58),
                Day03Entry::Right(83),
            ],
        ];

        assert_eq!(parsed, expected);
    }
}
