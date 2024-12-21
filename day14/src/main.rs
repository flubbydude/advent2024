mod bounds;
mod puzzle_input;
mod quadrant;
mod robot;

use enum_iterator::cardinality;

use puzzle_input::PuzzleInput;
use quadrant::Quadrant;
use robot::Robot;

fn part1(puzzle_input: &PuzzleInput) -> usize {
    const NUM_STEPS: usize = 100;

    let mut counts_by_quadrant: [usize; cardinality::<Quadrant>()] = [0; cardinality::<Quadrant>()];

    for robot in puzzle_input.robots() {
        let mut robot = robot.clone();

        for _ in 0..NUM_STEPS {
            robot.step(puzzle_input.bounds());
        }

        if let Some(quadrant) = robot.get_quadrant(puzzle_input.bounds()) {
            counts_by_quadrant[quadrant as usize] += 1;
        }
    }

    counts_by_quadrant.into_iter().product()
}

fn part2(mut puzzle_input: PuzzleInput) {
    const NUM_STEPS: usize = 6752;
    let bounds = puzzle_input.bounds().clone();

    for _ in 0..NUM_STEPS {
        for robot in puzzle_input.robots_mut() {
            robot.step(&bounds);
        }
    }

    println!("{}", puzzle_input.board_as_str());
}

fn main() {
    let file_contents_as_str = include_str!("../input.txt");

    let num_rows = 103;
    let num_columns = 101;
    let robots = file_contents_as_str.lines().map(Robot::from).collect();
    let puzzle_input = PuzzleInput::new(num_rows, num_columns, robots);

    println!("{}", part1(&puzzle_input));
    part2(puzzle_input);
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "p=0,4 v=3,-3\n\
                              p=6,3 v=-1,-3\n\
                              p=10,3 v=-1,2\n\
                              p=2,0 v=2,-1\n\
                              p=0,0 v=1,3\n\
                              p=3,0 v=-2,-2\n\
                              p=7,6 v=-1,-3\n\
                              p=3,0 v=-1,-2\n\
                              p=9,3 v=2,3\n\
                              p=7,3 v=-1,2\n\
                              p=2,4 v=2,-3\n\
                              p=9,5 v=-3,-3";

    #[test]
    fn test_part1() {
        let num_rows = 7;
        let num_columns = 11;
        let robots = TEST_INPUT.lines().map(Robot::from).collect();
        let puzzle_input = PuzzleInput::new(num_rows, num_columns, robots);
        assert_eq!(12, part1(&puzzle_input))
    }
}
