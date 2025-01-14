mod gate;
mod gate_type;
mod graph;
mod node;
mod node_name;
mod start_value;
mod toposort;

use graph::{evaluate_all_values, parse_graph, Graph};
use node_name::int_to_z_node_name;

fn part1(graph: &Graph) -> u64 {
    let values = evaluate_all_values(graph);

    let mut result = 0;
    for z_num in 0..64 {
        let z_node_name = int_to_z_node_name(z_num);
        if values.get(&z_node_name) == Some(&true) {
            result |= 1 << z_num;
        }
    }
    result
}

const INPUT: &str = include_str!("../input.txt");

// part2:
// 1. Create an addition circuit from 2 45 bit ints
// 2. Create a function to check if 2 circuits are equal
//      (start from z bits and traverse simultaneously) and why not
//   - Must always output at least 2 things you can swap bc
//     the swap cannot be in the same tree or there would be a cycle
// 3. Run the function #2 until equal, fixing the swap each time!?
// 4. Record what changed. Output it at the end
fn main() {
    let graph = parse_graph(INPUT);

    println!("{}", part1(&graph));
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
