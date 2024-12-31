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

    pub fn opposite(&self) -> Direction {
        match self {
            Direction::North => Direction::South,
            Direction::East => Direction::West,
            Direction::South => Direction::North,
            Direction::West => Direction::East,
        }
    }
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

impl From<Direction> for char {
    fn from(value: Direction) -> Self {
        match value {
            Direction::North => '^',
            Direction::East => '>',
            Direction::South => 'v',
            Direction::West => '<',
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
