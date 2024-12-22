use crate::direction::Direction;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ReindeerState {
    position: (usize, usize),
    direction: Direction,
}

impl ReindeerState {
    pub fn new(position: (usize, usize), direction: Direction) -> Self {
        Self {
            position,
            direction,
        }
    }

    pub fn moved_forward_one(&self) -> ReindeerState {
        let new_position = match self.direction {
            Direction::North => (self.position.0 - 1, self.position.1),
            Direction::East => (self.position.0, self.position.1 + 1),
            Direction::South => (self.position.0 + 1, self.position.1),
            Direction::West => (self.position.0, self.position.1 - 1),
        };

        ReindeerState {
            position: new_position,
            direction: self.direction,
        }
    }

    pub fn turned_ccw(&self) -> ReindeerState {
        ReindeerState {
            position: self.position,
            direction: self.direction.turned_ccw(),
        }
    }

    pub fn turned_cw(&self) -> ReindeerState {
        ReindeerState {
            position: self.position,
            direction: self.direction.turned_cw(),
        }
    }

    pub fn position(&self) -> (usize, usize) {
        self.position
    }
}
