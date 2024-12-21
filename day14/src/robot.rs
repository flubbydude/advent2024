use once_cell::sync::Lazy;
use regex::Regex;

use crate::{bounds::Bounds, quadrant::Quadrant};

#[derive(Debug, Clone)]
pub struct Robot {
    pub position: (usize, usize),
    pub velocity: (isize, isize),
}

impl From<&str> for Robot {
    fn from(value: &str) -> Self {
        static ROBOT_REGEX: Lazy<Regex> = Lazy::new(|| {
            Regex::new(r"^p=([0-9]{1,3}),([0-9]{1,3}) v=(-?[0-9]{1,3}),(-?[0-9]{1,3})$").unwrap()
        });

        let (_, [p1, p0, v1, v0]) = ROBOT_REGEX.captures(value).expect(value).extract();

        Robot {
            position: (p0.parse().unwrap(), p1.parse().unwrap()),
            velocity: (v0.parse().unwrap(), v1.parse().unwrap()),
        }
    }
}

impl Robot {
    pub fn step(&mut self, bounds: &Bounds) {
        self.position.0 = (self.position.0 + bounds.num_rows).wrapping_add_signed(self.velocity.0)
            % bounds.num_rows;
        self.position.1 = (self.position.1 + bounds.num_columns)
            .wrapping_add_signed(self.velocity.1)
            % bounds.num_columns;
    }

    pub fn get_quadrant(&self, bounds: &Bounds) -> Option<Quadrant> {
        if self.position.0 * 2 + 1 == bounds.num_rows
            || self.position.1 * 2 + 1 == bounds.num_columns
        {
            None
        } else {
            let is_left = self.position.0 < bounds.num_rows / 2;
            let is_top = self.position.1 < bounds.num_columns / 2;

            let quadrant = match (is_left, is_top) {
                (true, true) => Quadrant::TopLeft,
                (true, false) => Quadrant::BottomLeft,
                (false, true) => Quadrant::TopRight,
                (false, false) => Quadrant::BottomRight,
            };

            Some(quadrant)
        }
    }
}
