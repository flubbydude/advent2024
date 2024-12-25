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

        let mut b = (a % 8) as u8;
        b ^= 5;

        let c = a.checked_shr(b as u32).unwrap_or_default();

        a >>= 3;

        b ^= (c % 8) as u8;
        b ^= 6;

        Some(b)
    })
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

fn part2(input: &PuzzleInput) -> u64 {
    let mut a = 0;
    for &tribit in input.tribit_code.iter().rev() {
        let b0 = tribit ^ 0b110 ^ 0b101;

        a |= b0 as u64;
        a *= 8;
    }
    let answer = a;
    check_part2(input, answer);
    answer
}

fn main() {
    let input_str = include_str!("../input.txt");

    let input = PuzzleInput::parse_input(input_str);

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
    fn part2_example() {
        const PART2_EXAMPLE_SOLN: u64 = 117440;

        let input = PuzzleInput::parse_input(TEST_INPUT_PART2);
        let mut a = 0;
        for &tribit in input.tribit_code.iter().rev() {
            a |= tribit as u64;
            a <<= 3;
        }
        assert_eq!(a, PART2_EXAMPLE_SOLN);
        check_part2(&input, PART2_EXAMPLE_SOLN);
    }
}
