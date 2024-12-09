fn parse_input(input: &str) -> &str {
    input
}

fn part1(input: &str) -> usize {
    todo!()
}

fn part2(input: &str) -> usize {
    todo!()
}

fn main() {
    let file_contents_as_str = include_str!("../input.txt");

    let input = parse_input(file_contents_as_str);

    println!("{}", part1(input));
    println!("{}", part2(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "";

    #[test]
    fn test_parse_input() {
        todo!()
    }

    #[test]
    fn test_part1() {
        let input = parse_input(TEST_INPUT);
        assert_eq!(0, part1(input))
    }

    #[test]
    fn test_part2() {
        let input = parse_input(TEST_INPUT);
        assert_eq!(0, part2(input))
    }
}
