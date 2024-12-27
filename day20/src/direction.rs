use enum_iterator::Sequence;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Sequence)]
#[repr(u8)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    pub fn as_tuple(&self) -> (isize, isize) {
        match self {
            Direction::North => (-1, 0),
            Direction::East => (0, 1),
            Direction::South => (1, 0),
            Direction::West => (0, -1),
        }
    }
}
