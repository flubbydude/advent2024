use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap, HashSet},
    fmt,
    hash::Hash,
};

#[derive(PartialEq, Eq)]
struct PriorityQueueItem<S: Eq> {
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
        self.cost.cmp(&other.cost).reverse()
    }
}

pub fn best_cost_djikstra<S, F, I, G>(
    start_states: impl IntoIterator<Item = (u64, S)>,
    successors: F,
    is_goal: G,
) -> Option<u64>
where
    S: fmt::Debug + PartialEq + Eq + Hash + Clone,
    F: Fn(&S) -> I,
    I: IntoIterator<Item = (u64, S)>,
    G: Fn(&S) -> bool,
{
    let mut frontier = BinaryHeap::from_iter(
        start_states
            .into_iter()
            .map(|(cost, state)| PriorityQueueItem { cost, state }),
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

        for (edge_cost, successor) in successors(&item.state) {
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
                cost: new_cost,
                state: successor,
            });
        }
    }

    None
}

struct BestPathsHelperEnv<'a, S, F, I, G>
where
    S: fmt::Debug + PartialEq + Eq + Hash + Clone,
    F: Fn(&S) -> I,
    I: IntoIterator<Item = (u64, S)>,
    G: Fn(&S) -> bool,
{
    successors: &'a F,
    is_goal: &'a G,
    best_path_cost: u64,
    open_set: &'a mut Vec<S>,
    min_cost_seen: &'a mut HashMap<S, u64>,
    best_paths: &'a mut Vec<Vec<S>>,
}

fn best_paths_helper<S, F, I, G>(state: S, cost: u64, env: &mut BestPathsHelperEnv<'_, S, F, I, G>)
where
    S: fmt::Debug + PartialEq + Eq + Hash + Clone,
    F: Fn(&S) -> I,
    I: IntoIterator<Item = (u64, S)>,
    G: Fn(&S) -> bool,
{
    if (env.is_goal)(&state) {
        if cost == env.best_path_cost {
            let mut cloned_open_set = env.open_set.clone();
            cloned_open_set.push(state.clone());
            env.best_paths.push(cloned_open_set);
        }
        return;
    }

    match env.min_cost_seen.get_mut(&state) {
        Some(min_cost_seen) => {
            if *min_cost_seen < cost {
                // prune the branch
                return;
            } else {
                *min_cost_seen = cost;
            }
        }
        None => {
            env.min_cost_seen.insert(state.clone(), cost);
        }
    }

    env.open_set.push(state.clone());

    for (edge_cost, next_state) in (env.successors)(&state) {
        let next_cost = cost + edge_cost;
        if next_cost > env.best_path_cost {
            continue;
        }
        best_paths_helper(next_state, next_cost, env);
    }

    assert_eq!(state, env.open_set.pop().unwrap());
}

pub fn best_paths<S, F, I, G>(
    start_states: impl IntoIterator<Item = (u64, S)>,
    successors: F,
    is_goal: G,
    best_path_cost: u64,
) -> Vec<Vec<S>>
where
    S: fmt::Debug + PartialEq + Eq + Hash + Clone,
    F: Fn(&S) -> I,
    I: IntoIterator<Item = (u64, S)>,
    G: Fn(&S) -> bool,
{
    let mut best_paths = Vec::new();
    let mut min_cost_seen = HashMap::new();

    for (cost, start_state) in start_states {
        best_paths_helper(
            start_state,
            cost,
            &mut BestPathsHelperEnv {
                successors: &successors,
                is_goal: &is_goal,
                best_path_cost,
                open_set: &mut Vec::new(),
                min_cost_seen: &mut min_cost_seen,
                best_paths: &mut best_paths,
            },
        );
    }

    best_paths
}
