use std::array;

use enum_iterator::{all, cardinality};
use smallvec::SmallVec;

use crate::{direction::Direction, instruction::Instruction, keypad::Keypad};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct State<const N: usize> {
    robot_positions: [(usize, usize); N],
    robot_moves_since_activate: [SmallVec<[Direction; 8]>; N],
    numeric_robot_position: (usize, usize),
    num_chars_output: usize,
}

impl<const N: usize> State<N> {
    pub fn new_start_state() -> Self {
        let directional_start_position =
            Keypad::get_instruction_keypad().position_of(&Instruction::Activate);

        let robot_positions = [directional_start_position; N];
        let robot_moves_since_activate = array::from_fn(|_| SmallVec::new());
        let numeric_robot_position = Keypad::get_numeric_keypad().position_of(&b'A');

        State {
            robot_positions,
            robot_moves_since_activate,
            numeric_robot_position,
            num_chars_output: 0,
        }
    }

    #[inline(always)]
    const fn is_numeric_robot(keypad_presser: usize) -> bool {
        keypad_presser == N
    }

    #[inline(always)]
    pub const fn is_goal(&self, keycode: &[u8]) -> bool {
        self.num_chars_output == keycode.len()
    }

    // return empty if can't do the instruction
    fn process_instruction(
        mut self,
        instruction: Instruction,
        robot_to_move: usize,
        keycode: &[u8],
    ) -> Option<Self> {
        match instruction {
            Instruction::Direction(direction) => {
                // Assume best path to a key can always be a shortest path on
                // lattice points (i.e. doesnt have to avoid points >vv< type)
                if Self::is_numeric_robot(robot_to_move) {
                    let new_robot_position = Keypad::get_numeric_keypad()
                        .get_successor(self.numeric_robot_position, direction)?;
                    let dist = self.get_heuristic(keycode);
                    self.numeric_robot_position = new_robot_position;
                    let new_dist = self.get_heuristic(keycode);
                    if new_dist < dist {
                        Some(self)
                    } else {
                        None
                    }
                } else if self.robot_moves_since_activate[robot_to_move]
                    .contains(&direction.opposite())
                {
                    None
                } else {
                    let new_robot_position = Keypad::get_instruction_keypad()
                        .get_successor(self.robot_positions[robot_to_move], direction)?;
                    self.robot_positions[robot_to_move] = new_robot_position;
                    self.robot_moves_since_activate[robot_to_move].push(direction);
                    Some(self)
                }
            }
            Instruction::Activate => {
                if Self::is_numeric_robot(robot_to_move) {
                    let key_pressed = *Keypad::get_numeric_keypad()
                        .get(self.numeric_robot_position)
                        .unwrap();
                    if key_pressed == keycode[self.num_chars_output] {
                        self.num_chars_output += 1;
                        Some(self)
                    } else {
                        None
                    }
                } else {
                    let key_pressed = *Keypad::get_instruction_keypad()
                        .get(self.robot_positions[robot_to_move])
                        .unwrap();
                    self.robot_moves_since_activate[robot_to_move].clear();
                    self.process_instruction(key_pressed, robot_to_move + 1, keycode)
                }
            }
        }
    }

    fn get_heuristic(&self, keycode: &[u8]) -> usize {
        // assume self.num_chars_output < keycode.len()
        // Also assume best path to a key can always be a shortest path on
        // lattice points (i.e. doesnt have to avoid points)
        let next_key_to_input = keycode[self.num_chars_output];
        let next_key_position = Keypad::get_numeric_keypad().position_of(&next_key_to_input);
        self.numeric_robot_position.0.abs_diff(next_key_position.0)
            + self.numeric_robot_position.1.abs_diff(next_key_position.1)
    }

    pub fn get_successors(&self, keycode: &[u8]) -> impl IntoIterator<Item = Self> {
        all::<Instruction>()
            .filter_map(|instruction| self.clone().process_instruction(instruction, 0, keycode))
            .collect::<SmallVec<[_; cardinality::<Instruction>()]>>()
    }
}
