//! # Day 02: 1202 Program Alarm
//!
//! [Benchmarking report](../../../day02/target/criterion/report/index.html)
//!
//! ## Part One
//!
//! On the way to your [gravity assist](https://en.wikipedia.org/wiki/Gravity_assist) around the Moon,
//! your ship computer beeps angrily about a "[1202 program
//! alarm](https://www.hq.nasa.gov/alsj/a11/a11.landing.html#1023832)". On the radio, an Elf is already
//! explaining how to handle the situation: "Don't worry, that's perfectly norma--" The ship computer
//! [bursts into flames](https://en.wikipedia.org/wiki/Halt_and_Catch_Fire).
//!
//! You notify the Elves that the computer's [magic smoke](https://en.wikipedia.org/wiki/Magic_smoke)
//! seems to have escaped. "That computer ran _Intcode_ programs like the gravity assist program it was
//! working on; surely there are enough spare parts up there to build a new Intcode computer!"
//!
//! An Intcode program is a list of [integers](https://en.wikipedia.org/wiki/Integer) separated by
//! commas (like `1,0,0,3,99`). To run one, start by looking at the first integer (called position
//! `0`). Here, you will find an _opcode_ - either `1`, `2`, or `99`. The opcode indicates what to do;
//! for example, `99` means that the program is finished and should immediately halt. Encountering an
//! unknown opcode means something went wrong.
//!
//! Opcode `1` _adds_ together numbers read from two positions and stores the result in a third
//! position. The three integers _immediately after_ the opcode tell you these three positions - the
//! first two indicate the _positions_ from which you should read the input values, and the third
//! indicates the _position_ at which the output should be stored.
//!
//! For example, if your Intcode computer encounters `1,10,20,30`, it should read the values at
//! positions `10` and `20`, add those values, and then overwrite the value at position `30` with their
//! sum.
//!
//! Opcode `2` works exactly like opcode `1`, except it _multiplies_ the two inputs instead of adding
//! them. Again, the three integers after the opcode indicate _where_ the inputs and outputs are, not
//! their values.
//!
//! Once you're done processing an opcode, _move to the next one_ by stepping forward `4` positions.
//!
//! For example, suppose you have the following program:
//!
//! ```text
//! 1,9,10,3,2,3,11,0,99,30,40,50
//! ```
//!
//! For the purposes of illustration, here is the same program split into multiple lines:
//!
//! ```text
//! 1,9,10,3, 2,3,11,0, 99, 30,40,50
//! ```
//!
//! The first four integers, `1,9,10,3`, are at positions `0`, `1`, `2`, and `3`. Together, they
//! represent the first opcode (`1`, addition), the positions of the two inputs (`9` and `10`), and the
//! position of the output (`3`). To handle this opcode, you first need to get the values at the input
//! positions: position `9` contains `30`, and position `10` contains `40`. _Add_ these numbers
//! together to get `70`. Then, store this value at the output position; here, the output position
//! (`3`) is _at_ position `3`, so it overwrites itself. Afterward, the program looks like this:
//!
//! ```text
//!  1,9,10,70, 2,3,11,0, 99, 30,40,50
//! ```
//!
//! Step forward `4` positions to reach the next opcode, `2`. This opcode works just like the previous,
//! but it multiplies instead of adding. The inputs are at positions `3` and `11`; these positions
//! contain `70` and `50` respectively. Multiplying these produces `3500`; this is stored at position
//! `0`:
//!
//! ```text
//! 3500,9,10,70, 2,3,11,0, 99, 30,40,50
//! ```text
//!
//! Stepping forward `4` more positions arrives at opcode `99`, halting the program.
//!
//! Here are the initial and final states of a few more small programs:
//!
//! * `1,0,0,0,99` becomes `_2_,0,0,0,99` (`1 + 1 = 2`).
//! * `2,3,0,3,99` becomes `2,3,0,_6_,99` (`3 * 2 = 6`).
//! * `2,4,4,5,99,0` becomes `2,4,4,5,99,_9801_` (`99 * 99 = 9801`).
//! * `1,1,1,4,99,5,6,0,99` becomes `_30_,1,1,4,_2_,5,6,0,99`.
//!
//! Once you have a working computer, the first step is to restore the gravity assist program (your
//! puzzle input) to the "1202 program alarm" state it had just before the last computer caught fire.
//! To do this, _before running the program_, replace position `1` with the value `12` and replace
//! position `2` with the value `2`. _What value is left at position `0`_ after the program halts?
//!
//! ## Part Two
//!
//! "Good, the new computer seems to be working correctly! _Keep it nearby_ during this mission -
//! you'll probably use it again. Real Intcode computers support many more features than your new one,
//! but we'll let you know what they are as you need them."
//!
//! "However, your current priority should be to complete your gravity assist around the Moon. For this
//! mission to succeed, we should settle on some terminology for the parts you've already built."
//!
//! Intcode programs are given as a list of integers; these values are used as the initial state for
//! the computer's _memory_. When you run an Intcode program, make sure to start by initializing memory
//! to the program's values. A position in memory is called an _address_ (for example, the first value
//! in memory is at "address 0").
//!
//! Opcodes (like `1`, `2`, or `99`) mark the beginning of an _instruction_. The values used
//! immediately after an opcode, if any, are called the instruction's _parameters_. For example, in the
//! instruction `1,2,3,4`, `1` is the opcode; `2`, `3`, and `4` are the parameters. The instruction
//! `99` contains only an opcode and has no parameters.
//!
//! The address of the current instruction is called the _instruction pointer_; it starts at `0`. After
//! an instruction finishes, the instruction pointer increases by _the number of values in the
//! instruction_; until you add more instructions to the computer, this is always `4` (`1` opcode + `3`
//! parameters) for the add and multiply instructions. (The halt instruction would increase the
//! instruction pointer by `1`, but it halts the program instead.)
//!
//! "With terminology out of the way, we're ready to proceed. To complete the gravity assist, you need
//! to _determine what pair of inputs produces the output `19690720`_."
//!
//! The inputs should still be provided to the program by replacing the values at addresses `1` and
//! `2`, just like before. In this program, the value placed in address `1` is called the _noun_, and
//! the value placed in address `2` is called the _verb_. Each of the two input values will be between
//! `0` and `99`, inclusive.
//!
//! Once the program has halted, its output is available at address `0`, also just like before. Each
//! time you try a pair of inputs, make sure you first _reset the computer's memory to the values in
//! the program_ (your puzzle input) - in other words, don't reuse memory from a previous attempt.
//!
//! Find the input _noun_ and _verb_ that cause the program to produce the output `19690720`. _What is
//! `100 * noun + verb`?_ (For example, if `noun=12` and `verb=2`, the answer would be `1202`.)

use std::fmt::Debug;

pub mod initial;
pub use crate::initial::Day02Initial;

pub type OpCode = usize;

pub const OPCODE_ADD: usize = 1;
pub const OPCODE_MUL: usize = 2;
pub const OPCODE_DONE: usize = 99;
// Anything else is 'unknown'
pub const OPCODE_JUMP: usize = 4;

pub const IDX_OUTPUT: usize = 0;
pub const IDX_NOUN: usize = 1;
pub const IDX_VERB: usize = 2;
pub const PART2_EXPECTED_VALUE: usize = 19_690_720;

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
        vec![Box::new(Day02Initial::new(PUZZLE_INPUT))]
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
