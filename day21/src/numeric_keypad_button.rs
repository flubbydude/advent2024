#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NumericKeypadButton {
    Number(u8),
    Activate,
}

impl TryFrom<char> for NumericKeypadButton {
    type Error = char;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        if value == 'A' {
            Ok(NumericKeypadButton::Activate)
        } else if value.is_ascii_digit() {
            Ok(NumericKeypadButton::Number(value as u8 - b'0'))
        } else {
            Err(value)
        }
    }
}
