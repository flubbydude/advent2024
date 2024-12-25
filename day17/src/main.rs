mod computer;
mod puzzle_input;

use std::iter;

use itertools::Itertools;

use computer::run_program;
use puzzle_input::PuzzleInput;

fn part1(input: &PuzzleInput) -> String {
    run_program(&input.tribit_code, input.memory.clone())
        .into_iter()
        .join(",")
}

fn part1_decompiled(input: &PuzzleInput) -> String {
    let mut a = input.memory.registers.a;
    if a == 0 {
        return "0".to_string();
    }

    iter::from_fn(|| {
        if a == 0 {
            return None;
        }

        let b0 = (a % 8) as u8;
        let b1 = b0 ^ 5;

        let c = a.checked_shr(b1 as u32).unwrap_or_default();

        a >>= 3;

        let b2 = b1 ^ (c % 8) as u8;
        let b3 = b2 ^ 6;

        Some(b3)
    })
    .join(",")
}

fn part2_helper(next_a: u64, b3: u8, b1: u8, prev_output: &[u8]) -> Option<u64> {
    let b2 = b3 ^ 6;
    let b0 = b1 ^ 5;

    let a = (next_a << 3) | b0 as u64;

    let c = a.checked_shr(b1 as u32).unwrap_or_default() % 8;
    if b2 ^ (c % 8) as u8 != b1 {
        return None;
    }

    let Some((&prev_b3, prev_prev_output)) = prev_output.split_last() else {
        return Some(a);
    };

    (0..8)
        .flat_map(|prev_b1| part2_helper(a, prev_b3, prev_b1, prev_prev_output))
        .min()
}

fn part2(input: &PuzzleInput) -> u64 {
    let (&b3, prev_output) = input.tribit_code.split_last().unwrap();

    (0..8)
        .flat_map(|maybe_b1| part2_helper(0, b3, maybe_b1, prev_output))
        .min()
        .unwrap()
}

const INPUT: &str = include_str!("../input.txt");

fn main() {
    let input = PuzzleInput::parse_input(INPUT);

    println!("{}", part1(&input));
    println!("{}", part1_decompiled(&input));
    println!("{}", part2(&input));
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
    fn test_part2_example() {
        const PART2_EXAMPLE_SOLN: u64 = 117440;

        let input = PuzzleInput::parse_input(TEST_INPUT_PART2);
        let mut a = 0;
        for &tribit in input.tribit_code.iter().rev() {
            a |= tribit as u64;
            a <<= 3;
        }
        assert_eq!(a, PART2_EXAMPLE_SOLN);
        assert!(check_part2(&input, PART2_EXAMPLE_SOLN));
    }

    fn check_part2(input: &PuzzleInput, answer: u64) -> bool {
        let mut program_memory = input.memory.clone();
        program_memory.registers.a = answer;

        let output = run_program(&input.tribit_code, program_memory)
            .into_iter()
            .collect::<Vec<_>>();
        *input.tribit_code == *output
    }

    #[test]
    fn test_part2() {
        let input = PuzzleInput::parse_input(INPUT);
        let part2_solution = part2(&input);
        assert!(check_part2(&input, part2_solution));
    }
}
