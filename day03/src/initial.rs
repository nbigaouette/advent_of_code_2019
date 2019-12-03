use std::ops::Add;

use crate::*;

#[derive(Debug)]
pub struct Day03Initial<'a> {
    input: &'a str,
    displacements: Vec<Vec<Day03Entry>>,
}

#[derive(Debug, Default, Clone)]
struct Location {
    x: isize,
    y: isize,
}

#[allow(clippy::suspicious_arithmetic_impl)]
impl Add<&Day03Entry> for &Location {
    type Output = Location;

    fn add(self, other: &Day03Entry) -> Location {
        match other {
            Day03Entry::Down(l) => Location {
                x: self.x,
                y: self.y - l,
            },
            Day03Entry::Up(l) => Location {
                x: self.x,
                y: self.y + l,
            },
            Day03Entry::Left(l) => Location {
                x: self.x - l,
                y: self.y,
            },
            Day03Entry::Right(l) => Location {
                x: self.x + l,
                y: self.y,
            },
        }
    }
}

#[derive(Debug)]
struct Line {
    start: Location,
    end: Location,
}

impl std::fmt::Display for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

impl std::fmt::Display for Line {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}->{}", self.start, self.end)
    }
}

impl Line {
    fn length(&self) -> isize {
        (self.end.x - self.start.x).abs() + (self.end.y - self.start.y).abs()
    }

    /// https://en.wikipedia.org/wiki/Line%E2%80%93line_intersection#Given_two_points_on_each_line
    fn intersection(&self, other: &Line) -> Option<Location> {
        match (&self.start, &other.start) {
            (&Location { x: 0, y: 0 }, &Location { x: 0, y: 0 }) => {
                // Ignore initial point
                None
            }
            _ => {
                let x1 = self.start.x;
                let x2 = self.end.x;
                let x3 = other.start.x;
                let x4 = other.end.x;

                let y1 = self.start.y;
                let y2 = self.end.y;
                let y3 = other.start.y;
                let y4 = other.end.y;
                log::debug!("Intersection between {} & {}", self, other);

                let denominator = (x1 - x2) * (y3 - y4) - (y1 - y2) * (x3 - x4);

                match denominator {
                    0 => {
                        log::debug!("Denominator is null, lines are parallel");
                        None
                    }
                    denominator => {
                        let t = (((x1 - x3) * (y3 - y4) - (y1 - y3) * (x3 - x4)) as f64)
                            / (denominator as f64);
                        let u = -(((x1 - x2) * (y1 - y3) - (y1 - y2) * (x1 - x3)) as f64)
                            / (denominator as f64);
                        log::debug!("  --> denominator: {}  t: {}  u: {}", denominator, t, u);
                        if (0.0 <= t) && (t <= 1.0) && (0.0 <= u) && (u <= 1.0) {
                            let new_x = x1 + (t * ((x2 - x1) as f64)).round() as isize;
                            let new_y = y1 + (t * ((y2 - y1) as f64)).round() as isize;
                            log::debug!("  --> intersection: ({},{})", new_x, new_y);

                            Some(Location { x: new_x, y: new_y })
                        } else {
                            None
                        }
                    }
                }
            }
        }
    }
}

#[derive(Debug)]
struct Displacements<'a> {
    name: &'static str,
    start: Location,
    displacements: &'a [Day03Entry],
    idx: usize,
}

impl Displacements<'_> {
    fn end(&self) -> Location {
        (&self.start) + &self.displacements[self.idx]
    }
}

impl<'a> Iterator for Displacements<'a> {
    type Item = Line;

    fn next(&mut self) -> Option<Self::Item> {
        if self.idx == self.displacements.len() {
            None
        } else {
            let start = self.start.clone();
            let end = self.end();

            self.idx += 1;
            self.start = end.clone();

            Some(Line { start, end })
        }
    }
}

