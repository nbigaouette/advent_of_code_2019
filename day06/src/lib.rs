//! # Day 06: Universal Orbit Map
//!
//! [GitHub source](https://github.com/nbigaouette/advent_of_code_2019/tree/master/day06)
//!
//! [Benchmarking report](../../../day06/target/criterion/report/index.html)
//!
//! ## Part One
//!
//! You've landed at the Universal Orbit Map facility on Mercury. Because navigation in space often
//! involves transferring between orbits, the orbit maps here are useful for finding efficient routes
//! between, for example, you and Santa. You download a map of the local orbits (your puzzle input).
//!
//! Except for the universal Center of Mass (`COM`), every object in space is in orbit around exactly
//! one other object. An [orbit](https://en.wikipedia.org/wiki/Orbit) looks roughly like this:
//!
//! ```text
//!                   \
//!                    \
//!                     |
//!                     |
//! AAA--> o            o <--BBB
//!                     |
//!                     |
//!                    /
//!                   /
//! ```
//!
//! In this diagram, the object `BBB` is in orbit around `AAA`. The path that `BBB` takes around `AAA`
//! (drawn with lines) is only partly shown. In the map data, this orbital relationship is written
//! `AAA)BBB`, which means "`BBB` is in orbit around `AAA`".
//!
//! Before you use your map data to plot a course, you need to make sure it wasn't corrupted during the
//! download. To verify maps, the Universal Orbit Map facility uses _orbit count checksums_ - the total
//! number of _direct orbits_ (like the one shown above) and _indirect orbits_.
//!
//! Whenever `A` orbits `B` and `B` orbits `C`, then `A` _indirectly orbits_ `C`. This chain can be any
//! number of objects long: if `A` orbits `B`, `B` orbits `C`, and `C` orbits `D`, then `A` indirectly
//! orbits `D`.
//!
//! For example, suppose you have the following map:
//!
//! ```text
//! COM)B
//! B)C
//! C)D
//! D)E
//! E)F
//! B)G
//! G)H
//! D)I
//! E)J
//! J)K
//! K)L
//! ```
//!
//! Visually, the above map of orbits looks like this:
//!
//! ```text
//!         G - H       J - K - L
//!        /           /
//! COM - B - C - D - E - F
//!                \
//!                 I
//! ```
//!
//! In this visual representation, when two objects are connected by a line, the one on the right
//! directly orbits the one on the left.
//!
//! Here, we can count the total number of orbits as follows:
//!
//! * `D` directly orbits `C` and indirectly orbits `B` and `COM`, a total of `3` orbits.
//! * `L` directly orbits `K` and indirectly orbits `J`, `E`, `D`, `C`, `B`, and `COM`,
//!   a total of `7` orbits.
//! * `COM` orbits nothing.
//!
//! The total number of direct and indirect orbits in this example is `_42_`.
//!
//! _What is the total number of direct and indirect orbits_ in your map data?
//!
//! ## Part Two
//!
//! Now, you just need to figure out how many _orbital transfers_ you (`YOU`) need to take to get to
//! Santa (`SAN`).
//!
//! You start at the object `YOU` are orbiting; your destination is the object `SAN` is orbiting. An
//! orbital transfer lets you move from any object to an object orbiting or orbited by that object.
//!
//! For example, suppose you have the following map:
//!
//! ```text
//! COM)B
//! B)C
//! C)D
//! D)E
//! E)F
//! B)G
//! G)H
//! D)I
//! E)J
//! J)K
//! K)L
//! K)YOU
//! I)SAN
//! ```
//!
//! Visually, the above map of orbits looks like this:
//!
//! ```text
//!                           YOU
//!                          /
//!         G - H       J - K - L
//!        /           /
//! COM - B - C - D - E - F
//!                \
//!                 I - SAN
//! ```
//!
//! In this example, `YOU` are in orbit around `K`, and `SAN` is in orbit around `I`. To move from `K`
//! to `I`, a minimum of `4` orbital transfers are required:
//!
//! * `K` to `J`
//! * `J` to `E`
//! * `E` to `D`
//! * `D` to `I`
//!
//! Afterward, the map of orbits looks like this:
//!
//! ```text
//!         G - H       J - K - L
//!        /           /
//! COM - B - C - D - E - F
//!                \
//!                 I - SAN
//!                  \
//!                   YOU
//! ```
//! _What is the minimum number of orbital transfers required_ to move from the object `YOU` are
//! orbiting to the object `SAN` is orbiting? (Between the objects they are orbiting - _not_ between
//! `YOU` and `SAN`.)
//!

use std::{collections::HashMap, fmt::Debug};

pub use anyhow::{Context, Result};
use shrinkwraprs::Shrinkwrap;

