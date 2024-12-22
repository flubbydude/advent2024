use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap, HashSet},
    hash::Hash,
};

#[derive(PartialEq, Eq)]
struct PriorityQueueItem<S: Eq> {
    priority: u64,
    cost: u64,
    state: S,
}

impl<S: Eq> PartialOrd for PriorityQueueItem<S> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<S: Eq> Ord for PriorityQueueItem<S> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.priority.cmp(&other.priority).reverse()
    }
}

struct ParentsEntry<S> {
    cost: u64,
    state: S,
}

pub struct AStarOutput<S> {
    pub cost: u64,
    pub _order: Vec<S>,
}

pub fn a_star<S, F, I, G, H>(
    start_state: S,
    successors: F,
    is_goal: G,
    heuristic: H,
) -> Option<AStarOutput<S>>
where
    S: PartialEq + Eq + Hash + Clone,
    F: Fn(&S) -> I,
    I: Iterator<Item = (u64, S)>,
    G: Fn(&S) -> bool,
    H: Fn(&S) -> u64,
{
    let mut frontier = BinaryHeap::from([PriorityQueueItem {
        priority: 0,
        cost: 0,
        state: start_state.clone(),
    }]);
    let mut seen: HashSet<S> = HashSet::new();
    let mut parents: HashMap<S, ParentsEntry<S>> = HashMap::new();

    while let Some(item) = frontier.pop() {
        if seen.contains(&item.state) {
            continue;
        }

        if is_goal(&item.state) {
            let mut result = Vec::new();
            let mut last = &item.state;
            while *last != start_state {
                result.push(last.clone());
                last = &parents[last].state;
            }
            result.reverse();
            return Some(AStarOutput {
                cost: item.cost,
                _order: result,
            });
        }

        seen.insert(item.state.clone());

        for (edge_cost, successor) in successors(&item.state) {
            if seen.contains(&successor) {
                continue;
            }

            let new_cost = item.cost + edge_cost;
            if let Some(entry) = parents.get_mut(&successor) {
                if entry.cost > new_cost {
                    entry.cost = new_cost;
                } else {
                    continue;
                }
            } else {
                parents.insert(
                    successor.clone(),
                    ParentsEntry {
                        cost: new_cost,
                        state: item.state.clone(),
                    },
                );
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
