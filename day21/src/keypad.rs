use array2d::Array2D;
use once_cell::sync::Lazy;

use crate::{
    direction::{move_once_bounded, Direction},
    instruction::Instruction,
};

#[derive(Debug, Clone)]
pub struct Keypad<T>(Array2D<Option<T>>);

impl Keypad<Instruction> {
    pub fn get_instruction_keypad() -> &'static Self {
        &INSTRUCTION_KEYPAD
    }
}

impl Keypad<u8> {
    pub fn get_numeric_keypad() -> &'static Self {
        &NUMERIC_KEYPAD
    }
}

impl<T: Eq> Keypad<T> {
    pub fn position_of(&self, key: &T) -> (usize, usize) {
        self.0
            .enumerate_row_major()
            .find(|(_, button)| button.as_ref() == Some(key))
            .unwrap()
            .0
    }

    pub fn get(&self, position: (usize, usize)) -> Option<&T> {
        self.0[position].as_ref()
    }

    pub fn get_successor(
        &self,
        position: (usize, usize),
        direction: Direction,
    ) -> Option<(usize, usize)> {
        let new_position =
            move_once_bounded(position, direction, self.0.num_rows(), self.0.num_columns())?;
        if self.0[new_position].is_some() {
            Some(new_position)
        } else {
            None
        }
    }
}

impl<T> From<Array2D<Option<T>>> for Keypad<T> {
    fn from(value: Array2D<Option<T>>) -> Self {
        Keypad(value)
    }
}

static INSTRUCTION_KEYPAD: Lazy<Keypad<Instruction>> = Lazy::new(|| {
    use Direction::*;
    Array2D::from_rows(&[
        vec![None, Some(North.into()), Some(Instruction::Activate)],
        vec![Some(West.into()), Some(South.into()), Some(East.into())],
    ])
    .unwrap()
    .into()
});

static NUMERIC_KEYPAD: Lazy<Keypad<u8>> = Lazy::new(|| {
    Array2D::from_iter_row_major(
        b"789456123 0A"
            .iter()
            .map(|&c| if c == b' ' { None } else { Some(c) }),
        4,
        3,
    )
    .unwrap()
    .into()
});
