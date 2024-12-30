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

impl TryFrom<char> for Instruction {
    type Error = char;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        if value == 'A' {
            Ok(Instruction::Activate)
        } else {
            Ok(Direction::try_from(value)?.into())
        }
    }
}
