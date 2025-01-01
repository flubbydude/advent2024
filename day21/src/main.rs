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

// N is the number of intermediate robots
fn shortest_sequence_length<const N: usize>(keycode: &[u8]) -> usize {
    let start_state = State::<N>::new_start_state();
    let mut frontier = VecDeque::from([start_state.clone()]);
    let mut seen = HashSet::from([start_state]);

    let mut length = 1;
    while !frontier.is_empty() {
        let prev_frontier = frontier;
        frontier = VecDeque::new();

        for state in prev_frontier {
            for succ in state.get_successors(keycode) {
                if succ.is_goal(keycode) {
                    println!("Sequence found for keycode = {keycode:?}");
                    return length;
                }

                if seen.contains(&succ) {
                    continue;
                }

                seen.insert(succ.clone());
                frontier.push_back(succ);
            }
        }

        length += 1;
    }

    panic!("No sequence found for keycode = {keycode:?}")
}

fn part1(keypad_codes: &[&[u8]]) -> usize {
    const NUM_INTERMEDIATE_ROBOTS: usize = 2;
    keypad_codes
        .iter()
        .map(|&code| {
            shortest_sequence_length::<NUM_INTERMEDIATE_ROBOTS>(code)
                * get_numeric_part_of_code(code)
        })
        .sum()
}

// what if:
// Get all the best paths for numeric robot
// Then from that get all the best paths for normal robot N - 1
// Some will be pruned due to taking longer steps
// From that, get all the best paths for normal robot N - 2
// ...
// Get all/1 of the best paths for the main user guy
// return the length.
fn part2(keypad_codes: &[&[u8]]) -> usize {
    const NUM_INTERMEDIATE_ROBOTS: usize = 25;
    keypad_codes
        .iter()
        .map(|&code| {
            shortest_sequence_length::<NUM_INTERMEDIATE_ROBOTS>(code)
                * get_numeric_part_of_code(code)
        })
        .sum()
}

fn main() {
    const INPUT_STR: &str = include_str!("../input.txt");

    let codes = parse_keycodes(INPUT_STR);

    println!("{}", part1(&codes));
    println!("{}", part2(&codes));
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
