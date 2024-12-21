use enum_iterator::Sequence;

#[derive(Debug, Sequence)]
pub enum Quadrant {
    TopLeft,
    BottomLeft,
    BottomRight,
    TopRight,
}
