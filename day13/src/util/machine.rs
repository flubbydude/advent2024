use itertools::Itertools;

use num_integer::{ExtendedGcd, Integer};

use super::button::Button;
use super::prize::Prize;

#[derive(Debug, PartialEq, Eq)]
pub struct Machine {
    a_button: Button,
    b_button: Button,
    prize: Prize,
}

impl Machine {
    fn check(&self, a_presses: i32, b_presses: i32) {
        assert!(
            a_presses >= 0
                && b_presses >= 0
                && (self.a_button.x * a_presses + self.b_button.x * b_presses == self.prize.x)
                && (self.a_button.y * a_presses + self.b_button.y * b_presses == self.prize.y),
            "{:?}, a_presses = {}, b_presses = {}",
            self,
            a_presses,
            b_presses
        );
    }

    pub fn fewest_tokens_to_win(&self) -> Option<u32> {
        if let Some((a_presses, b_presses)) = self.fewest_tokens_to_win_helper() {
            self.check(a_presses, b_presses);
            Some((3 * a_presses + b_presses) as u32)
        } else {
            None
        }
    }

    fn fewest_tokens_to_win_helper(&self) -> Option<(i32, i32)> {
        // solve where:
        // a.x * a_presses + b.x * b_presses = prize.x
        // a.y * a_presses + b.y * b_presses = prize.y
        let ExtendedGcd {
            gcd: gcd_x,
            x: s_x,
            y: t_x,
            ..
        } = self.a_button.x.extended_gcd(&self.b_button.x);

        if self.prize.x % gcd_x != 0 {
            return None;
        }

        let ExtendedGcd {
            gcd: gcd_y,
            x: s_y,
            y: t_y,
            ..
        } = self.a_button.y.extended_gcd(&self.b_button.y);

        if self.prize.y % gcd_y != 0 {
            return None;
        }

        let p_prime_x = self.prize.x / gcd_x;

        if (self.a_button.x * self.b_button.y) == (self.a_button.y * self.b_button.x) {
            if self.a_button.y * (s_y * (self.prize.y / gcd_y) - s_x * (self.prize.x / gcd_x))
                == self.b_button.y * (t_x * (self.prize.x / gcd_x) - t_y * (self.prize.y / gcd_y))
            {
                // v_0 = t_x * prize_x / gcd_x + a_x / b_x * (s_x * prize_x / gcd_x - u_0)
                let a_presses = (s_x * p_prime_x).rem_euclid(self.b_button.x / gcd_x);
                let b_presses = t_x * p_prime_x
                    + self.a_button.x * ((s_x * p_prime_x - a_presses) / self.b_button.x);

                if b_presses < 0 {
                    return None;
                } else {
                    return Some((a_presses, b_presses));
                }
            } else {
                return None;
            }
        }

        let denominator =
            (self.a_button.x * self.b_button.y - self.a_button.y * self.b_button.x) / gcd_x;

        let p_prime_y = self.prize.y / gcd_y;

        let numerator = self.a_button.y * (s_y * p_prime_y - s_x * p_prime_x)
            + self.b_button.y * (t_y * p_prime_y - t_x * p_prime_x);

        if numerator % denominator != 0 {
            return None;
        }

        let k = numerator / denominator;

        let a_presses = s_x * p_prime_x - (self.b_button.x / gcd_x) * k;
        let b_presses = t_x * p_prime_x + (self.a_button.x / gcd_x) * k;
        return Some((a_presses, b_presses));
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
