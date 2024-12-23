use smallvec::SmallVec;

use crate::computer::memory::Memory;

#[derive(Debug, PartialEq, Eq)]
pub struct PuzzleInput {
    pub memory: Memory,
    pub tribit_code: Box<[u8]>,
}

impl PuzzleInput {
    pub fn parse_input(input: &str) -> Self {
        let registers = input
            .lines()
            .take(3)
            .map(|line| line.rsplit_once(' ').unwrap().1.parse::<u64>().unwrap())
            .collect::<SmallVec<[u64; 3]>>()
            .into();

        let tribit_code = input
            .lines()
            .last()
            .unwrap()
            .rsplit_once(' ')
            .unwrap()
            .1
            .split(',')
            .map(|s| s.parse().unwrap())
            .collect::<Vec<_>>()
            .into_boxed_slice();

        PuzzleInput {
            memory: Memory {
                registers,
                instruction_pointer: 0,
            },
            tribit_code,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::computer::memory::Registers;
    use crate::tests::TEST_INPUT;

    use super::*;

    #[test]
    fn test_parse_input() {
        assert_eq!(
            PuzzleInput {
                memory: Memory {
                    registers: Registers { a: 729, b: 0, c: 0 },
                    instruction_pointer: 0
                },
                tribit_code: vec![0, 1, 5, 4, 3, 0].into_boxed_slice()
            },
            PuzzleInput::parse_input(TEST_INPUT)
        )
    }
}
