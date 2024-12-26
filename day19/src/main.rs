mod color;
mod towel;

use std::collections::HashMap;

use color::Color;
use towel::Towel;

fn parse_input(input_str: &str) -> (Vec<Towel>, Vec<Box<[Color]>>) {
    let mut lines = input_str.lines();

    let towels = lines
        .next()
        .unwrap()
        .split(", ")
        .map(|towel_pattern_str| {
            towel_pattern_str
                .as_bytes()
                .iter()
                .map(|&c| Color::try_from(c).unwrap())
                .collect::<Towel>()
        })
        .collect();

    lines.next();

    let designs = lines
        .map(|line| {
            line.as_bytes()
                .iter()
                .map(|&c| Color::try_from(c).unwrap())
                .collect::<Vec<_>>()
                .into_boxed_slice()
        })
        .collect();

    (towels, designs)
}

fn is_possible_part1<'a>(
    towels: &[Towel],
    design: &'a [Color],
    memoization: &mut HashMap<&'a [Color], bool>,
) -> bool {
    if let Some(&is_possible) = memoization.get(design) {
        return is_possible;
    };

    let result = if design.is_empty() {
        true
    } else {
        towels.iter().any(|towel| {
            if let Some(rest) = design.strip_prefix(towel.pattern()) {
                is_possible_part1(towels, rest, memoization)
            } else {
                false
            }
        })
    };

    memoization.insert(design, result);

    result
}

fn part1(towels: &[Towel], designs: &[Box<[Color]>]) -> usize {
    let mut memoization: HashMap<&[Color], bool> = HashMap::new();

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
