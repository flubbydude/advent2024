use std::collections::{HashMap, HashSet};

use array2d::Array2D;
use itertools::Itertools;

trait CityMap {
    fn antennae_iter(&self) -> impl Iterator<Item = Antenna> + '_;
}

impl CityMap for Array2D<GridCell> {
    fn antennae_iter(&self) -> impl Iterator<Item = Antenna> {
        self.enumerate_row_major().filter_map(|(position, cell)| {
            if let &GridCell::Antenna { frequency } = cell {
                Some(Antenna {
                    frequency,
                    position,
                })
            } else {
                None
            }
        })
    }
}

struct Antenna {
    frequency: u8,
    position: (usize, usize),
}

fn get_antinodes(
    pos1: (usize, usize),
    pos2: (usize, usize),
    num_rows: usize,
    num_columns: usize,
) -> Vec<(usize, usize)> {
    // need to get 2v - w and 2w - v

    // idk if this even works but I love it?
    fn checked_2v_minus_w(v: (usize, usize), w: (usize, usize)) -> Option<(usize, usize)> {
        fn checked_2x_minus_y(x: usize, y: usize) -> Option<usize> {
            // try to compute 2 * x with checked addition
            let two_x_wrapped = match x.checked_add(x) {
                // If no overflow, try checked subtraction
                Some(z) => return z.checked_sub(y),
                // If overflow, wrapping add to get (2 * x) mod (usize::MAX + 1)
                None => x.wrapping_add(x),
            };

            // try to compute (2 * x) mod (usize::MAX + 1)
            // using checked subtraction.
            match two_x_wrapped.checked_sub(y) {
                // If there is no underflow, then there is overflow
                // in the calculation as a whole, so (2 * x) - y > usize::MAX
                Some(_) => None,
                // If there is underflow, then this is good as we go back to 2 * x - y
                // and (2 * x) - y <= usize::MAX
                None => Some(two_x_wrapped.wrapping_sub(y)),
            }
        }

        Some((checked_2x_minus_y(v.0, w.0)?, checked_2x_minus_y(v.1, w.1)?))
    }

    [
        checked_2v_minus_w(pos1, pos2),
        checked_2v_minus_w(pos2, pos1),
    ]
    .into_iter()
    .flatten()
    .filter(|&(i, j)| i < num_rows && j < num_columns)
    .collect()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum GridCell {
    Empty,
    Antenna { frequency: u8 },
}

impl TryFrom<char> for GridCell {
    type Error = &'static str;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        if value.is_ascii_alphanumeric() {
            Ok(GridCell::Antenna {
                frequency: value as u8,
            })
        } else if value == '.' {
            Ok(GridCell::Empty)
        } else {
            Err("Failed parsing {value} into GridCell")
        }
    }
}

fn parse_input(input: &str) -> Array2D<GridCell> {
    let num_rows = input.lines().count();
    let num_columns = input.lines().next().unwrap().len();

    Array2D::from_iter_row_major(
        input
            .lines()
            .flat_map(|line| line.chars().map(|c| c.try_into().unwrap())),
        num_rows,
        num_columns,
    )
    .unwrap()
}

fn part1(city_map: &Array2D<GridCell>) -> usize {
    let mut antenna_positions_by_freq: HashMap<u8, Vec<(usize, usize)>> = HashMap::new();
    for Antenna {
        frequency,
        position,
    } in city_map.antennae_iter()
    {
        antenna_positions_by_freq
            .entry(frequency)
            .or_default()
            .push(position);
    }

    let antenna_positions_by_freq = antenna_positions_by_freq;

    let antinode_positions = antenna_positions_by_freq
        .values()
        .flat_map(|positions| {
            positions
                .iter()
                .copied()
                .tuple_combinations()
                .flat_map(|(pos1, pos2)| {
                    get_antinodes(pos1, pos2, city_map.num_rows(), city_map.num_columns())
                })
        })
        .collect::<HashSet<_>>();

    antinode_positions.len()
}

fn main() {
    let file_contents_as_str = include_str!("../input.txt");

    let input = parse_input(file_contents_as_str);

    println!("{}", part1(&input));
}

#[cfg(test)]
mod tests {
    use std::iter::repeat_n;

    use super::*;

    const TEST_INPUT: &str = "............\n\
                              ........0...\n\
                              .....0......\n\
                              .......0....\n\
                              ....0.......\n\
                              ......A.....\n\
                              ............\n\
                              ............\n\
                              ........A...\n\
                              .........A..\n\
                              ............\n\
                              ............";

    #[test]
    fn test_parse_input() {
        let input = parse_input(TEST_INPUT);

        assert_eq!(
            repeat_n(b'0', 4)
                .chain(repeat_n(b'A', 3))
                .collect::<Vec<_>>(),
            input
                .antennae_iter()
                .map(|a| a.frequency)
                .collect::<Vec<_>>()
        );
    }

    #[test]
    fn test_part1() {
        let input = parse_input(TEST_INPUT);
        assert_eq!(14, part1(&input))
    }
}
