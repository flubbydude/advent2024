use once_cell::sync::Lazy;
use regex::Regex;

static PRIZE_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^Prize: X=([1-9][0-9]{1,4}), Y=([1-9][0-9]{1,4})$").unwrap());

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Prize {
    pub x: i64,
    pub y: i64,
}

impl From<&str> for Prize {
    fn from(value: &str) -> Self {
        let (_, [x_str, y_str]) = PRIZE_REGEX.captures(value).unwrap().extract();

        Prize {
            x: x_str.parse().unwrap(),
            y: y_str.parse().unwrap(),
        }
    }
}
