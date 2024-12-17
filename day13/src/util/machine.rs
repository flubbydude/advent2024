use itertools::Itertools;

use super::button::Button;
use super::prize::Prize;

#[derive(Debug, PartialEq, Eq)]
pub struct Machine {
    a_button: Button,
    b_button: Button,
    prize: Prize,
}

impl Machine {
    pub fn fewest_tokens_to_win(&self) -> Option<usize> {
        // solve where:
        // - a.x * a_presses + b.x * b_presses = prize.x
        // - a.y * a_presses + b.y * b_presses = prize.y
        //
        // minimize number of A presses since A costs 3 and B costs 1
        // return a_presses * 3 + b_presses

        // modular arithmetic?
        todo!()
    }
}

pub fn parse_machines(input: &str) -> Vec<Machine> {
    input
        .lines()
        .chunk_by(|&line| !line.is_empty())
        .into_iter()
        .filter_map(|(key, chunk)| if key { Some(chunk) } else { None })
        .map(|mut chunk| {
            let (a_str, b_str, prize_str) = chunk
                .next_tuple()
                .expect("Chunk in input does not contain 3 lines");

            Machine {
                a_button: Button::from_a_button_str(a_str),
                b_button: Button::from_b_button_str(b_str),
                prize: prize_str.into(),
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("../../example.txt");

    #[test]
    fn test_parse_machines() {
        let machines = parse_machines(TEST_INPUT);

        let expected_machines = vec![
            Machine {
                a_button: Button { x: 94, y: 34 },
                b_button: Button { x: 22, y: 67 },
                prize: Prize { x: 8400, y: 5400 },
            },
            Machine {
                a_button: Button { x: 26, y: 66 },
                b_button: Button { x: 67, y: 21 },
                prize: Prize { x: 12748, y: 12176 },
            },
            Machine {
                a_button: Button { x: 17, y: 86 },
                b_button: Button { x: 84, y: 37 },
                prize: Prize { x: 7870, y: 6450 },
            },
            Machine {
                a_button: Button { x: 69, y: 23 },
                b_button: Button { x: 27, y: 71 },
                prize: Prize { x: 18641, y: 10279 },
            },
        ];

        assert_eq!(expected_machines, machines);
    }
}
