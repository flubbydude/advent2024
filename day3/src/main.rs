use const_format::concatcp;
use once_cell::sync::Lazy;
use regex::Regex;

enum Instruction {
    Mul(u64, u64),
    Do,
    Dont,
}

const MUL_REGEX_STR: &str = r"mul\(([0-9]{1,3}),([0-9]{1,3})\)";

impl<'a> TryFrom<&'a str> for Instruction {
    type Error = &'a str;

    fn try_from(s: &'a str) -> Result<Instruction, Self::Error> {
        static FULL_MUL_REGEX: Lazy<Regex> =
            Lazy::new(|| Regex::new(concatcp!('^', MUL_REGEX_STR, '$')).unwrap());

        match s {
            "do()" => Ok(Instruction::Do),
            "don't()" => Ok(Instruction::Dont),
            _ => match FULL_MUL_REGEX.captures(s) {
                Some(caps) => Ok(Instruction::Mul(
                    caps[1].parse::<u64>().unwrap(),
                    caps[2].parse::<u64>().unwrap(),
                )),
                None => Err(s),
            },
        }
    }
}

fn part1(input: &str) -> u64 {
    static MUL_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(MUL_REGEX_STR).unwrap());

    MUL_REGEX
        .find_iter(input)
        .map(|m| Instruction::try_from(m.as_str()).unwrap())
        .map(|i| {
            if let Instruction::Mul(x, y) = i {
                x * y
            } else {
                panic!("Mul regex found non-mul instruction")
            }
        })
        .sum()
}

fn part2(input: &str) -> u64 {
    static INSTRUCTION_REGEX: Lazy<Regex> =
        Lazy::new(|| Regex::new(concatcp!(r"do\(\)|don't\(\)|", MUL_REGEX_STR)).unwrap());

    let mut enabled = true;

    INSTRUCTION_REGEX
        .find_iter(input)
        .map(|m| Instruction::try_from(m.as_str()).unwrap())
        .filter_map(|i| {
            match i {
                Instruction::Mul(x, y) => {
                    if enabled {
                        return Some(x * y);
                    }
                }
                Instruction::Do => enabled = true,
                Instruction::Dont => enabled = false,
            }

            None
        })
        .sum()
}

fn main() {
    let file_contents = std::fs::read("input.txt").unwrap();
    let input = std::str::from_utf8(&file_contents).unwrap();

    println!("{}", part1(input));
    println!("{}", part2(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT_PART1: &str =
        "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

    const TEST_INPUT_PART2: &str =
        "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

    #[test]
    fn test_part1() {
        assert_eq!(161, part1(TEST_INPUT_PART1));
    }

    #[test]
    fn test_part2() {
        assert_eq!(48, part2(TEST_INPUT_PART2))
    }
}
