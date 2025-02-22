mod util;

use util::{parse_input, LockOrKeyParseError, PuzzleInput, LOCK_KEY_HEIGHT, LOCK_KEY_WIDTH};

const INPUT: &str = include_str!("../input.txt");

fn lock_and_key_overlap(lock: &[u8; LOCK_KEY_WIDTH], key: &[u8; LOCK_KEY_WIDTH]) -> bool {
    lock.iter()
        .zip(key)
        .any(|(&lock_col, &key_col)| lock_col + key_col > (LOCK_KEY_HEIGHT as u8) - 2)
}

fn part1(puzzle_input: &PuzzleInput) -> usize {
    let PuzzleInput { locks, keys } = puzzle_input;
    locks
        .iter()
        .flat_map(|lock| keys.iter().map(move |key| (lock, key)))
        .filter(|&(lock, key)| !lock_and_key_overlap(lock, key))
        .count()
}

fn main() -> Result<(), LockOrKeyParseError> {
    let puzzle_input = parse_input(INPUT)?;

    println!("{}", part1(&puzzle_input));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("../example.txt");

    #[test]
    fn test_part1() -> Result<(), LockOrKeyParseError> {
        let puzzle_input = parse_input(TEST_INPUT)?;
        assert_eq!(part1(&puzzle_input), 3);
        Ok(())
    }
}
