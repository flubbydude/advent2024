use crate::direction::Direction;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Part1State {
    position: (usize, usize),
    direction: Direction,
}

impl Part1State {
    pub fn new(position: (usize, usize), direction: Direction) -> Self {
        Self {
            position,
            direction,
        }
    }

    pub fn moved_forward_one(&self) -> Part1State {
        let new_position = match self.direction {
            Direction::North => (self.position.0 - 1, self.position.1),
            Direction::East => (self.position.0, self.position.1 + 1),
            Direction::South => (self.position.0 + 1, self.position.1),
            Direction::West => (self.position.0, self.position.1 - 1),
        };

        Part1State {
            position: new_position,
            direction: self.direction,
        }
    }

    pub fn turned_ccw(&self) -> Part1State {
        Part1State {
            position: self.position,
            direction: self.direction.turned_ccw(),
        }
    }

    pub fn turned_cw(&self) -> Part1State {
        Part1State {
            position: self.position,
            direction: self.direction.turned_cw(),
        }
    }

    pub fn position(&self) -> (usize, usize) {
        self.position
    }
}
