use std::iter::once;

use array2d::Array2D;
use itertools::Itertools;
use memoize::memoize;
use once_cell::sync::Lazy;

use crate::{
    direction::Direction,
    instruction::Instruction,
    shortest_paths::{iter_positions, BestPaths},
};

#[derive(Debug, Clone)]
pub struct Keypad<T>(Array2D<Option<T>>);

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

    // assume the best solution contains only shortest paths for each robot
    // and the best solution contains only shortest paths with at most 1 turn
    fn get_best_paths(
        &self,
        current_position: (usize, usize),
        target_position: (usize, usize),
    ) -> Vec<impl IntoIterator<Item = Direction>> {
        BestPaths::with_at_most_one_turn(current_position, target_position)
            .into_iter()
            .filter(move |path| {
                iter_positions(path, current_position)
                    .into_iter()
                    .all(|pos| self.get(pos).is_some())
            })
            .collect()
    }

    // return possible best instructions needed to input next_input
    // starting from the location of prev_input
    fn get_successors(
        &self,
        prev_input: &T,
        next_input: &T,
    ) -> impl IntoIterator<Item = Vec<Instruction>> {
        let prev_position = self.position_of(prev_input);
        let next_position = self.position_of(next_input);
        let best_paths = self.get_best_paths(prev_position, next_position);

        best_paths.into_iter().map(|dirs| {
            dirs.into_iter()
                .map(Instruction::from)
                .chain(once(Instruction::Activate))
                .collect()
        })
    }

    // return all possible best instructions needed to input next_input
    // starting from start_key
    pub fn get_successors_for_input_sequence(
        &self,
        start_key: &T,
        sequence_to_input: &[T],
    ) -> Vec<Vec<Instruction>> {
        let mut best_paths: Vec<Vec<Instruction>> = vec![vec![]];

        for (prev_key, next_key) in once(start_key)
            .chain(sequence_to_input.iter())
            .tuple_windows()
        {
            let succs = self.get_successors(prev_key, next_key);
            best_paths = succs
                .into_iter()
                .flat_map(|next_path_part| {
                    best_paths
                        .iter()
                        .map(|best_path| [best_path.as_slice(), next_path_part.as_slice()].concat())
                        .collect::<Vec<_>>()
                })
                .collect();
        }

        best_paths
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

// robot_num = closest to numeric keypad is largest, 0 is me
// return the number of instructions it takes for the robot

// I high key have no idea what going on... need to redo this day
#[memoize]
fn shortest_length_helper(sequence_to_input: Vec<Instruction>, robot_num: usize) -> usize {
    let mut result = 0;

    for (prev_key, next_key) in once(Instruction::Activate)
        .chain(sequence_to_input.into_iter())
        .tuple_windows()
    {
        if robot_num == 0 {
            result += INSTRUCTION_KEYPAD
                .get_successors(&prev_key, &next_key)
                .into_iter()
                .next()
                .unwrap()
                .len();
        } else {
            result += INSTRUCTION_KEYPAD
                .get_successors(&prev_key, &next_key)
                .into_iter()
                .map(|path| shortest_length_helper(path, robot_num - 1))
                .min()
                .unwrap();
        }
    }

    result
}

pub fn shortest_sequence_length(keycode: &[u8], num_intermediate_robots: usize) -> usize {
    let best_paths_for_numeric_bot =
        NUMERIC_KEYPAD.get_successors_for_input_sequence(&b'A', keycode);

    best_paths_for_numeric_bot
        .into_iter()
        .map(|path| shortest_length_helper(path, num_intermediate_robots - 1))
        .min()
        .unwrap()
}
