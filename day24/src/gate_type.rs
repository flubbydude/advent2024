use anyhow::anyhow;

use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GateType {
    And,
    Or,
    Xor,
}

impl FromStr for GateType {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "AND" => Ok(GateType::And),
            "XOR" => Ok(GateType::Xor),
            "OR" => Ok(GateType::Or),
            _ => Err(anyhow!("Attempted to parse unknown GateType")),
        }
    }
}

impl GateType {
    pub fn evaluate(&self, input1: bool, input2: bool) -> bool {
        match self {
            GateType::And => input1 && input2,
            GateType::Or => input1 || input2,
            GateType::Xor => input1 ^ input2,
        }
    }
}
