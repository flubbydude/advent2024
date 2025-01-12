mod secret_number;

use secret_number::secret_number_iter;

fn parse_input(input: &str) -> Vec<u64> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

fn part1(initial_secret_numbers: &[u64]) -> u64 {
    const NEW_SECRETS_TO_GENERATE: usize = 2000;
    initial_secret_numbers
        .iter()
        .map(|&initial_secret_number| {
            secret_number_iter(initial_secret_number)
                .nth(NEW_SECRETS_TO_GENERATE)
                .unwrap()
        })
        .sum()
}

const INPUT: &str = include_str!("../input.txt");

fn main() {
    let initial_secret_numbers = parse_input(INPUT);
    println!("{}", part1(&initial_secret_numbers));
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT_PART_1: &str = include_str!("../example.txt");

    #[test]
    fn test_part_1() {
        let initial_secret_numbers = parse_input(TEST_INPUT_PART_1);
        assert_eq!(part1(&initial_secret_numbers), 37327623);
    }
}
