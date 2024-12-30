use crate::direction::Direction;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Instruction {
    Direction(Direction),
    Activate,
}

impl From<Direction> for Instruction {
    fn from(value: Direction) -> Self {
        Instruction::Direction(value)
    }
}
