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

// https://en.wikipedia.org/wiki/Bron%E2%80%93Kerbosch_algorithm
fn bron_kerbosch(
    r: &mut Vec<ComputerId>,
    mut p: HashSet<ComputerId>,
    mut x: HashSet<ComputerId>,
    result: &mut Vec<Vec<ComputerId>>,
    graph: &Graph,
) {
    if p.is_empty() && x.is_empty() {
        result.push(r.to_vec());
        return;
    }

    let pivot = p.union(&x).next().copied().unwrap();
    let vertices_not_neighbors_of_pivot = p.difference(&graph[&pivot]).copied().collect::<Vec<_>>();

    for comp in vertices_not_neighbors_of_pivot {
        r.push(comp);

        let new_p = p.intersection(&graph[&comp]).copied().collect();
        let new_x = x.intersection(&graph[&comp]).copied().collect();
        bron_kerbosch(r, new_p, new_x, result, graph);
        r.pop();

        p.remove(&comp);
        x.insert(comp);
    }
}

pub fn get_maximum_clique(graph: &Graph) -> Vec<ComputerId> {
    let mut result = Vec::new();
    bron_kerbosch(
        &mut Vec::new(),
        graph.keys().copied().collect(),
        HashSet::new(),
        &mut result,
        graph,
    );

    result.into_iter().max_by_key(Vec::len).unwrap()
}
