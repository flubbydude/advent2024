use std::mem::swap;

use array2d::Array2D;

#[derive(Clone, Copy)]
struct Position(usize, usize);

#[derive(Clone, Copy)]
struct Direction(isize, isize);

impl Direction {
    fn turn_cw(&mut self) {
        // let &Direction(i, j) = self;
        // Direction(j, -i)

        swap(&mut self.0, &mut self.1);
        self.1 *= -1;
    }
}

impl Position {
    fn add(&self, direction: Direction, num_rows: usize, num_columns: usize) -> Option<Position> {
        let new_i = self.0 as isize + direction.0;
        let new_j = self.1 as isize + direction.1;

        if new_i < 0 || new_j < 0 {
            return None;
        }

        let new_i = new_i as usize;
        let new_j = new_j as usize;

        if new_i >= num_rows || new_j >= num_columns {
            return None;
        }

        Some(Position(new_i, new_j))
    }

    fn to_tuple(&self) -> (usize, usize) {
        let &Position(i, j) = self;
        (i, j)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum InputCell {
    Obstacle,
    Empty,
    Start,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum MapCell {
    Obstacle,
    Empty,
}

impl From<InputCell> for MapCell {
    fn from(value: InputCell) -> Self {
        match value {
            InputCell::Obstacle => MapCell::Obstacle,
            InputCell::Empty => MapCell::Empty,
            InputCell::Start => MapCell::Empty,
        }
    }
}

fn parse_input_grid(input_str: &str) -> Array2D<InputCell> {
    let num_rows = input_str.lines().count();
    let num_columns = input_str.lines().next().unwrap().chars().count();

    Array2D::from_iter_row_major(
        input_str
            .chars()
            .filter(|c| !c.is_ascii_whitespace())
            .map(|c| match c {
                '#' => InputCell::Obstacle,
                '.' => InputCell::Empty,
                '^' => InputCell::Start,
                _ => panic!("Unknown character {c} in input"),
            }),
        num_rows,
        num_columns,
    )
    .unwrap()
}

struct PuzzleInput {
    map_grid: Array2D<MapCell>,
    start_pos: Position,
}

impl PuzzleInput {
    fn from_input_grid(input_grid: &Array2D<InputCell>) -> PuzzleInput {
        let start_pos = input_grid
            .enumerate_row_major()
            .find_map(|((i, j), cell)| {
                if matches!(cell, &InputCell::Start) {
                    Some(Position(i, j))
                } else {
                    None
                }
            })
            .expect("Puzzle doesn't have start position");

        let map_grid = Array2D::from_iter_row_major(
            input_grid
                .elements_row_major_iter()
                .cloned()
                .map(MapCell::from),
            input_grid.num_rows(),
            input_grid.num_columns(),
        )
        .unwrap();

        PuzzleInput {
            map_grid,
            start_pos,
        }
    }
}

fn parse_input(input_str: &str) -> PuzzleInput {
    PuzzleInput::from_input_grid(&parse_input_grid(input_str))
}

fn part1(input: &PuzzleInput) -> usize {
    let PuzzleInput {
        map_grid,
        start_pos,
    } = input;

    let num_rows = map_grid.num_rows();
    let num_columns = map_grid.num_columns();

    let mut visited_grid = Array2D::filled_with(false, num_rows, num_columns);

    let mut cur_pos = *start_pos;
    let mut cur_direction = Direction(-1, 0);

    loop {
        visited_grid[cur_pos.to_tuple()] = true;

        let in_front = match cur_pos.add(cur_direction, num_rows, num_columns) {
            Some(pos) => pos,
            None => break,
        };

        if map_grid[in_front.to_tuple()] == MapCell::Obstacle {
            cur_direction.turn_cw();
        } else {
            cur_pos = in_front;
        }
    }

    visited_grid
        .elements_row_major_iter()
        .filter(|&&b| b)
        .count()
}

// fn part2(input: &str) -> usize {
//     todo!()
// }

fn main() {
    let file_contents = std::fs::read("input.txt").unwrap();
    let file_contents_as_str = std::str::from_utf8(&file_contents).unwrap();

    let input = parse_input(file_contents_as_str);

    println!("{}", part1(&input));
    // println!("{}", part2(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "....#.....\n\
                              .........#\n\
                              ..........\n\
                              ..#.......\n\
                              .......#..\n\
                              ..........\n\
                              .#..^.....\n\
                              ........#.\n\
                              #.........\n\
                              ......#...";

    #[test]
    fn test_parse_input() {
        let grid = parse_input_grid(".#\n^#");
        let expected_grid = Array2D::from_rows(&[
            vec![InputCell::Empty, InputCell::Obstacle],
            vec![InputCell::Start, InputCell::Obstacle],
        ])
        .unwrap();

        assert_eq!(expected_grid, grid);
    }

    #[test]
    fn test_part1() {
        let input = parse_input(TEST_INPUT);
        assert_eq!(41, part1(&input))
    }

    // #[test]
    // fn test_part2() {
    //     let input = parse_input(TEST_INPUT);
    //     assert_eq!(0, part2(input))
    // }
}
