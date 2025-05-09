use std::{
    cell::RefCell,
    collections::{HashMap, HashSet},
    fmt::Display,
    hash::Hash,
    rc::{Rc, Weak},
};

#[derive(Debug, Clone)]
pub(super) struct GraphNode<K: Hash + Eq> {
    pub id: K,
    pub linked: Vec<Weak<RefCell<GraphNode<K>>>>,
}

impl<K: Hash + Eq> PartialEq for GraphNode<K> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl<K: Hash + Eq> Eq for GraphNode<K> {}

impl<K: Hash + Eq> Hash for GraphNode<K> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state)
    }
}

impl<K: Hash + Eq> GraphNode<K> {
    fn new(id: K) -> Self {
        GraphNode {
            id,
            linked: Vec::new(),
        }
    }

    pub(super) fn is_connected(&self, other: &GraphNode<K>) -> bool {
        self.linked
            .iter()
            .any(|el| el.upgrade().unwrap().borrow().id == other.id)
    }

    pub(super) fn upgrade_expect(rc: &Weak<RefCell<Self>>) -> Rc<RefCell<Self>> {
        rc.upgrade().expect("shouldn't have been dropped")
    }
}

#[derive(Debug)]
pub struct Graph<K: Hash + Eq> {
    pub(super) nodes: HashMap<K, Rc<RefCell<GraphNode<K>>>>,
}

impl<K: Hash + Eq + Copy + Ord> Graph<K> {
    pub(super) fn new() -> Self {
        Graph {
            nodes: HashMap::new(),
        }
    }

    /// adds nodes and edge between them if not yet present
    pub(super) fn add_edge(mut self, from: K, to: K) -> Self {
        let from = self
            .nodes
            .entry(from)
            .or_insert_with(|| Rc::new(RefCell::new(GraphNode::new(from))))
            .clone();

        let to = self
            .nodes
            .entry(to)
            .or_insert_with(|| Rc::new(RefCell::new(GraphNode::new(to))))
            .clone();

        from.borrow_mut().linked.push(Rc::downgrade(&to));
        to.borrow_mut().linked.push(Rc::downgrade(&from));

        self
    }
}

pub(super) type Computer = GraphNode<(char, char)>;

impl Display for Computer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.id.0, self.id.1)
    }
}

pub fn to_char_tupel(value: &str) -> (char, char) {
    assert_eq!(2, value.len());
    let mut chars = value.chars();
    (chars.next().unwrap(), chars.next().unwrap())
}

pub fn solve_day_23_part_01(input: &str) -> u32 {
    let graph = input
        .trim()
        .lines()
        .map(|line| line.split_once('-').expect("input must have one '-'"))
        .fold(Graph::new(), |graph, (left, right)| {
            graph.add_edge(to_char_tupel(left), to_char_tupel(right))
        });

    graph
        .nodes
        .iter()
        .filter(|&(&(c, _), _)| c == 't')
        .flat_map(|(&key, root)| {
            let root_id = root.borrow().id;
            root.borrow()
                .linked
                .iter()
                .flat_map(|child| {
                    let child_id = GraphNode::upgrade_expect(child).borrow().id;
                    GraphNode::upgrade_expect(child)
                        .borrow()
                        .linked
                        .iter()
                        .flat_map(|grand_child| {
                            let grand_child_id = GraphNode::upgrade_expect(grand_child).borrow().id;
                            if GraphNode::upgrade_expect(grand_child)
                                .borrow()
                                .linked
                                .iter()
                                .any(|root_again| {
                                    GraphNode::upgrade_expect(root_again).borrow().id == key
                                })
                            {
                                let mut triangle = [root_id, child_id, grand_child_id];
                                triangle.sort();
                                Some(triangle)
                            } else {
                                None
                            }
                        })
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>()
        })
        .collect::<HashSet<[(char, char); 3]>>()
        .len() as u32
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::file::read_string;

    #[test]
    fn should_solve_day_23_part_01() {
        let input = read_string("./src/day23/input.txt").unwrap();

        let solution = solve_day_23_part_01(&input);

        assert_eq!(1423, solution);
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

        assert_eq!(7, solve_day_23_part_01(input));
    }
}
