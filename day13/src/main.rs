mod util;

use util::{parse_machines, Machine};

fn part1(input: &[Machine]) -> usize {
    input.iter().filter_map(Machine::fewest_tokens_to_win).sum()
}

fn main() {
    let file_contents_as_str = include_str!("../input.txt");

    let input = parse_machines(file_contents_as_str);

    println!("{}", part1(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("../example.txt");

    #[test]
    fn test_part1() {
        let input = parse_machines(TEST_INPUT);
        assert_eq!(480, part1(&input))
    }
}
