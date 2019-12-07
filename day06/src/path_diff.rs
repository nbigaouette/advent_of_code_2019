use crate::*;

#[derive(Debug)]
pub struct Day06PathDiff<'a> {
    to_parent: Day06InverseMap<'a>,
    to_children: Day06Map<'a>,
}

impl<'a> AoC<'a> for Day06PathDiff<'a> {
    type SolutionPart1 = Day06SolutionPart1;
    type SolutionPart2 = Day06SolutionPart2;

    fn description(&self) -> &'static str {
        "Compare paths to root"
    }

    fn new(input: &'a str) -> Day06PathDiff<'_> {
        let (to_children, to_parent) = parse_input_part2(input);
        Day06PathDiff {
            to_parent,
            to_children,
        }
    }

    fn solution_part1(&self) -> Self::SolutionPart1 {
        // Same as Initial

        let all_bodies: Vec<Orbiter> = self.to_parent.0.keys().cloned().collect();

        let mut counter = 0;

        for body in all_bodies {
            let mut parent_opt: Option<Orbiter> = self.to_parent.get(&body).map(|o| o.as_orbiter());
            while let Some(parent) = parent_opt {
                counter += 1;
                parent_opt = self.to_parent.get(&parent).map(|o| o.as_orbiter().clone());
            }
        }

        counter
    }

    fn solution_part2(&self) -> Self::SolutionPart2 {
        const YOU: &str = "YOU";
        const SAN: &str = "SAN";

        let source = Orbiter(YOU);
        let target = Orbiter(SAN);

        let paths_to_root: Vec<Vec<Orbiter>> = [source, target]
            .iter()
            .map(|body| {
                let mut parent_opt: Option<Orbiter> =
                    self.to_parent.get(body).map(|o| o.as_orbiter());
                let mut path_to_root = Vec::new();
                while let Some(parent) = parent_opt {
                    parent_opt = self.to_parent.get(&parent).map(|o| o.as_orbiter().clone());
                    path_to_root.push(parent);
                }
                path_to_root
            })
            .collect();

        let diff_index = paths_to_root[0]
            .iter()
            .rev()
            .zip(paths_to_root[1].iter().rev())
            .position(|(from_you, from_santa)| from_you != from_santa)
            .unwrap();

        paths_to_root[0].iter().rev().skip(diff_index).count()
            + paths_to_root[1].iter().rev().skip(diff_index).count()
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

                let expected = 245089;
                let to_check = Day06PathDiff::new(PUZZLE_INPUT).solution_part1();

                assert_eq!(to_check, expected);
            }
        }

        mod given {
            use super::super::super::Day06PathDiff;
            use crate::{tests::init_logger, AoC};

            #[test]
            fn ex01() {
                init_logger();

                let expected = 42;
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
                let to_check = Day06PathDiff::new(input).solution_part1();

                assert_eq!(to_check, expected);
            }
        }

        /*
        mod extra {
            use super::super::super::Day06PathDiff;
            use crate::{tests::init_logger, AoC, PUZZLE_INPUT};
        }
        */
    }

    mod part2 {
        mod solution {
            use super::super::super::Day06PathDiff;
            use crate::{tests::init_logger, AoC, PUZZLE_INPUT};

            #[test]
            fn solution() {
                init_logger();

                let expected = 511;
                let to_check = Day06PathDiff::new(PUZZLE_INPUT).solution_part2();

                assert_eq!(to_check, expected);
            }
        }

        mod given {
            use super::super::super::Day06PathDiff;
            use crate::{tests::init_logger, AoC};

            #[test]
            fn ex01() {
                init_logger();

                let expected = 4;
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
                    K)L
                    K)YOU
                    I)SAN"#;
                let to_check = Day06PathDiff::new(input).solution_part2();

                assert_eq!(to_check, expected);
            }
        }

        /*
        mod extra {
            use super::super::super::Day06PathDiff;
            use crate::{tests::init_logger, AoC, PUZZLE_INPUT};
        }
        */
    }
}
