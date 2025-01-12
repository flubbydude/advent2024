use std::{cell::RefCell, collections::HashMap, iter::once, rc::Rc};

use array2d::Array2D;
use enum_iterator::all;
use once_cell::sync::Lazy;

use crate::{
    direction::Direction,
    instruction::Instruction,
    shortest_paths::{iter_positions, BestPaths},
    trie::InstructionTrie,
};

#[derive(Debug, Clone)]
pub struct Keypad<T>(Array2D<Option<T>>);

impl Keypad<Instruction> {
    pub fn get_instruction_keypad() -> &'static Self {
        &INSTRUCTION_KEYPAD
    }

    pub fn create_memoization(&self) -> HashMap<Position, Rc<RefCell<InstructionTrie>>> {
        let mut result = HashMap::new();
        for source in self
            .0
            .enumerate_row_major()
            .filter_map(|(pos, val)| val.map(|_| pos))
        {
            let mut trie = InstructionTrie::new(vec![vec![]]);
            for instruction in all::<Instruction>() {
                trie.insert(
                    instruction,
                    self.get_successors(source, &instruction)
                        .1
                        .into_iter()
                        .collect(),
                );
            }
            result.insert(source, Rc::new(RefCell::new(trie)));
        }
        result
    }

    pub fn get_successors_for_input_sequence_with_trie_memoization(
        &self,
        mut current_position: (usize, usize),
        sequence_to_input: &[Instruction],
        memoization: &mut HashMap<Position, Rc<RefCell<InstructionTrie>>>,
    ) -> Vec<Vec<Instruction>> {
        let mut best_paths: Vec<Vec<Instruction>> = vec![vec![]];
        let mut cur_trie = memoization[&current_position].clone();
        let mut prev_key = Instruction::Activate;

        for key in sequence_to_input {
            let maybe_next_trie = cur_trie.borrow().get_child(key);
            if let Some(next_trie) = maybe_next_trie {
                cur_trie = next_trie;
                prev_key = *key;
                continue;
            }

            {
                let mut cur_trie_ref_mut = (*cur_trie).borrow_mut();

                current_position = self.position_of(&prev_key);
                let (next_position, last_step_best_paths) =
                    self.get_successors(current_position, key);
                current_position = next_position;

                let memoized_paths = cur_trie_ref_mut.best_paths();

                let paths_to_insert = last_step_best_paths
                    .into_iter()
                    .flat_map(|last_step_best_path| {
                        memoized_paths
                            .iter()
                            .map(|prev_best_path| {
                                [prev_best_path.as_slice(), last_step_best_path.as_slice()].concat()
                            })
                            .collect::<Vec<_>>()
                    })
                    .collect::<Vec<_>>();

                best_paths = paths_to_insert
                    .iter()
                    .flat_map(|next_path_part| {
                        best_paths
                            .iter()
                            .map(|best_path| {
                                [best_path.as_slice(), next_path_part.as_slice()].concat()
                            })
                            .collect::<Vec<_>>()
                    })
                    .collect();

                cur_trie_ref_mut.insert(*key, paths_to_insert);
            }

            cur_trie = memoization[&current_position].clone();
            prev_key = *key;
        }

        best_paths = cur_trie
            .borrow()
            .best_paths()
            .iter()
            .flat_map(|next_path_part| {
                best_paths
                    .iter()
                    .map(|prev_best_path| {
                        [prev_best_path.as_slice(), next_path_part.as_slice()].concat()
                    })
                    .collect::<Vec<_>>()
            })
            .collect();

        best_paths
    }
}

impl Keypad<u8> {
    pub fn get_numeric_keypad() -> &'static Self {
        &NUMERIC_KEYPAD
    }
}

type Position = (usize, usize);

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

    // return (next position, instructions)
    fn get_successors(
        &self,
        current_position: (usize, usize),
        next_input: &T,
    ) -> ((usize, usize), impl IntoIterator<Item = Vec<Instruction>>) {
        let next_position = self.position_of(next_input);
        let best_paths = self.get_best_paths(current_position, next_position);

        assert!(!best_paths.is_empty());

        (
            next_position,
            best_paths.into_iter().map(|dirs| {
                dirs.into_iter()
                    .map(Instruction::from)
                    .chain(once(Instruction::Activate))
                    .collect()
            }),
        )
    }

    pub fn get_successors_for_input_sequence(
        &self,
        mut current_position: (usize, usize),
        sequence_to_input: &[T],
    ) -> Vec<Vec<Instruction>> {
        let mut best_paths: Vec<Vec<Instruction>> = vec![vec![]];

        for key in sequence_to_input {
            let (next_pos, succs) = self.get_successors(current_position, key);
            current_position = next_pos;
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
