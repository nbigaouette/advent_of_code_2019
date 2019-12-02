use crate::*;

#[derive(Debug)]
pub struct Day02Initial {
    initial_state: Vec<OpCode>,
    state: Vec<OpCode>,
    instruction_ptr: usize,
}

impl Day02Initial {
    pub fn state(&self) -> &[OpCode] {
        &self.state
    }

    fn reset(&mut self) {
        self.state = self.initial_state.clone();
        self.instruction_ptr = 0;
    }

    fn pointers(&self) -> (usize, usize, usize) {
        (
            self.state[self.instruction_ptr + 1],
            self.state[self.instruction_ptr + 2],
            self.state[self.instruction_ptr + 3],
        )
    }

    pub fn step(&mut self) {
        let opcode = self.state[self.instruction_ptr];
        match opcode {
            OPCODE_ADD => {
                let (address_left, address_right, address_result) = self.pointers();
                self.state[address_result] = self.state[address_left] + self.state[address_right];
            }
            OPCODE_MUL => {
                let (address_left, address_right, address_result) = self.pointers();
                self.state[address_result] = self.state[address_left] * self.state[address_right];
            }
            OPCODE_DONE => log::info!("Done!"),
            _ => unreachable!(),
        }
        self.instruction_ptr += OPCODE_JUMP;
    }

    pub fn run(&mut self) {
        while self.state[self.instruction_ptr] != OPCODE_DONE {
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
        let initial_state: Vec<usize> = parse_input(input).collect();
        Day02Initial {
            state: initial_state.clone(),
            initial_state,
            instruction_ptr: 0,
        }
    }

    fn solution_part1(&mut self) -> Self::SolutionPart1 {
        self.state[IDX_NOUN] = 12;
        self.state[IDX_VERB] = 2;

        self.run();

        self.state[IDX_OUTPUT]
    }

    fn solution_part2(&mut self) -> Self::SolutionPart2 {
        let mut found = false;
        'outer: for noun in 0..=99 {
            for verb in 0..=99 {
                // Reset computer to initial state
                self.reset();

                self.state[IDX_NOUN] = noun;
                self.state[IDX_VERB] = verb;

                self.run();

                if self.state[IDX_OUTPUT] == PART2_EXPECTED_VALUE {
                    log::info!("Found output {}", self.state[IDX_OUTPUT]);
                    found = true;
                    break 'outer;
                }
            }
        }

        assert!(found);

        let noun = self.state[IDX_NOUN];
        let verb = self.state[IDX_VERB];

        100 * noun + verb
    }
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

                let expected = 5741;
                let to_check = Day02Initial::new(PUZZLE_INPUT).solution_part2();

                assert_eq!(to_check, expected);
            }
        }
    }
}
