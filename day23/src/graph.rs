use std::collections::{HashMap, HashSet};

type ComputerId = [u8; 2];
pub type Graph = HashMap<ComputerId, HashSet<ComputerId>>;

pub fn parse_input(input: &str) -> Graph {
    let mut result: Graph = HashMap::new();

    for line in input.lines() {
        let (comp1_str, comp2_str) = line.split_once('-').unwrap();
        let comp1_id = comp1_str.as_bytes().try_into().unwrap();
        let comp2_id = comp2_str.as_bytes().try_into().unwrap();

        result.entry(comp1_id).or_default().insert(comp2_id);
        result.entry(comp2_id).or_default().insert(comp1_id);
    }

    result
}

pub fn get_triangles(graph: &Graph) -> impl IntoIterator<Item = [ComputerId; 3]> {
    let mut edges_seen: HashSet<[ComputerId; 2]> = HashSet::new();
    let mut result: HashSet<[ComputerId; 3]> = HashSet::new();

    for (&comp1, comp1_succs) in graph {
        for &comp2 in comp1_succs {
            let mut edge = [comp1, comp2];
            edge.sort_unstable();
            let edge = edge;
            if edges_seen.contains(&edge) {
                continue;
            }

            edges_seen.insert(edge);
            let comp2_succs = &graph[&comp2];
            result.extend(comp1_succs.intersection(comp2_succs).map(|&comp3| {
                let mut triangle = [comp1, comp2, comp3];
                triangle.sort_unstable();
                triangle
            }));
        }
    }

    result
}

fn get_connected_components(graph: &Graph) -> Vec<Vec<ComputerId>> {
    fn dfs(
        comp: ComputerId,
        graph: &Graph,
        explored: &mut HashMap<ComputerId, usize>,
        next_label: usize,
    ) -> usize {
        if let Some(&label) = explored.get(&comp) {
            return label;
        }

        explored.insert(comp, next_label);

        for &succ in graph[&comp].iter() {
            dfs(succ, graph, explored, next_label);
        }

        next_label
    }

    let mut explored: HashMap<ComputerId, usize> = HashMap::new();
    let mut result = Vec::new();

    for &comp in graph.keys() {
        let label = dfs(comp, graph, &mut explored, result.len());
        if label == result.len() {
            result.push(vec![comp]);
        } else {
            result[label].push(comp);
        }
    }

    result
}

fn get_subgraph<'a>(graph: &Graph, vertices: impl IntoIterator<Item = &'a ComputerId>) -> Graph {
    let mut result = Graph::new();
    for &vertex in vertices {
        result.insert(vertex, graph[&vertex].clone());
    }
    result
}

fn get_inverse_graph(graph: &Graph) -> Graph {
    graph
        .iter()
        .map(|(&comp_id, succs)| {
            (
                comp_id,
                graph
                    .keys()
                    .filter(|&&other_id| other_id != comp_id && !succs.contains(&other_id))
                    .copied()
                    .collect(),
            )
        })
        .collect()
}

fn get_maximum_clique_cc(connected_component: &Graph) -> Vec<ComputerId> {
    todo!()
}

pub fn get_maximum_clique(graph: &Graph) -> Vec<ComputerId> {
    let ccs = get_connected_components(graph);
    match ccs.len() {
        0 => Vec::new(),
        1 => get_maximum_clique_cc(graph),
        _ => ccs
            .into_iter()
            .map(|cc| get_subgraph(graph, &cc))
            .map(|cc_subgraph| get_maximum_clique_cc(&cc_subgraph))
            .max_by_key(Vec::len)
            .unwrap(),
    }
}
