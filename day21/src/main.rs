use std::collections::{HashSet, VecDeque};

use state::State;

mod direction;
mod instruction;
mod keypad;
mod state;

fn parse_keycodes(input_str: &str) -> Vec<&[u8]> {
    input_str.lines().map(str::as_bytes).collect()
}

fn get_numeric_part_of_code(keycode: &[u8]) -> usize {
    keycode
        .iter()
        .take_while(|c| c.is_ascii_digit())
        .map(|&c| c - b'0')
        .fold(0, |acc, val| acc * 10 + val as usize)
}

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

// Ideally paths can be stored as a trie of node of Instruction
// so best can be HashMap<State, &Node> somehow
fn shortest_sequence_length(keycode: &[u8]) -> usize {
    let start_state = State::new_start_state();
    let mut frontier = VecDeque::from([start_state.clone()]);
    let mut seen = HashSet::from([start_state]);

    let mut length = 1;
    while !frontier.is_empty() {
        let prev_frontier = frontier;
        frontier = VecDeque::new();

        for state in prev_frontier {
            for (_, state) in state.get_successors() {
                if state.output() == keycode {
                    return length;
                }

                if !keycode.starts_with(state.output()) || seen.contains(&state) {
                    continue;
                }

                seen.insert(state.clone());
                frontier.push_back(state);
            }
        }

        length += 1;
    }

    panic!("No sequence found for keycode = {keycode:?}")
}

fn part1(keypad_codes: &[&[u8]]) -> usize {
    keypad_codes
        .iter()
        .map(|&code| shortest_sequence_length(code) * get_numeric_part_of_code(code))
        .sum()
}

fn main() {
    const INPUT_STR: &str = include_str!("../input.txt");

    let codes = parse_keycodes(INPUT_STR);

    println!("{}", part1(&codes));
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT_STR: &str = include_str!("../example.txt");

    #[test]
    fn test_part1() {
        let codes = parse_keycodes(TEST_INPUT_STR);
        assert_eq!(126384, part1(&codes));
    }
}