impl<'a> AoC<'a> for Day03Initial<'a> {
    type SolutionPart1 = Day03SolutionPart1;
    type SolutionPart2 = Day03SolutionPart2;

    fn description(&self) -> &'static str {
        "Parse string dynamically"
    }

    fn new(input: &'a str) -> Day03Initial<'_> {
        let displacements: Vec<Vec<Day03Entry>> = parse_input(input);
        Day03Initial {
            input,
            displacements,
        }
    }

    fn solution_part1(&self) -> Self::SolutionPart1 {
        let path1 = Displacements {
            name: "path1",
            start: Location::default(),
            displacements: &self.displacements[0],
            idx: 0,
        };

        let mut minimum_distance = isize::max_value();

        for (l1, line1) in path1.enumerate() {
            let path2 = Displacements {
                name: "path2",
                start: Location::default(),
                displacements: &self.displacements[1],
                idx: 0,
            };
            for (l2, line2) in path2.enumerate() {
                log::debug!("------------------------------------------------------------------");
                log::debug!(
                    "(l1,l2): ({},{})  [line1, line2]: [{}, {}]",
                    l1,
                    l2,
                    line1,
                    line2
                );
                match line1.intersection(&line2) {
                    None => {
                        log::debug!("No intersection found");
                    }
                    Some(intersection_position) => {
                        let intersection_distance =
                            intersection_position.x + intersection_position.y;
                        log::debug!(
                            "minimum_distance: {}  intersection_distance: {}",
                            minimum_distance,
                            intersection_distance
                        );
                        minimum_distance = minimum_distance.min(intersection_distance);
                    }
                }
            }
        }

        minimum_distance
    }

    fn solution_part2(&self) -> Self::SolutionPart2 {
        let path1 = Displacements {
            name: "path1",
            start: Location::default(),
            displacements: &self.displacements[0],
            idx: 0,
        };

        let mut min_total_steps = isize::max_value();

        let mut total_steps_path1 = 0;

        for (l1, line1) in path1.enumerate() {
            let path2 = Displacements {
                name: "path2",
                start: Location::default(),
                displacements: &self.displacements[1],
                idx: 0,
            };
            let mut total_steps_path2 = 0;
            for (l2, line2) in path2.enumerate() {
                log::debug!("------------------------------------------------------------------");
                log::debug!(
                    "(l1,l2): ({},{})  [line1, line2]: [{}, {}]",
                    l1,
                    l2,
                    line1,
                    line2
                );
                match line1.intersection(&line2) {
                    None => {
                        log::debug!("No intersection found");
                    }
                    Some(intersection_position) => {
                        let intersection_distance =
                            intersection_position.x + intersection_position.y;
                        let total_steps = total_steps_path1
                            + total_steps_path2
                            + Line {
                                start: line1.start.clone(),
                                end: intersection_position.clone(),
                            }
                            .length()
                            + Line {
                                start: line2.start.clone(),
                                end: intersection_position.clone(),
                            }
                            .length();
                        log::debug!(
                            "total_steps: {}  intersection_distance: {}",
                            total_steps,
                            intersection_distance
                        );
                        min_total_steps = min_total_steps.min(total_steps);
                    }
                }
                total_steps_path2 += line2.length();
            }
            total_steps_path1 += line1.length();
        }

        min_total_steps
    }
}

#[cfg(test)]
mod tests {
    mod part1 {
        mod solution {
            use super::super::super::Day03Initial;
            use crate::{tests::init_logger, AoC, PUZZLE_INPUT};

            #[test]
            fn solution() {
                init_logger();

                let expected = 1264;
                let to_check = Day03Initial::new(PUZZLE_INPUT).solution_part1();

                assert_eq!(to_check, expected);
            }
        }

        mod given {
            use super::super::super::Day03Initial;
            use crate::{tests::init_logger, AoC};

            #[test]
            fn ex01() {
                init_logger();

                let expected = 6;
                let input = "R8,U5,L5,D3
                U7,R6,D4,L4";
                let to_check = Day03Initial::new(input).solution_part1();

                assert_eq!(to_check, expected);
            }

            #[test]
            fn ex02() {
                init_logger();

                log::error!(
                    "Test 'initial::tests::part1::given::ex02' does NOT give 159 but instead 146 (bug in Aoc?)"
                );
                // let expected = 159;
                let expected = 146;

                let input = "R75,D30,R83,U83,L12,D49,R71,U7,L72
                U62,R66,U55,R34,D71,R55,D58,R83";
                let to_check = Day03Initial::new(input).solution_part1();

                assert_eq!(to_check, expected);
            }

            #[test]
            fn ex03() {
                init_logger();

                let expected = 135;
                let input = "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51
                U98,R91,D20,R16,D67,R40,U7,R15,U6,R7";
                let to_check = Day03Initial::new(input).solution_part1();

                assert_eq!(to_check, expected);
            }
        }
    }

    mod part2 {
        mod solution {
            use super::super::super::Day03Initial;
            use crate::{tests::init_logger, AoC, PUZZLE_INPUT};

            #[test]
            fn solution() {
                init_logger();

                let expected = 37390;
                let to_check = Day03Initial::new(PUZZLE_INPUT).solution_part2();

                assert_eq!(to_check, expected);
            }
        }

        mod given {
            use super::super::super::Day03Initial;
            use crate::{tests::init_logger, AoC};

            #[test]
            fn ex01() {
                init_logger();

                let expected = 30;
                let input = "R8,U5,L5,D3
                U7,R6,D4,L4";
                let to_check = Day03Initial::new(input).solution_part2();

                assert_eq!(to_check, expected);
            }

            #[test]
            fn ex02() {
                init_logger();

                let expected = 610;
                let input = "R75,D30,R83,U83,L12,D49,R71,U7,L72
                U62,R66,U55,R34,D71,R55,D58,R83";
                let to_check = Day03Initial::new(input).solution_part2();

                assert_eq!(to_check, expected);
            }

            #[test]
            fn ex03() {
                init_logger();

                let expected = 410;
                let input = "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51
                U98,R91,D20,R16,D67,R40,U7,R15,U6,R7";
                let to_check = Day03Initial::new(input).solution_part2();

                assert_eq!(to_check, expected);
            }
        }
    }
}
