use std::iter::repeat_n;

fn parse_input(input: &[u8]) -> Vec<u8> {
    input
        .iter()
        .copied()
        .map(|c| {
            if !c.is_ascii_digit() {
                panic!("{c} in input is not a digit");
            } else {
                c - b'0'
            }
        })
        .collect()
}

fn part1(input: &[u8]) -> usize {
    // even indices are files
    let num_file_blocks = input
        .iter()
        .copied()
        .enumerate()
        .filter_map(|(i, file_len)| {
            if i % 2 == 0 {
                Some(file_len as usize)
            } else {
                None
            }
        })
        .sum::<usize>();

    let mut files_from_front = input
        .iter()
        .copied()
        .enumerate()
        .filter(|(i, _)| i % 2 == 0)
        .flat_map(|(i, file_len)| repeat_n(i / 2, file_len as usize));

    let mut files_from_back = input
        .iter()
        .copied()
        .enumerate()
        .rev()
        .filter(|(i, _)| i % 2 == 0)
        .flat_map(|(i, file_len)| repeat_n(i / 2, file_len as usize));

    input
        .iter()
        .copied()
        .enumerate()
        .flat_map(|(i, length)| {
            if i % 2 == 0 {
                (&mut files_from_front)
                    .take(length as usize)
                    .collect::<Vec<_>>()
            } else {
                (&mut files_from_back)
                    .take(length as usize)
                    .collect::<Vec<_>>()
            }
        })
        .take(num_file_blocks)
        .enumerate()
        .map(|(pos, file_id)| pos * file_id)
        .sum()
}

fn main() {
    let file_contents = include_bytes!("../input.txt");
    let input = parse_input(file_contents);

    println!("{}", part1(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT_SMALL: &[u8] = b"12345";
    const TEST_INPUT: &[u8] = b"2333133121414131402";

    #[test]
    fn test_parse_input() {
        assert_eq!(vec![1, 2, 3, 4, 5], parse_input(TEST_INPUT_SMALL));
    }

    #[test]
    fn test_part1() {
        let input = parse_input(TEST_INPUT);
        assert_eq!(1928, part1(&input))
    }
}
