#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GridCell {
    Empty,
    Antenna { frequency: u8 },
}

impl TryFrom<char> for GridCell {
    type Error = &'static str;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        if value.is_ascii_alphanumeric() {
            Ok(GridCell::Antenna {
                frequency: value as u8,
            })
        } else if value == '.' {
            Ok(GridCell::Empty)
        } else {
            Err("Failed parsing {value} into GridCell")
        }
    }
}
