use once_cell::sync::Lazy;
use regex::Regex;

static BUTTON_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^Button (A|B): X\+([0-9][1-9]), Y\+([0-9][1-9])$").unwrap());

#[derive(Debug, PartialEq, Eq)]
pub struct Button {
    pub x: i32,
    pub y: i32,
}

impl Button {
    fn from_button_str(button_str: &str, expected_label: char) -> Self {
        let (_, [label, x_str, y_str]) = BUTTON_REGEX.captures(button_str).unwrap().extract();

        let mut chrs = label.chars();
        if !(chrs.next() == Some(expected_label) && chrs.next() == None) {
            panic!("Wrong button label '{label}' expected to be '{expected_label}' in line: '{button_str}'");
        }

        Button {
            x: x_str.parse().unwrap(),
            y: y_str.parse().unwrap(),
        }
    }

    pub fn from_a_button_str(button_str: &str) -> Self {
        Button::from_button_str(button_str, 'A')
    }

    pub fn from_b_button_str(button_str: &str) -> Self {
        Button::from_button_str(button_str, 'B')
    }
}
