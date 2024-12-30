mod direction;
mod instruction;
mod keypad;
mod numeric_keypad_button;

use std::ops::Deref;

use array2d::Array2D;

use instruction::Instruction;
use keypad::{build_keypad_from_string, KeypadShortestPaths};
use numeric_keypad_button::NumericKeypadButton;

const NUMERIC_KEYPAD_STR: &str = include_str!("../numeric_keypad.txt");
const INSTRUCTION_KEYPAD_STR: &str = include_str!("../instruction_keypad.txt");

fn parse_keypad_codes(input_str: &str) -> Vec<Vec<NumericKeypadButton>> {
    input_str
        .lines()
        .map(|line| line.chars().map(|c| c.try_into().unwrap()).collect())
        .collect()
}

fn get_numeric_part_of_code(keypad_code: &[NumericKeypadButton]) -> usize {
    keypad_code
        .iter()
        .take_while(|&button| matches!(button, NumericKeypadButton::Number(_)))
        .map(|button| match button {
            &NumericKeypadButton::Number(number) => number,
            _ => panic!(),
        })
        .fold(0, |acc, val| acc * 10 + val as usize)
}

fn get_button_position<T: Eq>(button_to_find: &T, keypad: &Array2D<Option<T>>) -> (usize, usize) {
    keypad
        .enumerate_row_major()
        .find(|(_, button)| button.as_ref() == Some(button_to_find))
        .unwrap()
        .0
}

// return length of shortest sequence
fn part1_helper(keypad_code: &[NumericKeypadButton]) -> usize {
    let instruction_keypad_str = build_keypad_from_string::<Instruction>(INSTRUCTION_KEYPAD_STR);
    let instruction_keypad_shortest_paths =
        KeypadShortestPaths::new_from_keypad(&instruction_keypad_str);

    let numeric_keypad = build_keypad_from_string::<NumericKeypadButton>(NUMERIC_KEYPAD_STR);
    let numeric_keypad_shortest_paths = KeypadShortestPaths::new_from_keypad(&numeric_keypad);

    let mut my_position = get_button_position(&Instruction::Activate, &instruction_keypad_str);
    let mut numeric_robot_position =
        get_button_position(&NumericKeypadButton::Activate, &numeric_keypad);
    let mut robot1_position = my_position;
    let mut robot2_position = my_position;

    let mut my_sequence_length = 0;

    for number in keypad_code {
        let numeric_robot_target = 
        numeric_keypad_shortest_paths.get_shortest_paths(source, target)
    }

    todo!()
}

fn part1<A: Deref<Target = [NumericKeypadButton]>>(keypad_codes: &[A]) -> usize {
    keypad_codes
        .iter()
        .map(|code| part1_helper(code) * get_numeric_part_of_code(code))
        .sum()
}

const INPUT_STR: &str = include_str!("../input.txt");

fn main() {
    let codes = parse_keypad_codes(INPUT_STR);

    println!("{}", part1(&codes));
}
