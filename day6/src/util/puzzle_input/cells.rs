#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MapCell {
    Obstacle,
    Empty,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InputCell {
    Obstacle,
    Empty,
    Start,
}

impl From<InputCell> for MapCell {
    fn from(value: InputCell) -> Self {
        match value {
            InputCell::Obstacle => MapCell::Obstacle,
            InputCell::Empty => MapCell::Empty,
            InputCell::Start => MapCell::Empty,
        }
    }
}
