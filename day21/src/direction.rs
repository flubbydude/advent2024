use enum_iterator::Sequence;
use num_derive::FromPrimitive;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Sequence, FromPrimitive)]
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

fn move_once_checked(position: (usize, usize), direction: Direction) -> Option<(usize, usize)> {
    let (di, dj) = direction.as_tuple();
    Some((
        position.0.checked_add_signed(di)?,
        position.1.checked_add_signed(dj)?,
    ))
}

pub fn move_once_bounded(
    position: (usize, usize),
    direction: Direction,
    num_rows: usize,
    num_columns: usize,
) -> Option<(usize, usize)> {
    move_once_checked(position, direction).filter(|&(i, j)| i < num_rows && j < num_columns)
}
