use std::str::FromStr;

use itertools::Itertools;
use thiserror::Error;

pub const LOCK_KEY_WIDTH: usize = 5;
pub const LOCK_KEY_HEIGHT: usize = 7;

enum LockOrKey {
    Lock([u8; LOCK_KEY_WIDTH]),
    Key([u8; LOCK_KEY_WIDTH]),
}

#[derive(Debug, Error)]
pub enum LockOrKeyParseError {
    #[error("Lock or key has a row with width {0} != {LOCK_KEY_WIDTH}")]
    BadWidth(usize),
    #[error("Lock or key has height {0} != {LOCK_KEY_HEIGHT}")]
    BadHeight(usize),
    #[error("Unknown character {0}")]
    UnknownCharacter(char),
    #[error("Them top or bottom rows is bad")]
    NotLockNorKey,
    #[error("Them columns is bad")]
    BadLockOrKey,
}

impl FromStr for LockOrKey {
    type Err = LockOrKeyParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let line_count = s.lines().count();
        if line_count != LOCK_KEY_HEIGHT {
            return Err(LockOrKeyParseError::BadHeight(line_count));
        }

        s.lines().try_for_each(|line| {
            let line_width = line.chars().count();
            if line_width != LOCK_KEY_WIDTH {
                return Err(LockOrKeyParseError::BadWidth(line_width));
            }

            line.chars()
                .find(|c| !matches!(c, '#' | '.'))
                .map_or(Ok(()), |c| Err(LockOrKeyParseError::UnknownCharacter(c)))?;

            Ok(())
        })?;

        fn helper<'a, I>(lines: I) -> Result<[u8; LOCK_KEY_WIDTH], LockOrKeyParseError>
        where
            I: Iterator<Item = &'a str>,
        {
            // lines is iterator of after the first line and before the last line
            let mut result = [0; LOCK_KEY_WIDTH];
            for (line_num, line) in lines.enumerate() {
                for (c, result_elem) in line.chars().zip(result.iter_mut()) {
                    if c == '.' {
                        continue;
                    }

                    if *result_elem == line_num as u8 {
                        *result_elem += 1;
                    } else {
                        return Err(LockOrKeyParseError::BadLockOrKey);
                    }
                }
            }

            Ok(result)
        }

        let mut lines = s.lines();
        let first_line = lines.next().unwrap();
        let last_line = lines.next_back().unwrap();

        if first_line.chars().all(|c| c == '#') && last_line.chars().all(|c| c == '.') {
            Ok(LockOrKey::Lock(helper(lines)?))
        } else if first_line.chars().all(|c| c == '.') && last_line.chars().all(|c| c == '#') {
            Ok(LockOrKey::Key(helper(lines.rev())?))
        } else {
            Err(LockOrKeyParseError::NotLockNorKey)
        }
    }
}

pub struct PuzzleInput {
    pub locks: Vec<[u8; LOCK_KEY_WIDTH]>,
    pub keys: Vec<[u8; LOCK_KEY_WIDTH]>,
}

pub fn parse_input(input: &str) -> Result<PuzzleInput, LockOrKeyParseError> {
    let chunks = input.lines().chunk_by(|line| line.is_empty());

    let lock_or_keys = chunks
        .into_iter()
        .filter(|&(empty, _)| !empty)
        .map(|(_, group)| group.into_iter().join("\n"))
        .map(|lock_or_key_string| LockOrKey::from_str(&lock_or_key_string));

    let mut locks = Vec::new();
    let mut keys = Vec::new();

    for lock_or_key in lock_or_keys {
        match lock_or_key? {
            LockOrKey::Lock(lock) => locks.push(lock),
            LockOrKey::Key(key) => keys.push(key),
        }
    }

    Ok(PuzzleInput { locks, keys })
}
