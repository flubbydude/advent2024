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
    fn check(&self, a_presses: i32, b_presses: i32) -> bool {
        a_presses >= 0
            && b_presses >= 0
            && (self.a_button.x * a_presses + self.b_button.x * b_presses == self.prize.x)
            && (self.a_button.y * a_presses + self.b_button.y * b_presses == self.prize.y)
    }

    pub fn fewest_tokens_to_win(&self) -> Option<usize> {
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

        // now we have:
        // a.x * s.x + b.x * t.x = gcd.x

        // multiply by prize value / gcd value
        // a * (s * (prize / gcd)) + b * (t * (prize / gcd)) = prize

        // however, one of s.x and t.x may be negative lol
        // can find other bezout coeffs by this formula:
        // s' = s + k * (b / gcd), t' = t - k * (a / gcd)

        // s + ((-s * gcd) / b) * (b / gcd)
        // s' ~= 0 when k ~= -s * gcd / b
        // t' ~= 0 when k ~=  t * gcd / a

        // so -s * gcd / b <= k <=

        // example:
        // a = {x: 94, y: 34}
        // b = {x: 22, y: 67}
        // prize = {x: 8400, y: 5400}

        // gcd.x = 2, (a.x / gcd.x) = 47, (b.x / gcd.x) = 11, (prize.x / gcd.x) = 4200, s.x = 4, t.x = -17
        // gcd.y = 1, (a.y / gcd.y) = 34, (b.y / gcd.y) = 67, (prize.y / gcd.y) = 5400, s.y = 2, t.y = -1
        // 94 * (4 * 4200 - k_x * 11) + 22 * (-17 * 4200 + k_x * 47) = 8400
        // 34 * (2 * 5400 - k_y * 67) + 67 * (-1 * 5400  + k_y * 34) = 5400

        // 4 * 4200   - k_x * 11 =  2 * 5400 - k_y * 67
        // -17 * 4200 + k_x * 47 = -1 * 5400 + k_y * 34

        // s.x * (prize.x / gcd.x) - k.x * (b.x / gcd.x) = s.y * (prize.y / gcd.y) - k.y * (b.y / gcd.y)
        // t.x * (prize.x / gcd.x) + k.x * (a.x / gcd.x) = t.y * (prize.y / gcd.y) + k.y * (a.y / gcd.y)

        // sx * px - kx * bx = sy * py - ky * by
        // tx * px + kx * ax = ty * py + ky * ay

        // multiply top by ay and bottom by by then add together.
        // sx * px * ay - kx * bx * ay = sy * py * ay - ky * by * ay
        // tx * px * by + kx * ax * by = ty * py * by + ky * ay * by
        // kx = by * (ty * py - tx * px) / (ax * by - bx * ay)

        // in above case, by = 67, ty = -1, py = 5400, tx = -17, px = 4200, ax = 47, bx = 11, ay = 34
        // k_x = 67 * (-1 * 5400 + 17 * 4200) / (47 * 67 - 11 * 34) = 1593.51351351 (wrong lol whoops)

        // a_presses = sx * px - kx * bx =

        // multiply top equation by a.y / gcd.y and bottom equation by b.y / gcd.y and add together
        // = (s.y + t.y * ())

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
