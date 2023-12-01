use std::{collections::HashMap, fmt::Display};

use rayon::iter::{ParallelBridge, ParallelIterator};

pub fn main() {
    let input = include_str!("../input/day18.txt");
    println!("Ex1: {:?}", exercise_1(&input));
    println!("Ex2: {:?}", exercise_2(&input));
}

fn exercise_1(input: &str) -> usize {
    let mut tree = Tree::default();
    let mut lines = input.lines();

    let first = tree.set_root(lines.next().unwrap());
    tree.root = first.id;

    let tree = lines.fold(tree, |mut tree, x| {
        tree.add_line(x);
        tree
    });

    magnitude(&tree, tree.root)
}

fn exercise_2(input: &str) -> usize {
    let input = input.lines().collect::<Vec<_>>();

    let max = (0..input.len())
        .par_bridge()
        .map(|i| {
            let mut max = 0;
            for j in 0..input.len() {
                if i == j {
                    continue;
                }

                let mut tree = Tree::default();
                let node = tree.set_root(&input[i]);
                tree.root = node.id;

                tree.add_line(&input[j]);
                let magnitude = magnitude(&tree, tree.root);
                if magnitude > max {
                    max = magnitude;
                }
            }

            max
        })
        .max()
        .unwrap();
    max
}

fn magnitude(tree: &Tree, node: usize) -> usize {
    match tree.map[&node].value {
        TreeChild::Value(n) => n as usize,
        TreeChild::Node(l, r) => 3 * magnitude(tree, l) + 2 * magnitude(tree, r),
    }
}

#[derive(Debug, Default)]
struct Tree {
    root: usize,
    counter: usize,
    map: HashMap<usize, TreeNode>,
}

impl Tree {
    pub fn set_root(&mut self, line: &str) -> TreeNode {
        let mut chars = line.chars();
        let start = chars.next().unwrap();
        debug_assert!(start == '[');

        parse_tree(self, None, &mut chars)
    }

    pub fn add_line(&mut self, x: &str) {
        let newly = self.set_root(x);

        let old_root = self.root;

        let new_node = TreeNode {
            id: self.counter,
            parent: None,
            value: TreeChild::Node(old_root, newly.id),
        };

        self.counter += 1;
        self.root = new_node.id;

        self.map.get_mut(&newly.id).unwrap().parent = Some(new_node.id);
        self.map.get_mut(&old_root).unwrap().parent = Some(new_node.id);

        self.map.insert(new_node.id, new_node.clone());

        loop {
            self.cleanup_explode(new_node.id, 0);
            if self.cleanup_split(new_node.id, 0) {
                continue;
            } else {
                break;
            }
        }
    }

    pub fn cleanup_explode(&mut self, node: usize, depth: usize) {
        if depth >= 4 && !self.map[&node].value.is_value() {
            self.explode(node);
            return;
        }

        let node_value = self.map.get(&node).unwrap().value.clone();
        let (l, r) = match node_value {
            TreeChild::Value(_) => {
                return;
            }
            TreeChild::Node(l, r) => (l, r),
        };

        self.cleanup_explode(l, depth + 1);
        self.cleanup_explode(r, depth + 1);
    }

    pub fn cleanup_split(&mut self, node: usize, depth: usize) -> bool {
        let node_value = self.map.get(&node).unwrap().value.clone();
        let (l, r) = match node_value {
            TreeChild::Value(n) => {
                if n >= 10 {
                    if self.split(node) && depth != 4 {
                        return self.cleanup_split(node, depth);
                    }
                    return depth == 4;
                }
                return false;
            }
            TreeChild::Node(l, r) => (l, r),
        };

        self.cleanup_split(l, depth + 1) || self.cleanup_split(r, depth + 1)
    }

    pub fn split(&mut self, node: usize) -> bool {
        let current_value = match self.map[&node].value {
            TreeChild::Value(n) => n,
            TreeChild::Node(_, _) => unreachable!(),
        };
        let left_value = current_value / 2;
        let right_value = (current_value + 1) / 2;

        let continue_split = left_value > 9 || right_value > 9;

        let left_value = create_value_node(self, Some(node), left_value);
        let right_value = create_value_node(self, Some(node), right_value);

        self.map.get_mut(&node).unwrap().value = TreeChild::Node(left_value, right_value);

        continue_split
    }

