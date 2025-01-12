mod direction;
mod instruction;
mod keypad;
mod shortest_paths;

use keypad::shortest_sequence_length;

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

fn run(keypad_codes: &[&[u8]], num_intermediate_robots: usize) -> usize {
    keypad_codes
        .iter()
        .map(|&code| {
            shortest_sequence_length(code, num_intermediate_robots) * get_numeric_part_of_code(code)
        })
        .sum()
}

const NUM_INTERMEDIATE_ROBOTS_PART_1: usize = 2;
const NUM_INTERMEDIATE_ROBOTS_PART_2: usize = 25;

fn main() {
    const INPUT_STR: &str = include_str!("../input.txt");

    let codes = parse_keycodes(INPUT_STR);

    println!("{}", run(&codes, NUM_INTERMEDIATE_ROBOTS_PART_1));
    println!("{}", run(&codes, NUM_INTERMEDIATE_ROBOTS_PART_2));
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT_STR: &str = include_str!("../example.txt");

    #[test]
    fn test_part1() {
        let codes = parse_keycodes(TEST_INPUT_STR);
        assert_eq!(126384, run(&codes, NUM_INTERMEDIATE_ROBOTS_PART_1));
    }
}
