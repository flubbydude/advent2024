mod secret_number;

use std::{array, collections::HashMap};

use itertools::Itertools;

use secret_number::secret_number_iter;

const NUM_PRICES_GENERATED: usize = 2001;
const SEQUENCE_LEN: usize = 4;

fn parse_input(input: &str) -> Vec<u64> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

fn part1(initial_secret_numbers: &[u64]) -> u64 {
    initial_secret_numbers
        .iter()
        .map(|&initial_secret_number| {
            secret_number_iter(initial_secret_number)
                .nth(NUM_PRICES_GENERATED - 1)
                .unwrap()
        })
        .sum()
}

fn get_prices_by_buyer(initial_secret_numbers: &[u64]) -> Vec<[u8; NUM_PRICES_GENERATED]> {
    initial_secret_numbers
        .iter()
        .map(|&initial_secret_number| {
            secret_number_iter(initial_secret_number)
                .take(NUM_PRICES_GENERATED)
                .map(|secret_number| (secret_number % 10) as u8)
                .collect_array()
                .unwrap()
        })
        .collect()
}

fn generate_sequences_and_profit_for_buyer(
    buyer: &[u8; NUM_PRICES_GENERATED],
) -> HashMap<[i8; SEQUENCE_LEN], u8> {
    let mut result = HashMap::new();

    for window in buyer.windows(SEQUENCE_LEN + 1) {
        let sequence = window
            .iter()
            .copied()
            .tuple_windows()
            .map(|(prev_price, next_price)| ((next_price as i16) - (prev_price as i16)) as i8)
            .collect_array::<SEQUENCE_LEN>()
            .unwrap();

        result.entry(sequence).or_insert(*window.last().unwrap());
    }

    result
}

fn part2(initial_secret_numbers: &[u64]) -> usize {
    let buyers = get_prices_by_buyer(initial_secret_numbers);
    let cached_profit_by_sequence = buyers
        .iter()
        .map(generate_sequences_and_profit_for_buyer)
        .collect::<Vec<_>>();

    let all_possible_sequences = array::from_fn::<_, SEQUENCE_LEN, _>(|_| (-9..=9))
        .into_iter()
        .multi_cartesian_product();

    all_possible_sequences
        .into_iter()
        .map(|sequence| {
            let sequence_array: [i8; SEQUENCE_LEN] = sequence.try_into().unwrap();
            cached_profit_by_sequence
                .iter()
                .map(|cached_profit| cached_profit.get(&sequence_array).unwrap_or(&0))
                .map(|&profit| profit as usize)
                .sum()
        })
        .max()
        .unwrap()
}

const INPUT: &str = include_str!("../input.txt");

fn main() {
    let initial_secret_numbers = parse_input(INPUT);
    println!("{}", part1(&initial_secret_numbers));
    println!("{}", part2(&initial_secret_numbers));
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT_PART_1: &str = include_str!("../example.txt");
    const TEST_INPUT_PART_2: &str = include_str!("../example2.txt");

    #[test]
    fn test_part_1() {
        let initial_secret_numbers = parse_input(TEST_INPUT_PART_1);
        assert_eq!(part1(&initial_secret_numbers), 37327623);
    }

    #[test]
    fn test_part_2() {
        let initial_secret_numbers = parse_input(TEST_INPUT_PART_2);
        assert_eq!(part2(&initial_secret_numbers), 23);
    }
}
