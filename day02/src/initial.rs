use crate::*;

#[derive(Debug)]
pub struct Day02Initial {
    state: Vec<OpCode>,
    address: usize,
}

impl Day02Initial {
    pub fn state(&self) -> &[OpCode] {
        &self.state
    }
    fn addresses(&self) -> (usize, usize, usize) {
        (
            self.state[self.address + 1],
            self.state[self.address + 2],
            self.state[self.address + 3],
        )
    }
    pub fn step(&mut self) {
        let opcode = self.state[self.address];
        match opcode {
            OPCODE_ADD => {
                log::info!("OpCode ADD");
                let (address_left, address_right, address_result) = self.addresses();
                self.state[address_result] = self.state[address_left] + self.state[address_right];
            }
            OPCODE_MUL => {
                log::info!("OpCode MUL");
                let (address_left, address_right, address_result) = self.addresses();
                self.state[address_result] = self.state[address_left] * self.state[address_right];
            }
            OPCODE_DONE => log::info!("Done!"),
            _ => unreachable!(),
        }
        self.address += OPCODE_JUMP;
    }

    pub fn run(&mut self) {
        while self.state[self.address] != OPCODE_DONE {
            self.step();
        }
    }
}

impl AoC<'_> for Day02Initial {
    type SolutionPart1 = Day02SolutionPart1;
    type SolutionPart2 = Day02SolutionPart2;

    fn description(&self) -> &'static str {
        "Parse string dynamically"
    }

    fn new(input: &str) -> Day02Initial {
        Day02Initial {
            state: parse_input(input).collect(),
            address: 0,
        }
    }

    fn solution_part1(&mut self) -> Self::SolutionPart1 {
        self.state[1] = 12;
        self.state[2] = 2;

        self.run();

        self.state[0]
    }

    // fn solution_part2(&mut self) -> Self::SolutionPart2 {
    // }
}

#[cfg(test)]
mod tests {
    mod part1 {
        mod solution {
            use super::super::super::Day02Initial;
            use crate::{tests::init_logger, AoC, PUZZLE_INPUT};

            #[test]
            fn solution() {
                init_logger();

                let expected = 5290681;

                let mut computer = Day02Initial::new(PUZZLE_INPUT);

                let to_check = computer.solution_part1();

                assert_eq!(to_check, expected);
            }
        }

        mod given {
            use super::super::super::Day02Initial;
            use crate::{tests::init_logger, AoC};

            #[test]
            fn ex01() {
                init_logger();

                let input = "1,9,10,3,2,3,11,0,99,30,40,50";

                let mut computer = Day02Initial::new(input);

                computer.step();
                assert_eq!(
                    computer.state(),
                    &[1, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50]
                );

                computer.step();
                assert_eq!(
                    computer.state(),
                    &[3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50]
                );
            }

            #[test]
            fn ex02() {
                init_logger();

                let input = "1,0,0,0,99";
                let expected = &[2, 0, 0, 0, 99];

                let mut computer = Day02Initial::new(input);

                computer.run();

                let to_check = computer.state();
                assert_eq!(to_check, expected);
            }

            #[test]
            fn ex03() {
                init_logger();

                let input = "2,3,0,3,99";
                let expected = &[2, 3, 0, 6, 99];

                let mut computer = Day02Initial::new(input);

                computer.run();

                let to_check = computer.state();
                assert_eq!(to_check, expected);
            }

            #[test]
            fn ex04() {
                init_logger();

                let input = "2,4,4,5,99,0";
                let expected = &[2, 4, 4, 5, 99, 9801];

                let mut computer = Day02Initial::new(input);

                computer.run();

                let to_check = computer.state();
                assert_eq!(to_check, expected);
            }

            #[test]
            fn ex05() {
                init_logger();

                let input = "1,1,1,4,99,5,6,0,99";
                let expected = &[30, 1, 1, 4, 2, 5, 6, 0, 99];

                let mut computer = Day02Initial::new(input);

                computer.run();

                let to_check = computer.state();
                assert_eq!(to_check, expected);
            }
        }

        /*
        mod extra {
            use super::super::super::Day02Initial;
            use crate::{tests::init_logger, AoC, PUZZLE_INPUT};
        }
        */
    }

    mod part2 {
        mod solution {
            use super::super::super::Day02Initial;
            use crate::{tests::init_logger, AoC, PUZZLE_INPUT};

            #[test]
            fn solution() {
                init_logger();

                unimplemented!();

                let expected = 0;
                let to_check = Day02Initial::new(PUZZLE_INPUT).solution_part2();

                assert_eq!(to_check, expected);
            }
        }

        mod given {
            use super::super::super::Day02Initial;
            use crate::{tests::init_logger, AoC};

            #[test]
            fn ex01() {
                init_logger();

                unimplemented!();

                let expected = 0;
                let input = "";
                let to_check = Day02Initial::new(input).solution_part2();

                assert_eq!(to_check, expected);
            }
        }

        /*
        mod extra {
            use super::super::super::Day02Initial;
            use crate::{tests::init_logger, AoC, PUZZLE_INPUT};
        }
        */
    }
}
