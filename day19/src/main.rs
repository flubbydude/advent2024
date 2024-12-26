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

fn num_ways_to_make_design<'a>(
    towels: &[&[u8]],
    design: &'a [u8],
    memoization: &mut HashMap<&'a [u8], usize>,
) -> usize {
    if let Some(&num_ways) = memoization.get(design) {
        return num_ways;
    };

    let result = if design.is_empty() {
        1
    } else {
        towels
            .iter()
            .map(|&towel| {
                design
                    .strip_prefix(towel)
                    .map(|rest| num_ways_to_make_design(towels, rest, memoization))
                    .unwrap_or(0)
            })
            .sum()
    };

    memoization.insert(design, result);

    result
}

fn is_design_possible<'a>(
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
            design
                .strip_prefix(towel)
                .map(|rest| is_design_possible(towels, rest, memoization))
                .unwrap_or(false)
        })
    };

    memoization.insert(design, result);
    result
}

fn part1(towels: &[&[u8]], designs: &[&[u8]]) -> usize {
    let mut memoization: HashMap<&[u8], bool> = HashMap::new();

    designs
        .iter()
        .filter(|&design| is_design_possible(towels, design, &mut memoization))
        .count()
}

fn part2(towels: &[&[u8]], designs: &[&[u8]]) -> usize {
    let mut memoization: HashMap<&[u8], usize> = HashMap::new();

    designs
        .iter()
        .map(|design| num_ways_to_make_design(towels, design, &mut memoization))
        .sum()
}

fn main() {
    const INPUT_STR: &str = include_str!("../input.txt");

    let (towels, designs) = parse_input(INPUT_STR);

    println!("{}", part1(&towels, &designs));
    println!("{}", part2(&towels, &designs));
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

    #[test]
    fn test_part2() {
        let (towels, designs) = parse_input(INPUT_STR);

        assert_eq!(part2(&towels, &designs), 16);
    }
}
