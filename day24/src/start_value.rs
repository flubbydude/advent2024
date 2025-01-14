use anyhow::{anyhow, Context};

use std::str::FromStr;

use crate::{node::Node, node_name::NodeName};

#[derive(Debug)]
pub struct StartValue {
    name: NodeName,
    value: bool,
}

impl Node for StartValue {
    fn name(&self) -> NodeName {
        self.name
    }
}

impl StartValue {
    pub fn value(&self) -> bool {
        self.value
    }
}

impl FromStr for StartValue {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (name_str, value_str) = s
            .split_once(": ")
            .context("Unable to split StartValue string")?;
        let name = name_str
            .as_bytes()
            .try_into()
            .context("Node name is not 3 bytes")?;

        let value = match value_str {
            "0" => false,
            "1" => true,
            _ => return Err(anyhow!("Value is not 0 or 1")),
        };

        Ok(StartValue { name, value })
    }
}
