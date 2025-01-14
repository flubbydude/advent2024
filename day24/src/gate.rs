use std::str::FromStr;

use anyhow::Context;

use crate::{gate_type::GateType, node::Node, node_name::NodeName};

#[derive(Debug)]
pub struct Gate {
    name: NodeName,
    input1: NodeName,
    input2: NodeName,
    gate_type: GateType,
}

impl Node for Gate {
    fn name(&self) -> NodeName {
        self.name
    }
}

impl Gate {
    pub fn inputs(&self) -> [NodeName; 2] {
        [self.input1, self.input2]
    }

    pub fn gate_type(&self) -> &GateType {
        &self.gate_type
    }
}

impl FromStr for Gate {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split_ascii_whitespace();
        let input1_str = split.next().context("Unable to get input1")?;
        let gate_type = split
            .next()
            .context("Unable to get gate_type string")?
            .parse()
            .context("Unable to parse get_type string")?;
        let input2_str = split.next().context("Unable to get input2")?;
        let name_str = split
            .next_back()
            .context("Unable to get output node name")?;

        let name = name_str
            .as_bytes()
            .try_into()
            .context("Output node name is not 3 bytes")?;
        let input1 = input1_str
            .as_bytes()
            .try_into()
            .context("Input1 node name is not 3 bytes")?;
        let input2 = input2_str
            .as_bytes()
            .try_into()
            .context("Input2 node name is not 3 bytes")?;

        Ok(Gate {
            name,
            input1,
            input2,
            gate_type,
        })
    }
}
