use crate::*;

#[derive(Debug)]
pub struct Day01Initial<'a> {
    input: &'a str,
}

#[derive(Debug)]
pub struct Day01Part2WeightIterator {
    remaining_weight: usize,
    total_weight: usize,
}

impl Day01Part2WeightIterator {
    pub fn new(module: &Day01Entry) -> Day01Part2WeightIterator {
        Day01Part2WeightIterator {
            remaining_weight: module.0,
            total_weight: 0,
        }
    }
}

impl Iterator for Day01Part2WeightIterator {
    type Item = Day01SolutionPart2;

    fn next(&mut self) -> Option<Self::Item> {
        let new_weight = weight(self.remaining_weight);
        self.total_weight += new_weight;
        self.remaining_weight = new_weight;
        if self.remaining_weight == 0 {
            None
        } else {
            Some(self.remaining_weight)
        }
    }
}

impl<'a> AoC<'a> for Day01Initial<'a> {
    type SolutionPart1 = Day01SolutionPart1;
    type SolutionPart2 = Day01SolutionPart2;

    fn description(&self) -> &'static str {
        "Parse string dynamically"
    }

    fn new(input: &'a str) -> Day01Initial<'_> {
        Day01Initial { input }
    }

    fn solution_part1(&self) -> Self::SolutionPart1 {
        parse_input(self.input).map(|module| module.weight()).sum()
    }

    fn solution_part2(&self) -> Self::SolutionPart2 {
        parse_input(self.input)
            .map(|module| Day01Part2WeightIterator::new(&module).sum::<usize>())
            .sum()
    }
}

#[cfg(test)]
mod tests {
    mod part1 {
        mod solution {
            use super::super::super::Day01Initial;
            use crate::{tests::init_logger, AoC, PUZZLE_INPUT};

            #[test]
            fn solution() {
                init_logger();

                let expected = 3382284;
                let to_check = Day01Initial::new(PUZZLE_INPUT).solution_part1();

                assert_eq!(to_check, expected);
            }
        }

        mod given {
            use super::super::super::Day01Initial;
            use crate::{tests::init_logger, AoC};

            #[test]
            fn ex01() {
                init_logger();

                let expected = 34_241;
                let input = "12
                14
                1969
                100756";
                let to_check = Day01Initial::new(input).solution_part1();

                assert_eq!(to_check, expected);
            }
        }
    }

    mod part2 {
        mod solution {
            use super::super::super::Day01Initial;
            use crate::{tests::init_logger, AoC, PUZZLE_INPUT};

            #[test]
            fn solution() {
                init_logger();

                let expected = 5070541;
                let to_check = Day01Initial::new(PUZZLE_INPUT).solution_part2();

                assert_eq!(to_check, expected);
            }
        }

        mod given {
            use super::super::super::Day01Initial;
            use crate::{tests::init_logger, AoC};

            #[test]
            fn ex01() {
                init_logger();

                let input = "14";
                let expected = 2;
                let to_check = Day01Initial::new(input).solution_part2();

                assert_eq!(to_check, expected);
            }

            #[test]
            fn ex02() {
                init_logger();

                let input = "1969";
                let expected = 966;
                let to_check = Day01Initial::new(input).solution_part2();

                assert_eq!(to_check, expected);
            }

            #[test]
            fn ex03() {
                init_logger();

                let input = "100756";
                let expected = 50346;
                let to_check = Day01Initial::new(input).solution_part2();

                assert_eq!(to_check, expected);
            }
        }

        /*
        mod extra {
            use super::super::super::Day01Initial;
            use crate::{tests::init_logger, AoC, PUZZLE_INPUT};
        }
        */
    }
}
