use std::collections::HashMap;

use crate::{
    node::{Node, NodeEnum},
    node_name::NodeName,
    toposort::toposort,
};

pub type Graph = HashMap<NodeName, NodeEnum>;

pub fn parse_graph(input: &str) -> Graph {
    input
        .lines()
        .filter_map(|line| line.parse::<NodeEnum>().ok())
        .map(|node| (node.name(), node))
        .collect()
}

pub fn evaluate_all_values(graph: &Graph) -> HashMap<NodeName, bool> {
    let mut result = graph
        .iter()
        .filter_map(|(node_name, node)| {
            if let NodeEnum::StartValue(start_value) = node {
                Some((*node_name, start_value.value()))
            } else {
                None
            }
        })
        .collect::<HashMap<_, _>>();

    for node_name in toposort(graph) {
        if let NodeEnum::Gate(gate) = &graph[&node_name] {
            let [input1_name, input2_name] = gate.inputs();
            let value = gate
                .gate_type()
                .evaluate(result[&input1_name], result[&input2_name]);
            result.insert(node_name, value);
        }
    }

    result
}

pub fn successor_graph(graph: &Graph) -> HashMap<NodeName, Vec<NodeName>> {
    let mut result: HashMap<NodeName, Vec<NodeName>> = HashMap::new();
    for node in graph.values() {
        if let NodeEnum::Gate(gate) = node {
            for input_node_name in gate.inputs() {
                result.entry(input_node_name).or_default().push(node.name());
            }
        }
    }
    result
}
