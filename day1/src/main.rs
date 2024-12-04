use std::collections::HashMap;

trait CollectToCounterExt<T> {
    fn collect_to_counter(self) -> HashMap<T, usize>;
}

impl<T: std::hash::Hash + std::cmp::Eq, U: Iterator<Item = T>> CollectToCounterExt<T> for U {
    fn collect_to_counter(self) -> HashMap<T, usize> {
        self.fold(HashMap::new(), |mut acc, elem| {
            *acc.entry(elem).or_insert(0) += 1;
            acc
        })
    }
}

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

fn part1(list1: &[usize], list2: &[usize]) -> usize {
    list1
        .iter()
        .zip(list2.iter())
        .map(|(&e1, &e2)| e1.abs_diff(e2))
        .sum()
}

fn part2(list1: &[usize], list2: &[usize]) -> usize {
    let counter1 = list1.iter().copied().collect_to_counter();
    let counter2 = list2.iter().copied().collect_to_counter();

    counter1
        .into_iter()
        .map(|(key, val)| key * val * (*counter2.get(&key).unwrap_or(&0)))
        .sum::<usize>()
}

fn main() {
    let file_contents = std::fs::read("input.txt").unwrap();
    let file_contents_as_str = std::str::from_utf8(&file_contents).unwrap();

    let (mut list1, mut list2) = parse_input(file_contents_as_str);
    list1.sort();
    list2.sort();

    let (list1, list2) = (list1, list2);

    println!("{}", part1(&list1, &list2));
    println!("{}", part2(&list1, &list2));
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
        let (mut list1, mut list2) = parse_input(TEST_INPUT);
        list1.sort();
        list2.sort();

        assert_eq!(11, part1(&list1, &list2));
    }

    #[test]
    fn test_part2() {
        let (list1, list2) = parse_input(TEST_INPUT);

        assert_eq!(31, part2(&list1, &list2));
    }
}
