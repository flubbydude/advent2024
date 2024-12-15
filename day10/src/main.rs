use std::collections::HashMap;

use array2d::Array2D;
use smallvec::SmallVec;

fn parse_input(input: &str) -> Array2D<u8> {
    let num_rows = input.lines().count();
    let num_columns = input.lines().next().unwrap().chars().count();

    Array2D::from_iter_row_major(
        input
            .lines()
            .flat_map(|line| line.chars())
            .map(|c| (c as u8) - b'0'),
        num_rows,
        num_columns,
    )
    .unwrap()
}

fn get_neighbors(
    position: (usize, usize),
    num_rows: usize,
    num_columns: usize,
) -> SmallVec<[(usize, usize); 4]> {
    let mut result = SmallVec::new();
    if position.0 > 0 {
        result.push((position.0 - 1, position.1));
    }
    if position.0 + 1 < num_rows {
        result.push((position.0 + 1, position.1));
    }
    if position.1 > 0 {
        result.push((position.0, position.1 - 1));
    }
    if position.1 + 1 < num_columns {
        result.push((position.0, position.1 + 1));
    }
    result
}

fn run(grid: &Array2D<u8>) -> HashMap<(usize, usize), HashMap<(usize, usize), usize>> {
    let mut frontier: HashMap<(usize, usize), HashMap<(usize, usize), usize>> = grid
        .enumerate_row_major()
        .filter_map(|(pos, &elem)| {
            if elem == 9 {
                Some((pos, HashMap::from([(pos, 1)])))
            } else {
                None
            }
        })
        .collect();

    for height in (0..9).rev() {
        let mut next_frontier = HashMap::new();
        for (position, num_ways_to_get_to_nines) in frontier {
            for neighbor in get_neighbors(position, grid.num_rows(), grid.num_columns()) {
                if grid[neighbor] == height {
                    next_frontier
                        .entry(neighbor)
                        .and_modify(|to_map: &mut HashMap<(usize, usize), usize>| {
                            for (&nine_pos, &num_ways) in num_ways_to_get_to_nines.iter() {
                                to_map
                                    .entry(nine_pos)
                                    .and_modify(|count| *count += num_ways)
                                    .or_insert(num_ways);
                            }
                        })
                        .or_insert(num_ways_to_get_to_nines.clone());
                }
            }
        }
        frontier = next_frontier;
    }

    frontier
}

fn part1(grid: &Array2D<u8>) -> usize {
    run(grid)
        .into_values()
        .map(|num_ways_to_get_to_nines| num_ways_to_get_to_nines.len())
        .sum()
}

fn part2(grid: &Array2D<u8>) -> usize {
    run(grid).into_values().flat_map(HashMap::into_values).sum()
}

fn main() {
    let file_contents_as_str = include_str!("../input.txt");

    let grid = parse_input(file_contents_as_str);

    println!("{}", part1(&grid));
    println!("{}", part2(&grid));
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "89010123\n\
                              78121874\n\
                              87430965\n\
                              96549874\n\
                              45678903\n\
                              32019012\n\
                              01329801\n\
                              10456732";

    #[test]
    fn test_parse_input() {
        let grid = parse_input(TEST_INPUT);
        assert_eq!(
            vec![8, 9, 0, 1, 0, 1, 2, 3],
            grid.row_iter(0).unwrap().copied().collect::<Vec<_>>()
        );
        assert_eq!(
            vec![7, 8, 1, 2, 1, 8, 7, 4],
            grid.row_iter(1).unwrap().copied().collect::<Vec<_>>()
        );
        assert_eq!(8, grid.num_rows());
        assert_eq!(8, grid.num_columns());
    }

    #[test]
    fn test_part1() {
        let grid = parse_input(TEST_INPUT);
        assert_eq!(36, part1(&grid));
    }

    #[test]
    fn test_part2() {
        let grid = parse_input(TEST_INPUT);
        assert_eq!(81, part2(&grid));
    }
}
