mod util;

use util::{parse_machines, Machine};

fn run(input: &[Machine]) -> u64 {
    input.iter().filter_map(Machine::fewest_tokens_to_win).sum()
}

fn main() {
    let file_contents_as_str = include_str!("../input.txt");

    let input = parse_machines(file_contents_as_str);

    println!("{}", run(&input));

    let part2_input = input
        .into_iter()
        .map(Machine::into_part_2)
        .collect::<Vec<_>>();

    println!("{}", run(&part2_input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("../example.txt");

    #[test]
    fn test_run() {
        let input = parse_machines(TEST_INPUT);
        assert_eq!(480, run(&input))
    }
}