pub mod initial;
pub mod path_diff;
pub use crate::initial::Day06Initial;
pub use crate::path_diff::Day06PathDiff;

type Day06SolutionPart1 = usize;
type Day06SolutionPart2 = usize;

#[derive(Debug, Shrinkwrap, PartialEq)]
#[shrinkwrap(mutable)]
pub struct Day06InverseMap<'a>(pub HashMap<Orbiter<'a>, Orbiting<'a>>);

#[derive(Debug, Shrinkwrap, PartialEq)]
#[shrinkwrap(mutable)]
pub struct Day06Map<'a>(pub HashMap<Orbiting<'a>, Vec<Orbiter<'a>>>);

#[derive(Debug, Clone, Shrinkwrap, PartialEq, Eq, Hash)]
pub struct Orbiter<'a>(&'a str);

#[derive(Debug, Clone, Shrinkwrap, PartialEq, Eq, Hash)]
pub struct Orbiting<'a>(&'a str);

impl<'a> Day06InverseMap<'a> {
    pub fn new() -> Day06InverseMap<'a> {
        Day06InverseMap(HashMap::new())
    }
}

impl<'a> Day06Map<'a> {
    pub fn new() -> Day06Map<'a> {
        Day06Map(HashMap::new())
    }
}

impl<'a> Orbiting<'a> {
    fn as_orbiter(&'a self) -> Orbiter<'a> {
        Orbiter(self.0)
    }
    fn into_orbiter(self) -> Orbiter<'a> {
        Orbiter(self.0)
    }
}

impl<'a> Orbiter<'a> {
    fn into_orbiting(self) -> Orbiting<'a> {
        Orbiting(self.0)
    }
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

pub fn parse_input_part1<'a>(input: &'a str) -> Day06InverseMap<'a> {
    Day06InverseMap(
        input
            .trim()
            .lines()
            .filter_map(|line| {
                let mut objects = line.split(')');
                let orbiting = Orbiting(objects.next()?.trim());
                let orbiter = Orbiter(objects.next()?.trim());

                Some((orbiter, orbiting))
            })
            .collect(),
    )
}

pub fn parse_input_part2<'a>(input: &'a str) -> (Day06Map<'a>, Day06InverseMap<'a>) {
    let inverse_orb_map = parse_input_part1(input);

    let mut orb_map = Day06Map::new();
    input
        .trim()
        .lines()
        .filter_map(|line| {
            let mut objects = line.split(')');
            let orbiting = Orbiting(objects.next()?.trim());
            let orbiter = Orbiter(objects.next()?.trim());

            Some((orbiting, orbiter))
        })
        .for_each(|(orbiting, orbiter)| {
            let orbiting_ptr = orb_map.0.entry(orbiting.clone()).or_insert(Vec::new());
            orbiting_ptr.push(orbiter)
        });

    (orb_map, inverse_orb_map)
}

pub static PUZZLE_INPUT: &str = include_str!("../input");

pub mod benchmark {
    use super::*;

    pub type BenchmarkVector<'a> = Vec<
        Box<
            dyn AoC<'a, SolutionPart1 = Day06SolutionPart1, SolutionPart2 = Day06SolutionPart2>
                + 'a,
        >,
    >;

    pub fn to_benchmark<'a>() -> BenchmarkVector<'a> {
        vec![
            //
            Box::new(Day06Initial::new(PUZZLE_INPUT)),
            Box::new(Day06PathDiff::new(PUZZLE_INPUT)),
        ]
    }
}

#[cfg(test)]
mod tests {
    use std::env;

    use env_logger;
    use pretty_assertions::assert_eq;

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

        let input = r#"
            COM)B
            B)C
            C)D
            D)E
            E)F
            B)G
            G)H
            D)I
            E)J
            J)K
            K)L"#;

        let parsed = parse_input_part1(input);
        let mut expected = Day06InverseMap::new();
        expected.insert(Orbiter("B"), Orbiting("COM"));
        expected.insert(Orbiter("C"), Orbiting("B"));
        expected.insert(Orbiter("D"), Orbiting("C"));
        expected.insert(Orbiter("E"), Orbiting("D"));
        expected.insert(Orbiter("F"), Orbiting("E"));
        expected.insert(Orbiter("G"), Orbiting("B"));
        expected.insert(Orbiter("H"), Orbiting("G"));
        expected.insert(Orbiter("I"), Orbiting("D"));
        expected.insert(Orbiter("J"), Orbiting("E"));
        expected.insert(Orbiter("K"), Orbiting("J"));
        expected.insert(Orbiter("L"), Orbiting("K"));

        assert_eq!(parsed, expected);
    }
}
