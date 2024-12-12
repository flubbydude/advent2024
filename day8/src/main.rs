mod antenna;
mod antinode_iter;
mod city_map;
mod grid_cell;

use std::collections::HashSet;

use antinode_iter::AntinodeIter;
use array2d::Array2D;
use city_map::CityMap;
use itertools::Itertools;

fn get_antinodes_part1(
    pos1: (usize, usize),
    pos2: (usize, usize),
    num_rows: usize,
    num_columns: usize,
) -> impl Iterator<Item = (usize, usize)> {
    AntinodeIter::new(pos1, pos2, pos1, num_rows, num_columns)
        .skip(1)
        .next()
        .into_iter()
        .chain(
            AntinodeIter::new(pos2, pos1, pos2, num_rows, num_columns)
                .skip(1)
                .next()
                .into_iter(),
        )
}

fn get_antinodes_part2(
    pos1: (usize, usize),
    pos2: (usize, usize),
    num_rows: usize,
    num_columns: usize,
) -> impl Iterator<Item = (usize, usize)> {
    AntinodeIter::new(pos2, pos2, pos1, num_rows, num_columns)
        .skip(1)
        .chain(AntinodeIter::new(pos2, pos1, pos2, num_rows, num_columns))
}

fn parse_input(input: &str) -> CityMap {
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
    .into()
}

fn run<T>(
    city_map: &CityMap,
    get_antinodes: fn((usize, usize), (usize, usize), usize, usize) -> T,
) -> usize
where
    T: Iterator<Item = (usize, usize)>,
{
    let antenna_positions_by_freq = city_map.get_antenna_positions_by_freq();

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
