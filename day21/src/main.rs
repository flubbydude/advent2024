mod direction;
mod instruction;
mod keypad;
mod numeric_keypad_button;

use std::ops::Deref;

use array2d::Array2D;

use instruction::Instruction;
use keypad::build_keypad_from_string;
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

fn shortest_sequence_length(keypad_code: &[NumericKeypadButton]) -> usize {
    let instruction_keypad_str = build_keypad_from_string::<Instruction>(INSTRUCTION_KEYPAD_STR);

    let numeric_keypad = build_keypad_from_string::<NumericKeypadButton>(NUMERIC_KEYPAD_STR);

    // todo rethink this as search
    // the code needs to store state as the robot positions along with
    // what has been output so far, and one step is going to be a movement from my_position
    // which will affect the downstream robots as needed.
    // Also store the path at each step
    //
    // Prune branches if the code entered is incorrect
    // Also prune branches if any robot goes off grid
    //
    // Prune branches based on stored path length at that state if seen (HashMap<State, Vec<Instruction>>)
    // Also can prune branches if number robot veers off of an ideal path, like if the numeric robot goes
    //    in a direction away from the next key (ex: need to go from 9 to A but went left)
    //    - Could use heuristic as distance numeric robot hand to the next button needed?!
    //    - If numeric robot moves in a direction, make sure the heuristic goes down
    //
    // Quite optional since idk if easily possible, prune branch if a robot went back the way it just came or off
    // back on a path since the last press. Would cover the "number robot veers off of an ideal path"
    //
    // Much simpler!
    todo!()
}

fn part1<A: Deref<Target = [NumericKeypadButton]>>(keypad_codes: &[A]) -> usize {
    keypad_codes
        .iter()
        .map(|code| shortest_sequence_length(code) * get_numeric_part_of_code(code))
        .sum()
}

const INPUT_STR: &str = include_str!("../input.txt");

fn main() {
    let codes = parse_keypad_codes(INPUT_STR);

    println!("{}", part1(&codes));
}
