#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum Color {
    White = b'w',
    Blue = b'u',
    Black = b'b',
    Red = b'r',
    Green = b'g',
}

impl TryFrom<u8> for Color {
    type Error = u8;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            b'w' => Ok(Color::White),
            b'u' => Ok(Color::Blue),
            b'b' => Ok(Color::Black),
            b'r' => Ok(Color::Red),
            b'g' => Ok(Color::Green),
            _ => Err(value),
        }
    }
}
