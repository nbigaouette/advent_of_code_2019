use crate::*;

#[derive(Debug)]
pub struct Day06Initial<'a> {
    orb_inv_map: Day06InverseMap<'a>,
    orb_map: Day06Map<'a>,
}

impl<'a> AoC<'a> for Day06Initial<'a> {
    type SolutionPart1 = Day06SolutionPart1;
    type SolutionPart2 = Day06SolutionPart2;

    fn description(&self) -> &'static str {
        "Walk tree DFS"
    }

    fn new(input: &'a str) -> Day06Initial<'_> {
        let (orb_map, orb_inv_map) = parse_input_part2(input);
        Day06Initial {
            orb_inv_map,
            orb_map,
        }
    }

    fn solution_part1(&self) -> Self::SolutionPart1 {
        let all_bodies: Vec<Orbiter> = self.orb_inv_map.0.keys().cloned().collect();

        let mut counter = 0;

        for body in all_bodies {
            let mut parent_opt: Option<Orbiter> =
                self.orb_inv_map.get(&body).map(|o| o.as_orbiter());
            while let Some(parent) = parent_opt {
                counter += 1;
                parent_opt = self
                    .orb_inv_map
                    .get(&parent)
                    .map(|o| o.as_orbiter().clone());
            }
        }

        counter
    }

    fn solution_part2(&self) -> Self::SolutionPart2 {
        let mut children_map = self.orb_map.clone();
        let parent_map = self.orb_inv_map.clone();

        const YOU: &str = "YOU";
        const SAN: &str = "SAN";

        let source = Orbiter(YOU);
        let target = Orbiter(SAN);

        // Remove the source from the children map
        children_map
            .entry(parent_map[&source].clone())
            .and_modify(|e| e.retain(|child| child.0 != source.0));

        let mut currently_orbiting_opt: Option<Orbiting> = Some(parent_map[&source].clone());

        let mut counter_parent: usize = 0;
        let mut counter_child: usize = 0;
        let mut highest_visited = parent_map[&source].clone().into_orbiter();

        while let Some(currently_orbiting) = currently_orbiting_opt {
            if currently_orbiting.as_orbiter() == highest_visited {
                assert_eq!(counter_child, 0);
            }

            match children_map.get(&currently_orbiting) {
                Some(children) => {
                    if children.iter().any(|child| child == &target) {
                        currently_orbiting_opt = None;
                    } else {
                        match children_map.get_mut(&currently_orbiting).unwrap().pop() {
                            Some(child) => {
                                // Visit the first child
                                counter_child += 1;
                                currently_orbiting_opt = Some(child.into_orbiting());
                            }
                            None => {
                                // Currently orbiting body has no children; remove it from children_map
                                children_map.remove(&currently_orbiting);
                                // Remove it from the children of its parent too
                                let parent = &parent_map[&currently_orbiting.as_orbiter()];
                                children_map
                                    .get_mut(parent)
                                    .unwrap()
                                    .retain(|c| c != &currently_orbiting.as_orbiter());

                                if currently_orbiting.as_orbiter() == highest_visited {
                                    highest_visited = parent.clone().into_orbiter();
                                    counter_parent += 1;
                                }
                                counter_child = counter_child.saturating_sub(1);

                                currently_orbiting_opt =
                                    Some(parent_map[&currently_orbiting.as_orbiter()].clone());
                            }
                        }
                    }
                }
                None => {
                    if currently_orbiting.as_orbiter() == highest_visited {
                        highest_visited = currently_orbiting.clone().into_orbiter();
                        counter_parent += 1;
                    }
                    counter_child = counter_child.saturating_sub(1);
                    currently_orbiting_opt =
                        Some(parent_map[&currently_orbiting.as_orbiter()].clone());
                }
            }
        }

        counter_parent + counter_child
    }
}

#[cfg(test)]
mod tests {
    mod part1 {
        mod solution {
            use super::super::super::Day06Initial;
            use crate::{tests::init_logger, AoC, PUZZLE_INPUT};

            #[test]
            fn solution() {
                init_logger();

                let expected = 245089;
                let to_check = Day06Initial::new(PUZZLE_INPUT).solution_part1();

                assert_eq!(to_check, expected);
            }
        }

        mod given {
            use super::super::super::Day06Initial;
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
                let to_check = Day06Initial::new(input).solution_part1();

                assert_eq!(to_check, expected);
            }
        }

        /*
        mod extra {
            use super::super::super::Day06Initial;
            use crate::{tests::init_logger, AoC, PUZZLE_INPUT};
        }
        */
    }

    mod part2 {
        mod solution {
            use super::super::super::Day06Initial;
            use crate::{tests::init_logger, AoC, PUZZLE_INPUT};

            #[test]
            fn solution() {
                init_logger();

                let expected = 511;
                let to_check = Day06Initial::new(PUZZLE_INPUT).solution_part2();

                assert_eq!(to_check, expected);
            }
        }

        mod given {
            use super::super::super::Day06Initial;
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
                let to_check = Day06Initial::new(input).solution_part2();

                assert_eq!(to_check, expected);
            }
        }

        /*
        mod extra {
            use super::super::super::Day06Initial;
            use crate::{tests::init_logger, AoC, PUZZLE_INPUT};
        }
        */
    }
}
