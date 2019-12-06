use crate::*;

#[derive(Debug)]
pub struct Day04Initial {
    input: Day04Input,
}

impl AoC for Day04Initial {
    type SolutionPart1 = Day04SolutionPart1;
    type SolutionPart2 = Day04SolutionPart2;

    fn description(&self) -> &'static str {
        "Parse string dynamically"
    }

    fn new(input: &str) -> Day04Initial {
        Day04Initial {
            input: parse_input(input).unwrap(),
        }
    }

    fn solution_part1(&self) -> Self::SolutionPart1 {
        let mut current_password: Password = self.input.start.clone();

        let mut counter = 0;

        while current_password.value <= self.input.stop_inc.value {
            if current_password.is_valid_part1() {
                counter += 1;
            }

            current_password.value += 1;
            current_password.digits[PASSWORD_LENGTH - 1] += 1;
            for idx in 0..PASSWORD_LENGTH {
                let rev_idx = PASSWORD_LENGTH - idx - 1;
                if current_password.digits[rev_idx] == 10 {
                    current_password.digits[rev_idx] = 0;
                    // Hopefully the first digit is never 9, else this will access element -1 and panic
                    current_password.digits[rev_idx - 1] += 1;
                }
            }
        }

        counter
    }

    fn solution_part2(&self) -> Self::SolutionPart2 {
        let mut current_password: Password = self.input.start.clone();

        let mut counter = 0;

        while current_password.value <= self.input.stop_inc.value {
            if current_password.is_valid_part2() {
                counter += 1;
            }

            current_password.value += 1;
            current_password.digits[PASSWORD_LENGTH - 1] += 1;
            for idx in 0..PASSWORD_LENGTH {
                let rev_idx = PASSWORD_LENGTH - idx - 1;
                if current_password.digits[rev_idx] == 10 {
                    current_password.digits[rev_idx] = 0;
                    // Hopefully the first digit is never 9, else this will access element -1 and panic
                    current_password.digits[rev_idx - 1] += 1;
                }
            }
        }

        counter
    }
}

#[cfg(test)]
mod tests {
    mod part1 {
        mod solution {
            use super::super::super::*;
            use crate::{tests::init_logger, AoC, PUZZLE_INPUT};

            #[test]
            fn solution() {
                init_logger();

                let expected = 1650;
                let to_check = Day04Initial::new(PUZZLE_INPUT).solution_part1();

                assert_eq!(to_check, expected);
            }
        }

        mod given {
            use super::super::super::*;
            use crate::tests::init_logger;

            #[test]
            fn ex01() {
                init_logger();

                let input = "111111";
                let expected = true;

                let password = Password::try_from(input).unwrap();
                let to_check = password.is_valid_part1();

                assert_eq!(to_check, expected);
            }

            #[test]
            fn ex02() {
                init_logger();

                let input = "223450";
                let expected = false;

                let password = Password::try_from(input).unwrap();
                let to_check = password.is_valid_part1();

                assert_eq!(to_check, expected);
            }

            #[test]
            fn ex03() {
                init_logger();

                let input = "123789";
                let expected = false;

                let password = Password::try_from(input).unwrap();
                let to_check = password.is_valid_part1();

                assert_eq!(to_check, expected);
            }
        }

        /*
        mod extra {
            use super::super::super::Day04Initial;
            use crate::{tests::init_logger, AoC, PUZZLE_INPUT};
        }
        */
    }

    mod part2 {
        mod solution {
            use super::super::super::Day04Initial;
            use crate::{tests::init_logger, AoC, PUZZLE_INPUT};

            #[test]
            fn solution() {
                init_logger();

                let expected = 1129;
                let to_check = Day04Initial::new(PUZZLE_INPUT).solution_part2();

                assert_eq!(to_check, expected);
            }
        }

        mod given {
            use super::super::super::*;
            use crate::tests::init_logger;

            #[test]
            fn ex01() {
                init_logger();

                let input = "112233";
                let expected = true;

                let password = Password::try_from(input).unwrap();
                let to_check = password.is_valid_part2();

                assert_eq!(to_check, expected);
            }

            #[test]
            fn ex02() {
                init_logger();

                let input = "123444";
                let expected = false;

                let password = Password::try_from(input).unwrap();
                let to_check = password.is_valid_part2();

                assert_eq!(to_check, expected);
            }

            #[test]
            fn ex03() {
                init_logger();

                let input = "111122";
                let expected = true;

                let password = Password::try_from(input).unwrap();
                let to_check = password.is_valid_part2();

                assert_eq!(to_check, expected);
            }
        }

        /*
        mod extra {
            use super::super::super::Day04Initial;
            use crate::{tests::init_logger, AoC, PUZZLE_INPUT};
        }
        */
    }
}
