use std::collections::HashMap;

fn parse_input(input_str: &str) -> (Vec<&[u8]>, Vec<&[u8]>) {
    let mut lines = input_str.lines();

    let towels = lines
        .next()
        .unwrap()
        .split(", ")
        .map(str::as_bytes)
        .collect();

    lines.next();

    let designs = lines.map(str::as_bytes).collect();

    (towels, designs)
}

fn is_possible_part1<'a>(
    towels: &[&[u8]],
    design: &'a [u8],
    memoization: &mut HashMap<&'a [u8], bool>,
) -> bool {
    if let Some(&is_possible) = memoization.get(design) {
        return is_possible;
    };

    let result = if design.is_empty() {
        true
    } else {
        towels.iter().any(|&towel| {
            if let Some(rest) = design.strip_prefix(towel) {
                is_possible_part1(towels, rest, memoization)
            } else {
                false
            }
        })
    };

    memoization.insert(design, result);

    result
}

fn part1(towels: &[&[u8]], designs: &[&[u8]]) -> usize {
    let mut memoization: HashMap<&[u8], bool> = HashMap::new();

    designs
        .iter()
        .filter(|&design| is_possible_part1(towels, design, &mut memoization))
        .count()
}

fn main() {
    const INPUT_STR: &str = include_str!("../input.txt");

    let (towels, designs) = parse_input(INPUT_STR);

    println!("{}", part1(&towels, &designs));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_STR: &str = include_str!("../example.txt");

    #[test]
    fn test_parse_input() {
        let (towels, designs) = parse_input(INPUT_STR);
        assert_eq!(towels.len(), 8);
        assert_eq!(designs.len(), 8);
    }

    #[test]
    fn test_part1() {
        let (towels, designs) = parse_input(INPUT_STR);

        assert_eq!(part1(&towels, &designs), 6);
    }
}
