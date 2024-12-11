use array2d::Array2D;

use crate::{antenna::Antenna, grid_cell::GridCell};

pub trait CityMap {
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
