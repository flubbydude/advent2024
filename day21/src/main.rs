use instruction::Instruction;
use keypad::Keypad;

mod direction;
mod instruction;
mod keypad;
mod shortest_paths;
mod trie;

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
// assume the best solution contains only shortest paths for each robot
// and the best solution contains only shortest paths with at most 1 turn
fn shortest_sequence_length<const N: usize>(keycode: &[u8]) -> usize {
    let numeric_keypad = Keypad::get_numeric_keypad();

    let mut best_paths = numeric_keypad
        .get_successors_for_input_sequence(numeric_keypad.position_of(&b'A'), keycode);

    let instr_keypad = Keypad::get_instruction_keypad();
    let instr_keypad_start_pos = instr_keypad.position_of(&Instruction::Activate);

    // maybe prune paths by length at start of loop? Not sure if it
    // preserves the correct answer still...
    for i in 0..N {
        println!("\n\n{i}:");

        let shortest_paths = best_paths.iter().map(|p| p.len()).min().unwrap();

        for path in &best_paths {
            println!("{}", instrs_to_string(path));
        }

        let mut next_best_paths = Vec::new();
        for best_path in best_paths
            .into_iter()
            .filter(|p| p.len() <= shortest_paths + 1)
        {
            next_best_paths.extend(
                instr_keypad.get_successors_for_input_sequence(instr_keypad_start_pos, &best_path),
            );
        }
        best_paths = next_best_paths;
    }

    best_paths.into_iter().map(|path| path.len()).min().unwrap()
}

fn instrs_to_string(instrs: &[Instruction]) -> String {
    instrs.iter().copied().map(char::from).collect()
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
// Then from that get all the best paths for normal robot N - 1 (to ANY POSITION)
// Some will be pruned due to taking longer steps
// From that, get all the best paths for normal robot N - 2 (to ANY POSITION!)
// ...
// Get all/1 of the best paths for the main user guy
// return the length.
// gotta be backwards fr
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
