use std::{char, collections::HashMap, iter::once};

use crate::{bounds::Bounds, robot::Robot};

pub struct PuzzleInput {
    bounds: Bounds,
    robots: Vec<Robot>,
}

impl PuzzleInput {
    pub fn new(num_rows: usize, num_columns: usize, robots: Vec<Robot>) -> Self {
        for robot in robots.iter() {
            assert!(
                robot.position.0 < num_rows && robot.position.1 < num_columns,
                "robot.position = {:?}",
                robot.position
            );
        }
        Self {
            bounds: Bounds {
                num_rows,
                num_columns,
            },
            robots,
        }
    }

    pub fn bounds(&self) -> &Bounds {
        &self.bounds
    }

    pub fn robots(&self) -> &Vec<Robot> {
        &self.robots
    }

    pub fn robots_mut(&mut self) -> &mut Vec<Robot> {
        &mut self.robots
    }

    pub fn board_as_str(&self) -> String {
        let mut num_robots = HashMap::new();
        for robot in self.robots.iter() {
            *num_robots.entry(robot.position).or_insert(0) += 1;
        }
        let num_robots = num_robots;
        let ref_num_robots = &num_robots;

        (0..self.bounds.num_rows)
            .flat_map(|i| {
                (0..self.bounds.num_columns)
                    .map(move |j| match ref_num_robots.get(&(i, j)) {
                        Some(&count) => {
                            if count < 36 {
                                char::from_digit(count, 36).unwrap()
                            } else {
                                '@'
                            }
                        }
                        None => '.',
                    })
                    .chain(once('\n'))
            })
            .collect()
    }
}
