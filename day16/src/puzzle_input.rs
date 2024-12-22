use std::fmt::{Display, Formatter};

use array2d::Array2D;

use crate::grid_cell::GridCell;

pub struct PuzzleInput {
    pub grid: Array2D<GridCell>,
    pub start_position: (usize, usize),
    pub end_position: (usize, usize),
}

#[derive(Debug)]
pub enum Error {
    MultipleStartPositions((usize, usize), (usize, usize)),
    MultipleEndPositions((usize, usize), (usize, usize)),
    Array2D(array2d::Error),
    NoStartPosition,
    NoEndPosition,
    UnrecognizedCharacter(char),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        use Error::*;
        match self {
            MultipleStartPositions(position1, position2) => write!(
                f,
                "Multiple start positions found in the input: {position1:?}, {position2:?}"
            ),
            MultipleEndPositions(position1, position2) => write!(
                f,
                "Multiple end positions found in the input: {position1:?}, {position2:?}"
            ),
            Array2D(error) => write!(f, "Received error when parsing into Array2D: '{error}'"),
            NoStartPosition => write!(f, "No start position found in the input"),
            NoEndPosition => write!(f, "No end position found in the input"),
            UnrecognizedCharacter(c) => {
                write!(f, "Found unrecognized character '{c}' in the input")
            }
        }
    }
}

impl PuzzleInput {
    pub fn parse_from_input(input: &str) -> Result<PuzzleInput, Error> {
        let mut maybe_start_position = None;
        let mut maybe_end_position = None;

        let vecs = input
            .lines()
            .enumerate()
            .map(|(i, line)| {
                line.chars()
                    .enumerate()
                    .map(|(j, c)| match c {
                        'S' => {
                            if let Some(prev_pos) = maybe_start_position {
                                Err(Error::MultipleStartPositions((i, j), prev_pos))
                            } else {
                                maybe_start_position = Some((i, j));
                                Ok(GridCell::Empty)
                            }
                        }
                        'E' => {
                            if let Some(prev_pos) = maybe_end_position {
                                Err(Error::MultipleEndPositions((i, j), prev_pos))
                            } else {
                                maybe_end_position = Some((i, j));
                                Ok(GridCell::Empty)
                            }
                        }
                        _ => c.try_into().map_err(Error::UnrecognizedCharacter),
                    })
                    .collect::<Result<Vec<_>, _>>()
            })
            .collect::<Result<Vec<_>, _>>()?;

        let grid = Array2D::from_rows(&vecs).map_err(Error::Array2D)?;
        let start_position = maybe_start_position.ok_or(Error::NoStartPosition)?;
        let end_position = maybe_end_position.ok_or(Error::NoEndPosition)?;

        Ok(PuzzleInput {
            grid,
            start_position,
            end_position,
        })
    }
}
