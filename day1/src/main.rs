fn parse_input(input: &str) -> (Vec<usize>, Vec<usize>) {
    input
        .lines()
        .map(|line| {
            line.split_once("   ")
                .map(|(s1, s2)| (s1.parse::<usize>().unwrap(), s2.parse::<usize>().unwrap()))
                .unwrap()
        })
        .unzip()
}

fn part1(mut list1: Vec<usize>, mut list2: Vec<usize>) -> usize {
    list1.sort();
    list2.sort();

    list1
        .into_iter()
        .zip(list2.into_iter())
        .map(|(e1, e2)| e1.abs_diff(e2))
        .sum()
}

fn main() {
    let file_contents = std::fs::read("input.txt").unwrap();
    let file_contents_as_str = std::str::from_utf8(&file_contents).unwrap();

    let (list1, list2) = parse_input(file_contents_as_str);

    println!("{}", part1(list1, list2));
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "3   4\n\
                              4   3\n\
                              2   5\n\
                              1   3\n\
                              3   9\n\
                              3   3";

    #[test]
    fn test_parse_input() {
        let (list1, list2) = parse_input(TEST_INPUT);

        assert_eq!(vec![3, 4, 2, 1, 3, 3], list1);
        assert_eq!(vec![4, 3, 5, 3, 9, 3], list2);
    }

    #[test]
    fn test_part1() {
        let (list1, list2) = parse_input(TEST_INPUT);

        assert_eq!(11, part1(list1, list2));
    }
}
