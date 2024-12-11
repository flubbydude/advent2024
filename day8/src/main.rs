mod antenna;
mod city_map;
mod grid_cell;

use std::{
    collections::{HashMap, HashSet},
    iter,
};

use antenna::Antenna;
use array2d::Array2D;
use city_map::CityMap;
use grid_cell::GridCell;
use itertools::Itertools;
use num::Integer;

fn get_antinodes_part1(
    pos1: (usize, usize),
    pos2: (usize, usize),
    num_rows: usize,
    num_columns: usize,
) -> impl Iterator<Item = (usize, usize)> {
    // need to get 2v - w and 2w - v

    // idk if this even works but I love it?
    fn checked_2v_minus_w(v: (usize, usize), w: (usize, usize)) -> Option<(usize, usize)> {
        fn checked_2x_minus_y(x: usize, y: usize) -> Option<usize> {
            // assume x + x can't overflow lol
            (x + x).checked_sub(y)
        }

        Some((checked_2x_minus_y(v.0, w.0)?, checked_2x_minus_y(v.1, w.1)?))
    }

    [
        checked_2v_minus_w(pos1, pos2),
        checked_2v_minus_w(pos2, pos1),
    ]
    .into_iter()
    .flatten()
    .filter(move |&(i, j)| i < num_rows && j < num_columns)
}

fn get_antinodes_part2(
    pos1: (usize, usize),
    pos2: (usize, usize),
    num_rows: usize,
    num_columns: usize,
) -> impl Iterator<Item = (usize, usize)> {
    let delta = if pos1.0 == pos2.0 {
        (0, 1)
    } else if pos1.1 == pos2.1 {
        (1, 0)
    } else {
        let gcd = pos2.0.abs_diff(pos1.0).gcd(&pos2.1.abs_diff(pos1.1));

        let delta_i = (pos2.0 as isize - pos1.0 as isize) / (gcd as isize);
        let delta_j = (pos2.1 as isize - pos1.1 as isize) / (gcd as isize);

        (delta_i, delta_j)
    };

    let create_iter = |mut start: (usize, usize), delta: (isize, isize)| {
        let mut done = false;
        iter::from_fn(move || {
            if done {
                return None;
            }

            let result = start;
            match start.0.checked_add_signed(delta.0) {
                Some(i) => {
                    if i >= num_rows {
                        done = true;
                    } else {
                        start.0 = i;
                    }
                }
                None => done = true,
            };
            match start.1.checked_add_signed(delta.1) {
                Some(j) => {
                    if j >= num_columns {
                        done = true;
                    } else {
                        start.1 = j;
                    }
                }
                None => done = true,
            };

            Some(result)
        })
    };

    create_iter(pos1, delta).chain(create_iter(pos1, (-delta.0, -delta.1)))
}

fn parse_input(input: &str) -> Array2D<GridCell> {
    let num_rows = input.lines().count();
    let num_columns = input.lines().next().unwrap().len();

    // now we can always change to isize
    if num_rows > (isize::MAX / 2) as usize || num_columns > (isize::MAX / 2) as usize {
        panic!("Now this is crazy!");
    }

    Array2D::from_iter_row_major(
        input
            .lines()
            .flat_map(|line| line.chars().map(|c| c.try_into().unwrap())),
        num_rows,
        num_columns,
    )
    .unwrap()
}

fn run<T>(
    city_map: &Array2D<GridCell>,
    get_antinodes: fn((usize, usize), (usize, usize), usize, usize) -> T,
) -> usize
where
    T: Iterator<Item = (usize, usize)>,
{
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

    println!("{}", run(&input, get_antinodes_part1));
    println!("{}", run(&input, get_antinodes_part2));
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
        assert_eq!(14, run(&input, get_antinodes_part1))
    }

    #[test]
    fn test_part2() {
        let input = parse_input(TEST_INPUT);
        assert_eq!(34, run(&input, get_antinodes_part2))
    }
}
