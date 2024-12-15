use array2d::Array2D;
use smallvec::SmallVec;

fn parse_input(input: &str) -> Array2D<u8> {
    let num_rows = input.lines().count();
    let num_columns = input.lines().next().unwrap().chars().count();

    Array2D::from_iter_row_major(
        input
            .lines()
            .flat_map(|line| line.as_bytes().iter().copied()),
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

fn part1(grid: &Array2D<u8>) -> usize {
    let mut seen = Array2D::filled_with(false, grid.num_rows(), grid.num_columns());
    let mut result = 0;

    for start in grid.indices_row_major() {
        if seen[start] {
            continue;
        }

        seen[start] = true;

        let region_plant = grid[start];
        let mut stack = vec![start];
        let mut area = 0;
        let mut perimeter = 0;

        while let Some(position) = stack.pop() {
            area += 1;
            perimeter += 4;

            for neighbor in get_neighbors(position, grid.num_rows(), grid.num_columns()) {
                if grid[neighbor] == region_plant {
                    perimeter -= 1;

                    if !seen[neighbor] {
                        seen[neighbor] = true;
                        stack.push(neighbor);
                    }
                }
            }
        }

        result += area * perimeter;
    }

    result
}

fn part2(grid: &Array2D<u8>) -> usize {
    todo!()
}

fn main() {
    let file_contents_as_str = include_str!("../input.txt");

    let input = parse_input(file_contents_as_str);

    println!("{}", part1(&input));
    println!("{}", part2(&input));
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use super::*;

    const TEST_INPUT_SMALL: &str = "AAAA\n\
                                    BBCD\n\
                                    BBCC\n\
                                    EEEC";

    const TEST_INPUT_MEDIUM: &str = "OOOOO\n\
                                     OXOXO\n\
                                     OOOOO\n\
                                     OXOXO\n\
                                     OOOOO";

    const TEST_INPUT_BIG: &str = "RRRRIICCFF\n\
                                  RRRRIICCCF\n\
                                  VVRRRCCFFF\n\
                                  VVRCCCJFFF\n\
                                  VVVVCJJCFE\n\
                                  VVIVCCJJEE\n\
                                  VVIIICJJEE\n\
                                  MIIIIIJJEE\n\
                                  MIIISIJEEE\n\
                                  MMMISSJEEE";

    const TEST_INPUT_E: &str = "EEEEE\n\
                                EXXXX\n\
                                EEEEE\n\
                                EXXXX\n\
                                EEEEE";

    const TEST_INPUT_AB: &str = "AAAAAA\n\
                                 AAABBA\n\
                                 AAABBA\n\
                                 ABBAAA\n\
                                 ABBAAA\n\
                                 AAAAAA";

    #[test_case(TEST_INPUT_SMALL => 140 ; "small")]
    #[test_case(TEST_INPUT_MEDIUM => 772 ; "medium")]
    #[test_case(TEST_INPUT_BIG => 1930 ; "big")]
    fn test_part1(input_str: &str) -> usize {
        part1(&parse_input(input_str))
    }

    #[test_case(TEST_INPUT_SMALL => 80 ; "small")]
    #[test_case(TEST_INPUT_MEDIUM => 436 ; "medium")]
    #[test_case(TEST_INPUT_E => 236 ; "big e")]
    #[test_case(TEST_INPUT_AB => 368 ; "ab")]
    #[test_case(TEST_INPUT_BIG => 1206 ; "big")]
    fn test_part2(input_str: &str) -> usize {
        part2(&parse_input(input_str))
    }
}
