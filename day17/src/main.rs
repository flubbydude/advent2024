mod computer;
mod puzzle_input;

use itertools::Itertools;

use computer::run_program;
use puzzle_input::PuzzleInput;

fn part1(input: &PuzzleInput) -> String {
    run_program(&input.tribit_code, input.memory.clone())
        .into_iter()
        .join(",")
}

fn check_part2(input: &PuzzleInput, answer: u64) {
    let mut program_memory = input.memory.clone();
    program_memory.registers.a = answer;

    let output = run_program(&input.tribit_code, program_memory)
        .into_iter()
        .collect::<Vec<_>>();
    assert_eq!(*input.tribit_code, *output);
}

fn main() {
    let input_str = include_str!("../input.txt");

    let input = PuzzleInput::parse_input(input_str);

    println!("{}", part1(&input));
    check_part2(&input, 5);
}

#[cfg(test)]
mod tests {
    use super::*;

    pub const TEST_INPUT: &str = include_str!("../example.txt");
    const TEST_INPUT_PART2: &str = include_str!("../example2.txt");

    #[test]
    fn test_part1() {
        let input = PuzzleInput::parse_input(TEST_INPUT);
        assert_eq!("4,6,3,5,6,3,5,2,1,0", part1(&input))
    }

    #[test]
    fn test_check_part2() {
        let input = PuzzleInput::parse_input(TEST_INPUT_PART2);
        check_part2(&input, 117440);
    }
}
