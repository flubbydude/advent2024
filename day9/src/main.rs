mod contiguous_space_iter;

use std::{iter::repeat_n, mem};

use contiguous_space_iter::{ContiguousDiskSpace, ContiguousDiskSpaceIter, DiskFile, EmptySpace};

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

fn combine_empties(contiguous_disk_spaces: &mut Vec<ContiguousDiskSpace>, i: usize) -> usize {
    // given: contiguous_disk_spaces[i] is empty.
    // combine empties of contiguous_disk_spaces[i - 1], contiguous_disk_spaces[i],
    // contiguous_disk_spaces[i + 1]

    // return the next index after original i and after the removal (either i or i + 1)

    let ContiguousDiskSpace::EmptySpace(EmptySpace { length: cur_length }) =
        contiguous_disk_spaces[i]
    else {
        panic!()
    };

    let maybe_next_elem = contiguous_disk_spaces.get(i + 1);
    let maybe_next_length = maybe_next_elem.and_then(|elem| match elem {
        &ContiguousDiskSpace::EmptySpace(EmptySpace { length }) => Some(length),
        _ => None,
    });

    if let Some(ContiguousDiskSpace::EmptySpace(EmptySpace {
        length: prev_length,
    })) = i
        .checked_sub(1)
        .and_then(|prev_index| contiguous_disk_spaces.get_mut(prev_index))
    {
        *prev_length += cur_length;
        match maybe_next_length {
            Some(next_length) => {
                *prev_length += next_length;
                contiguous_disk_spaces.drain(i..=i + 1);
            }
            None => {
                contiguous_disk_spaces.remove(i);
            }
        }
        return i;
    }

    if let Some(next_length) = maybe_next_length {
        let ContiguousDiskSpace::EmptySpace(EmptySpace { length: cur_length }) =
            &mut contiguous_disk_spaces[i]
        else {
            panic!()
        };

        *cur_length += next_length;
        contiguous_disk_spaces.remove(i + 1);
    }

    i + 1
}

fn part2(input: &[u8]) -> usize {
    let mut contiguous_disk_spaces = ContiguousDiskSpaceIter::new(input).collect::<Vec<_>>();
    contiguous_disk_spaces.reverse();

    let mut i = 0;
    let final_file = contiguous_disk_spaces
        .iter()
        .filter_map(|space| {
            if let ContiguousDiskSpace::File(disk_file) = space {
                Some(disk_file)
            } else {
                None
            }
        })
        .next();

    let mut prev_id = match final_file {
        Some(&DiskFile { id, .. }) => id,
        None => return 0,
    };

    while i < contiguous_disk_spaces.len() {
        let ContiguousDiskSpace::File(disk_file) = &contiguous_disk_spaces[i] else {
            i += 1;
            continue;
        };

        // already moved this file
        if disk_file.id > prev_id {
            i += 1;
            continue;
        }

        prev_id = disk_file.id;

        let Some((j, empty_space)) = contiguous_disk_spaces
            .iter()
            .enumerate()
            .rev()
            .take_while(|&(j, _)| j > i)
            .filter_map(|(j, space)| {
                if let ContiguousDiskSpace::EmptySpace(empty_space) = space {
                    if empty_space.length >= disk_file.length {
                        return Some((j, empty_space));
                    }
                }
                None
            })
            .next()
        else {
            i += 1;
            continue;
        };

        if empty_space.length == disk_file.length {
            contiguous_disk_spaces.swap(i, j);
            i = combine_empties(&mut contiguous_disk_spaces, i);
        } else {
            let disk_file_length = disk_file.length;

            let disk_file_to_insert = mem::replace(
                &mut contiguous_disk_spaces[i],
                ContiguousDiskSpace::EmptySpace(EmptySpace {
                    length: disk_file_length,
                }),
            );
            let ContiguousDiskSpace::EmptySpace(e) = &mut contiguous_disk_spaces[j] else {
                panic!()
            };
            e.length -= disk_file_length;
            contiguous_disk_spaces.insert(j + 1, disk_file_to_insert);
            i = combine_empties(&mut contiguous_disk_spaces, i);
        }
    }

    contiguous_disk_spaces.reverse();

    let mut cur_index = 0;
    contiguous_disk_spaces
        .into_iter()
        .filter_map(|contiguous_disk_space| {
            match contiguous_disk_space {
                ContiguousDiskSpace::File(DiskFile { id, length }) => {
                    // sum of cur_index * id + (cur_index + 1) * id + ... + (cur_index + length - 1) * id
                    let result = (id * length * (cur_index * 2 + length - 1)) / 2;
                    cur_index += length;
                    Some(result)
                }
                ContiguousDiskSpace::EmptySpace(EmptySpace { length }) => {
                    cur_index += length;
                    None
                }
            }
        })
        .sum()
}

fn main() {
    let file_contents = include_bytes!("../input.txt");
    let input = parse_input(file_contents);

    println!("{}", part1(&input));
    println!("{}", part2(&input));
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

    #[test]
    fn test_part2() {
        let input = parse_input(TEST_INPUT);
        assert_eq!(2858, part2(&input))
    }
}
