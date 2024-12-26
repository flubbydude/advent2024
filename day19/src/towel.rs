use smallvec::SmallVec;

use crate::color::Color;

const TOWEL_PATTERN_MAX_LEN: usize = 8;

pub struct Towel {
    pattern: SmallVec<[Color; TOWEL_PATTERN_MAX_LEN]>,
}

impl Towel {
    pub fn pattern(&self) -> &[Color] {
        &self.pattern
    }
}

impl FromIterator<Color> for Towel {
    fn from_iter<T: IntoIterator<Item = Color>>(iter: T) -> Self {
        Towel {
            pattern: iter.into_iter().collect(),
        }
    }
}
