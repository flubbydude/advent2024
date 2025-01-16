mod gate;
mod gate_type;
mod graph;
mod node;
mod node_name;
mod start_value;
mod toposort;

use std::iter::once;

use gate::Gate;
use gate_type::GateType;
use graph::{evaluate_all_values, parse_graph, successor_graph, Graph};
use itertools::Itertools;
use node::{Node, NodeEnum};
use node_name::{create_node_name, NodeName};

fn part1(graph: &Graph) -> u64 {
    let values = evaluate_all_values(graph);

    let mut result = 0;
    for z_num in 0..64 {
        let z_node_name = create_node_name(b'z', z_num);
        if values.get(&z_node_name) == Some(&true) {
            result |= 1 << z_num;
        }
    }
    result
}

const INPUT: &str = include_str!("../input.txt");

// TODO: check the yoinked rules and come up with it on my own basically

// https://www.reddit.com/r/adventofcode/comments/1hl698z/comment/m3kkp24/?utm_source=share&utm_medium=web3x&utm_name=web3xcss&utm_term=1&utm_content=share_button
// r = lambda c, y: any(y == x and c in (a, b) for a, x, b, _, _ in lines)
// line is a <operation> b -> c
//
// bad where:
// x == "XOR" and all(d[0] not in 'xyz' for d in (a, b, c)) or
// x == "AND" and not "x00" in (a, b) and r(c, 'XOR') or
// x == "XOR" and not "x00" in (a, b) and r(c, 'OR') or
// x != "XOR" and c[0] == 'z' and c != "z45")
fn part2(graph: &Graph) -> String {
    let successor_graph = successor_graph(graph);

    let is_input_to_gate_with_type = |node_name: NodeName, gate_type: GateType| -> bool {
        successor_graph.get(&node_name).is_some_and(|succs| {
            succs
                .iter()
                .any(|succ_node_name| match &graph[succ_node_name] {
                    NodeEnum::Gate(gate) => gate.gate_type() == gate_type,
                    NodeEnum::StartValue(_) => false,
                })
        })
    };

    let is_bad_gate = |gate: &Gate| -> bool {
        if gate.gate_type() == GateType::Xor {
            once(gate.name())
                .chain(gate.inputs())
                .all(|node_name| !b"xyz".contains(&node_name[0]))
                || (!gate.inputs().contains(b"x00")
                    && is_input_to_gate_with_type(gate.name(), GateType::Or))
        } else if gate.name()[0] == b'z' && gate.name().as_slice() != b"z45".as_slice() {
            true
        } else {
            gate.gate_type() == GateType::And
                && !gate.inputs().contains(b"x00")
                && is_input_to_gate_with_type(gate.name(), GateType::Xor)
        }
    };

    let bad_nodes = graph
        .values()
        .filter_map(NodeEnum::gate)
        .filter(|&gate| is_bad_gate(gate))
        .map(Gate::name);

    bad_nodes
        .map(|node_name| String::from_utf8(node_name.to_vec()).unwrap())
        .sorted_unstable()
        .join(",")
}

fn main() {
    let graph = parse_graph(INPUT);

    println!("{}", part1(&graph));
    println!("{}", part2(&graph));
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("../example.txt");

    #[test]
    fn test_part1() {
        let graph = parse_graph(TEST_INPUT);
        assert_eq!(part1(&graph), 2024);
    }
}
