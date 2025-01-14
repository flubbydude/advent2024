use std::collections::{hash_map::Entry, HashMap};

use crate::{graph::Graph, node::NodeEnum, node_name::NodeName};

pub fn toposort(graph: &Graph) -> Vec<NodeName> {
    enum Mark {
        Temporary,
        Permanent,
    }

    fn dfs(
        node_name: NodeName,
        graph: &Graph,
        marks: &mut HashMap<NodeName, Mark>,
        result: &mut Vec<NodeName>,
    ) {
        match marks.entry(node_name) {
            Entry::Occupied(entry) => match entry.get() {
                Mark::Permanent => return,
                Mark::Temporary => panic!("Graph has a cycle"),
            },
            Entry::Vacant(entry) => {
                entry.insert(Mark::Temporary);
            }
        }

        if let NodeEnum::Gate(gate) = &graph[&node_name] {
            for input_node_name in gate.inputs() {
                dfs(input_node_name, graph, marks, result);
            }
        }

        *marks.get_mut(&node_name).unwrap() = Mark::Permanent;
        result.push(node_name);
    }

    let marks = &mut HashMap::new();
    let mut result = Vec::new();

    for &node_name in graph.keys() {
        dfs(node_name, graph, marks, &mut result);
    }

    result
}
