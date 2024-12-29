use crate::direction::Direction;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Instruction {
    Direction(Direction),
    Activate,
}
