#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GridCell {
    Empty,
    Wall,
}

impl TryFrom<char> for GridCell {
    type Error = char;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(GridCell::Empty),
            '#' => Ok(GridCell::Wall),
            _ => Err(value),
        }
    }
}
