#[derive(Debug, Clone, Copy)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

impl TryFrom<char> for Direction {
    type Error = char;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '^' => Ok(Direction::North),
            '>' => Ok(Direction::East),
            'v' => Ok(Direction::South),
            '<' => Ok(Direction::West),
            _ => Err(value),
        }
    }
}

impl Direction {
    pub fn turn_ccw(&self) -> Direction {
        use Direction::*;
        match *self {
            North => West,
            East => North,
            South => East,
            West => South,
        }
    }

    pub fn turn_cw(&self) -> Direction {
        use Direction::*;
        match *self {
            North => East,
            East => South,
            South => West,
            West => North,
        }
    }
}
