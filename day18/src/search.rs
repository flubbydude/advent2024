use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap, HashSet},
    hash::Hash,
};

struct PriorityQueueItem<S> {
    priority: u64,
    cost: u64,
    state: S,
}

impl<S> PartialOrd for PriorityQueueItem<S> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<S> Ord for PriorityQueueItem<S> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.priority.cmp(&other.priority).reverse()
    }
}

impl<S> PartialEq for PriorityQueueItem<S> {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl<S> Eq for PriorityQueueItem<S> {}

pub fn best_cost_a_star<S, F, I, G, H>(
    start_states: impl IntoIterator<Item = (u64, S)>,
    successors: F,
    is_goal: G,
    heuristic: H,
) -> Option<u64>
where
    S: PartialEq + Eq + Hash + Clone,
    F: Fn(&S) -> I,
    I: IntoIterator<Item = (u64, S)>,
    G: Fn(&S) -> bool,
    H: Fn(&S) -> u64,
{
    let mut frontier =
        BinaryHeap::from_iter(
            start_states
                .into_iter()
                .map(|(cost, state)| PriorityQueueItem {
                    priority: cost + heuristic(&state),
                    cost,
                    state,
                }),
        );
    let mut seen: HashSet<S> = HashSet::new();
    let mut costs: HashMap<S, u64> = HashMap::new();

    while let Some(item) = frontier.pop() {
        if seen.contains(&item.state) {
            continue;
        }

        if is_goal(&item.state) {
            return Some(item.cost);
        }

        seen.insert(item.state.clone());

        let succs_iter = successors(&item.state);

        for (edge_cost, successor) in succs_iter {
            if seen.contains(&successor) {
                continue;
            }

            let new_cost = item.cost + edge_cost;
            if let Some(entry) = costs.get_mut(&successor) {
                if *entry > new_cost {
                    *entry = new_cost;
                } else {
                    continue;
                }
            } else {
                costs.insert(successor.clone(), new_cost);
            }

            frontier.push(PriorityQueueItem {
                priority: new_cost + heuristic(&successor),
                cost: new_cost,
                state: successor,
            });
        }
    }

    None
}