    pub fn explode(&mut self, node: usize) {
        let (left, right) = {
            let node = &self.map[&node];
            let (left, right) = match node.value {
                TreeChild::Node(left, right) => (left, right),
                _ => unreachable!(),
            };
            (left, right)
        };

        {
            let old_child_value = match self.map[&left].value {
                TreeChild::Value(v) => v,
                TreeChild::Node(_, _) => unreachable!(
                    "Should not be able to explode a node with children that are not both a value"
                ),
            };
            if let Some(node) = self.left_value_node(node) {
                self.map.entry(node).and_modify(|x| match &mut x.value {
                    TreeChild::Value(a) => *a += old_child_value,
                    TreeChild::Node(_, _) => unreachable!(),
                });
            }
            self.map.remove(&left);
        }

        {
            let old_child_value = match self.map[&right].value {
                TreeChild::Value(v) => v,
                TreeChild::Node(_, _) => unreachable!(
                    "Should not be able to explode a node with children that are not both a value"
                ),
            };

            if let Some(node) = self.right_value_node(node) {
                self.map.entry(node).and_modify(|x| match &mut x.value {
                    TreeChild::Value(a) => *a += old_child_value,
                    TreeChild::Node(_, _) => unreachable!(),
                });
            }
            self.map.remove(&right);
        }

        self.map.get_mut(&node).unwrap().value = TreeChild::Value(0);
    }

    pub fn left_value_node(&self, mut node: usize) -> Option<usize> {
        // up part, go up

        let mut node = loop {
            let parent = self.map[&node].parent?;
            let parent = self.map[&parent].clone();
            match parent.value {
                TreeChild::Node(l, r) => {
                    if node == r {
                        break l;
                    }
                }
                TreeChild::Value(_) => unreachable!(),
            }
            node = parent.id;
        };

        // go down
        loop {
            node = match self.map[&node].value {
                TreeChild::Value(_) => return Some(node),
                TreeChild::Node(_, r) => r,
            }
        }
    }

    pub fn right_value_node(&self, mut node: usize) -> Option<usize> {
        // up part, go up

        let mut node = loop {
            let parent = self.map[&node].parent?;
            let parent = self.map[&parent].clone();
            match parent.value {
                TreeChild::Node(l, r) => {
                    if node == l {
                        break r;
                    }
                }
                TreeChild::Value(_) => unreachable!(),
            }
            node = parent.id;
        };

        // go down
        loop {
            node = match self.map[&node].value {
                TreeChild::Value(_) => return Some(node),
                TreeChild::Node(l, _) => l,
            }
        }
    }
}

fn create_value_node(tree: &mut Tree, parent: Option<usize>, value: u8) -> usize {
    let node_id = tree.counter;
    tree.counter += 1;
    let node = TreeNode {
        id: node_id,
        parent: parent,
        value: TreeChild::Value(value),
    };
    tree.map.insert(node_id, node);

    node_id
}
fn parse_tree(
    tree: &mut Tree,
    parent: Option<usize>,
    chars: &mut dyn Iterator<Item = char>,
) -> TreeNode {
    let tree_node_id = tree.counter;

    tree.counter += 1;

    let left = match chars.next().unwrap() {
        '[' => {
            let node = parse_tree(tree, Some(tree_node_id), chars);
            node.id
        }
        n => create_value_node(tree, Some(tree_node_id), n as u8 - '0' as u8),
    };

    let komma = chars.next().unwrap();
    debug_assert!(komma == ',', "but was actually: {}", komma);

    let right = match chars.next().unwrap() {
        '[' => {
            let node = parse_tree(tree, Some(tree_node_id), chars);
            node.id
        }
        n => create_value_node(tree, Some(tree_node_id), n as u8 - '0' as u8),
    };

    let o = chars.next().unwrap();
    debug_assert!(o == ']', "but was actually: {}", o);

    let tree_node = TreeNode {
        id: tree_node_id,
        parent: parent,
        value: TreeChild::Node(left, right),
    };

    tree.map.insert(tree_node.id, tree_node.clone());

    tree_node
}
#[derive(Debug, Clone)]
struct TreeNode {
    id: usize,
    parent: Option<usize>,
    value: TreeChild,
}

#[derive(Debug, Clone)]
enum TreeChild {
    Value(u8),
    Node(usize, usize),
}

impl TreeChild {
    pub fn is_value(&self) -> bool {
        match self {
            TreeChild::Value(_) => true,
            TreeChild::Node(_, _) => false,
        }
    }
}

impl Display for Tree {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&to_format(self, self.root))
    }
}

fn to_format(tree: &Tree, node: usize) -> String {
    match tree.map[&node].value {
        TreeChild::Value(x) => x.to_string(),
        TreeChild::Node(l, r) => format!("[{},{}]", to_format(tree, l), to_format(tree, r)),
    }
}
