use std::str::FromStr;

use anyhow::anyhow;
use enum_dispatch::enum_dispatch;

use crate::{gate::Gate, node_name::NodeName, start_value::StartValue};

#[enum_dispatch]
pub trait Node {
    fn name(&self) -> NodeName;
}

#[enum_dispatch(Node)]
#[derive(Debug)]

pub enum NodeEnum {
    StartValue,
    Gate,
}

impl NodeEnum {
    pub fn gate(&self) -> Option<&Gate> {
        match self {
            NodeEnum::Gate(gate) => Some(gate),
            _ => None,
        }
    }
}

impl FromStr for NodeEnum {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.parse::<StartValue>()
            .map(NodeEnum::StartValue)
            .or(s.parse::<Gate>().map(NodeEnum::Gate))
            .map_err(|_| anyhow!("Unable to parse node as StartValue nor as Gate"))
    }
}
