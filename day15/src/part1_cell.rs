#[derive(Debug, Clone, Copy)]
pub enum Part1Cell {
    Empty,
    Box,
    Wall,
}

impl TryFrom<char> for Part1Cell {
    type Error = char;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(Part1Cell::Empty),
            'O' => Ok(Part1Cell::Box),
            '#' => Ok(Part1Cell::Wall),
            _ => Err(value),
        }
    }
}
