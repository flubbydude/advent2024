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
}
