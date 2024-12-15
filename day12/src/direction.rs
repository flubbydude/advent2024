extern crate num_derive;

use enum_iterator::Sequence;
use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::{FromPrimitive, ToPrimitive};

#[derive(Debug, Clone, Copy, FromPrimitive, ToPrimitive, Sequence, PartialEq, Eq)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    pub fn turn_cw(&self) -> Direction {
        Direction::from_usize((self.to_usize().unwrap() + 1) % 4).unwrap()
    }

    pub fn turn_ccw(&self) -> Direction {
        Direction::from_usize((self.to_usize().unwrap() + 3) % 4).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::Direction;
    use enum_iterator::all;
    use num_traits::{FromPrimitive, ToPrimitive};

    #[test]
    fn test_sequence_value_order_as_expected() {
        assert_eq!(
            vec![0, 1, 2, 3],
            all::<Direction>()
                .map(|dir| dir.to_usize().unwrap())
                .collect::<Vec<_>>()
        );

        assert_eq!(
            all::<Direction>().collect::<Vec<_>>(),
            (0..4)
                .into_iter()
                .map(|i| Direction::from_usize(i).unwrap())
                .collect::<Vec<_>>()
        );
    }

    #[test]
    fn test_turn() {
        assert_eq!(Direction::North, Direction::West.turn_cw());
        assert_eq!(Direction::South, Direction::West.turn_ccw());

        assert_eq!(Direction::West, Direction::North.turn_ccw());
    }
}
