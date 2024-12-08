use std::{
    collections::{HashMap, HashSet},
    ops::Deref,
};

trait Update {
    fn follows_rules(&self, rules: &HashMap<u8, Vec<u8>>) -> bool;
}

impl<T: Deref<Target = [u8]>> Update for T {
    fn follows_rules(&self, rules: &HashMap<u8, Vec<u8>>) -> bool {
        let mut bad_set: HashSet<u8> = HashSet::new();

        for cur_page in self.iter().rev() {
            if bad_set.contains(cur_page) {
                return false;
            }

            bad_set.extend(rules.get(cur_page).into_iter().flatten());
        }

        true
    }
}

fn create_rules_subgraph(pages: &[u8], rules: &HashMap<u8, Vec<u8>>) -> HashMap<u8, Vec<u8>> {
    let pages_set: HashSet<u8> = pages.iter().copied().collect();
    pages
        .iter()
        .copied()
        .map(|page| {
            (
                page,
                rules
                    .get(&page)
                    .map(|succs| {
                        succs
                            .iter()
                            .copied()
                            .filter(|succ| pages_set.contains(succ))
                            .collect()
                    })
                    .unwrap_or_default(),
            )
        })
        .collect()
}

// assume all successors are also keys
fn toposort_pages_by_rules(rules: &HashMap<u8, Vec<u8>>) -> Vec<u8> {
    enum Tag {
        Open,
        Explored,
    }

    enum StackItem {
        Todo(u8),
        Cleanup(u8),
    }

    let mut seen: HashMap<u8, Tag> = HashMap::new();
    let mut result = Vec::new();

    for &start_page in rules.keys() {
        let mut stack = Vec::from([StackItem::Todo(start_page)]);

        while let Some(item) = stack.pop() {
            match item {
                StackItem::Todo(page) => {
                    match seen.get(&page) {
                        Some(Tag::Explored) => continue,
                        Some(Tag::Open) => panic!("Graph has a cycle"),
                        None => (),
                    }

                    seen.insert(page, Tag::Open);

                    stack.push(StackItem::Cleanup(page));

                    if let Some(succs) = rules.get(&page) {
                        for &succ in succs {
                            stack.push(StackItem::Todo(succ));
                        }
                    }
                }
                StackItem::Cleanup(page) => {
                    *seen.get_mut(&page).unwrap() = Tag::Explored;
                    result.push(page);
                }
            }
        }
    }

    result.reverse();
    result
}

struct PuzzleInput {
    rules: HashMap<u8, Vec<u8>>,
    updates: Vec<Vec<u8>>,
}

fn parse_input(input: &str) -> PuzzleInput {
    let mut lines = input.lines();

    let mut rules: HashMap<u8, Vec<u8>> = HashMap::new();

    for line in &mut lines {
        if line.is_empty() {
            break;
        }

        let (page1, page2) = line.split_once('|').unwrap();
        let page1 = page1.parse().unwrap();
        let page2 = page2.parse().unwrap();

        rules.entry(page1).or_default().push(page2);
    }

    let updates = lines
        .map(|line| line.split(',').map(|s| s.parse::<u8>().unwrap()).collect())
        .collect();

    PuzzleInput { rules, updates }
}

fn part1(input: &PuzzleInput) -> u64 {
    input
        .updates
        .iter()
        .filter_map(|update| {
            if update.follows_rules(&input.rules) {
                Some(update[update.len() / 2] as u64)
            } else {
                None
            }
        })
        .sum()
}

// Assumption: only 1 solution for middle page after toposort
fn part2(PuzzleInput { rules, updates }: &PuzzleInput) -> u64 {
    updates
        .iter()
        .filter(|&update| !update.follows_rules(&rules))
        .map(|update| {
            // copy over the rules for relevant pages, insert empty list if no rules
            let rules_subgraph = create_rules_subgraph(update, rules);
            let pages_ordering = toposort_pages_by_rules(&rules_subgraph);
            pages_ordering[pages_ordering.len() / 2] as u64
        })
        .sum()
}

fn main() {
    let file_contents = std::fs::read("input.txt").unwrap();
    let file_contents_as_str = std::str::from_utf8(&file_contents).unwrap();

    let input = parse_input(file_contents_as_str);

    println!("{}", part1(&input));
    println!("{}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "47|53\n\
                              97|13\n\
                              97|61\n\
                              97|47\n\
                              75|29\n\
                              61|13\n\
                              75|53\n\
                              29|13\n\
                              97|29\n\
                              53|29\n\
                              61|53\n\
                              97|53\n\
                              61|29\n\
                              47|13\n\
                              75|47\n\
                              97|75\n\
                              47|61\n\
                              75|61\n\
                              47|29\n\
                              75|13\n\
                              53|13\n\n\
                              75,47,61,53,29\n\
                              97,61,53,29,13\n\
                              75,29,13\n\
                              75,97,47,61,53\n\
                              61,13,29\n\
                              97,13,75,29,47";

    #[test]
    fn test_part1() {
        let input = parse_input(TEST_INPUT);
        assert_eq!(143, part1(&input))
    }

    #[test]
    fn test_part2() {
        let input = parse_input(TEST_INPUT);
        assert_eq!(123, part2(&input))
    }
}
