use std::ops::{Index, IndexMut};

use enum_iterator::{all, Sequence};
use smallvec::SmallVec;

use crate::{instruction::Instruction, keypad::Keypad};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct RobotPositions {
    robot1_position: (usize, usize),
    robot2_position: (usize, usize),
    numeric_robot_position: (usize, usize),
}

#[derive(Debug, Clone, Copy, Sequence)]
enum KeypadPresser {
    Me,
    Robot(Robot),
}

impl KeypadPresser {
    fn successor_robot(&self) -> Robot {
        match self {
            KeypadPresser::Me => Robot::first().unwrap(),
            KeypadPresser::Robot(robot) => {
                Robot::next(robot).expect("Cannot get successor of final robot")
            }
        }
    }
}

#[derive(Debug, Clone, Copy, Sequence)]
enum Robot {
    First,
    Second,
    Numeric,
}

impl Index<Robot> for RobotPositions {
    type Output = (usize, usize);

    fn index(&self, robot: Robot) -> &Self::Output {
        match robot {
            Robot::First => &self.robot1_position,
            Robot::Second => &self.robot2_position,
            Robot::Numeric => &self.numeric_robot_position,
        }
    }
}

impl IndexMut<Robot> for RobotPositions {
    fn index_mut(&mut self, robot: Robot) -> &mut Self::Output {
        match robot {
            Robot::First => &mut self.robot1_position,
            Robot::Second => &mut self.robot2_position,
            Robot::Numeric => &mut self.numeric_robot_position,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct State {
    positions: RobotPositions,
    output: SmallVec<[u8; 4]>,
}

impl State {
    pub fn new_start_state() -> Self {
        let other_start_position =
            Keypad::get_instruction_keypad().position_of(&Instruction::Activate);
        let numeric_start_position = Keypad::get_numeric_keypad().position_of(&b'A');

        State {
            positions: RobotPositions {
                robot1_position: other_start_position,
                robot2_position: other_start_position,
                numeric_robot_position: numeric_start_position,
            },
            output: SmallVec::new(),
        }
    }

    fn process_instruction(
        &self,
        instruction: Instruction,
        keypad_presser: KeypadPresser,
    ) -> Option<State> {
        let robot_to_move = keypad_presser.successor_robot();
        match instruction {
            Instruction::Direction(direction) => {
                let new_robot_position = match robot_to_move {
                    Robot::Numeric => Keypad::get_numeric_keypad()
                        .get_successor(self.positions.numeric_robot_position, direction),
                    _ => Keypad::get_instruction_keypad()
                        .get_successor(self.positions[robot_to_move], direction),
                }?;
                let mut result = self.clone();
                result.positions[robot_to_move] = new_robot_position;
                Some(result)
            }
            Instruction::Activate => match robot_to_move {
                Robot::Numeric => {
                    let key_pressed = *Keypad::get_numeric_keypad()
                        .get(self.positions.numeric_robot_position)
                        .unwrap();
                    let mut result = self.clone();
                    result.output.push(key_pressed);
                    Some(result)
                }
                robot => {
                    let key_pressed = *Keypad::get_instruction_keypad()
                        .get(self.positions[robot])
                        .unwrap();
                    self.process_instruction(key_pressed, KeypadPresser::Robot(robot))
                }
            },
        }
    }

    pub fn get_successors(&self) -> impl '_ + IntoIterator<Item = (Instruction, State)> {
        all::<Instruction>().filter_map(|instruction| {
            self.process_instruction(instruction, KeypadPresser::Me)
                .map(|new_state| (instruction, new_state))
        })
    }

    pub fn output(&self) -> &[u8] {
        &self.output
    }
}
