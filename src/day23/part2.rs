use super::part1::Computer;
use crate::day23::part1::{to_char_tupel, Graph};
use std::{
    cmp::Reverse,
    collections::{HashMap, HashSet},
};

fn expand(
    current_clique: &mut Vec<Computer>,
    candidates: &HashSet<Computer>,
    best: &mut Vec<Computer>,
) {
    if candidates.is_empty() {
        if current_clique.len() > best.len() {
            *best = (*current_clique).clone();
        }
        return;
    }

    if best.len() >= current_clique.len() + candidates.len() {
        return;
    }

    for node in candidates {
        if current_clique.iter().all(|n| n.is_connected(&node)) {
            let next_candidates: HashSet<Computer> = node
                .linked
                .iter()
                .filter_map(|e| {
                    let node = e.upgrade().expect("shouldn't have been dropped");

                    if candidates.contains(&node.borrow()) {
                        Some(node.borrow().clone())
                    } else {
                        None
                    }
                })
                .collect();

            current_clique.push(node.clone());
            expand(current_clique, &next_candidates, best);
            current_clique.pop();
        }
    }
}

pub fn solve_day_23_part_02(input: &str) -> String {
    let graph = input
        .trim()
        .lines()
        .map(|line| line.split_once('-').expect("input must have one '-'"))
        .fold(Graph::new(), |graph, (left, right)| {
            graph.add_edge(to_char_tupel(left), to_char_tupel(right))
        });

    let mut best_clique: Vec<Computer> = vec![];
    let mut nodes: Vec<_> = graph.nodes.values().collect();
    nodes.sort_by_key(|rc| rc.borrow().linked.len());
    let index_of: HashMap<Computer, usize> = nodes
        .iter()
        .enumerate()
        .map(|(i, rc)| (rc.borrow().clone(), i))
        .collect();

    for node in nodes {
        let node = node.borrow();
        let start_i = index_of[&node];
        let candidates: HashSet<Computer> = node
            .linked
            .iter()
            .filter_map(|e| {
                let neigh = e.upgrade().unwrap().borrow().clone();
                if index_of[&neigh] > start_i {
                    Some(neigh)
                } else {
                    None
                }
            })
            .collect();

        expand(&mut vec![node.clone()], &candidates, &mut best_clique);
    }

    let mut best_clique: Vec<String> = best_clique
        .iter()
        .map(|a| format!("{}{}", a.id.0, a.id.1))
        .collect();
    best_clique.sort();
    best_clique.join(",")
}

/// another part where it's not about the best solution to the problem but about some hacky only works for this
/// specific input kinda situation ... why WHY WHYY??
/// inspired by https://blog.jverkamp.com/2024/12/23/aoc-2024-day-23-lan-partinator/
pub fn solve_day_23_part_02_with_hacky_shit_solution(input: &str) -> String {
    let graph = input
        .trim()
        .lines()
        .map(|line| line.split_once('-').unwrap())
        .fold(Graph::new(), |g, (l, r)| {
            g.add_edge(to_char_tupel(l), to_char_tupel(r))
        });

    let mut nodes: Vec<Computer> = graph.nodes.values().map(|rc| rc.borrow().clone()).collect();
    nodes.sort_by_key(|c| Reverse(c.linked.len()));

    for node in nodes {
        let neighbors: Vec<Computer> = node
            .linked
            .iter()
            .filter_map(|w| w.upgrade())
            .map(|rc| rc.borrow().clone())
            .collect();

        for i in 0..neighbors.len() {
            let mut clique = neighbors.clone();
            clique.remove(i);

            let complete = clique
                .iter()
                .all(|a| clique.iter().all(|b| a == b || a.is_connected(b)));
            if complete {
                clique.push(node.clone());
                let mut ids: Vec<String> = clique
                    .into_iter()
                    .map(|c| format!("{}{}", c.id.0, c.id.1))
                    .collect();
                ids.sort();
                return ids.join(",");
            }
        }
    }

    panic!("No maximal clique found");
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::file::read_string;

    #[test]
    fn should_solve_day_23_part_02() {
        let input = read_string("./src/day23/input.txt").unwrap();

        let solution = solve_day_23_part_02_with_hacky_shit_solution(&input);

        assert_eq!(solution, "gt,ha,ir,jn,jq,kb,lr,lt,nl,oj,pp,qh,vy");
    }

    #[test]
    fn example() {
        let input = "kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn"
            .trim();

        assert_eq!("co,de,ka,ta", solve_day_23_part_02(input));
    }
}
