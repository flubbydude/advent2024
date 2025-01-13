use graph::{get_maximum_clique, get_triangles, parse_input, Graph};
use itertools::Itertools;

mod graph;

fn part1(graph: &Graph) -> usize {
    get_triangles(graph)
        .into_iter()
        .filter(|tri| tri.iter().any(|id| id[0] == b't'))
        .count()
}

fn part2(graph: &Graph) -> String {
    let mut lan_party = get_maximum_clique(graph);

    lan_party.sort_unstable();

    lan_party
        .into_iter()
        .map(|comp_id| String::from_utf8(comp_id.to_vec()).unwrap())
        .join(",")
}

const INPUT: &str = include_str!("../input.txt");

fn main() {
    let graph = parse_input(INPUT);

    println!("{}", part1(&graph));
    println!("{}", part2(&graph));
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("../example.txt");

    #[test]
    fn test_part1() {
        let graph = parse_input(TEST_INPUT);
        assert_eq!(part1(&graph), 7);
    }

    #[test]
    fn test_part2() {
        let graph = parse_input(TEST_INPUT);
        assert_eq!(part2(&graph), "co,de,ka,ta");
    }
}
