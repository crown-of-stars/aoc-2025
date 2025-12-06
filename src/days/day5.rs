use std::path::Path;

use crate::{DayPart, Test};

pub struct PartOne;
pub struct PartTwo;

fn parse(data: &str) -> (Vec<(u64, u64)>, Vec<u64>) {
    let mut ranges = vec![];
    let mut ingredients = vec![];

    for line in data.lines() {
        let mut line_parts = line.split('-');
        if let Some(a) = line_parts.next() {
            if a.len() == 0 { continue; }
            if let Some(b) = line_parts.next() {
                ranges.push((a.parse().expect("Should be number"), b.parse().expect("Should be a number")));
            } else {
                ingredients.push(a.parse().expect("Should be number"));
            }
        }
    }

    (ranges, ingredients)
}

impl DayPart for PartOne {
    fn solve(&self, data: &str) -> u64 {
        let (ranges, ingredients) = parse(data);
        
        let tree = ranges.into_iter().fold(BinTree::BinLeaf(BinLeaf { fresh: false }), |tree, (start, end)| {
            insert_range(tree, start, end, 0, u64::MAX)
        });
        // print_tree(&tree, 0);

        ingredients.into_iter().filter(|n| in_tree(&tree, *n)).count() as u64
    }
    
    fn tests(&self) -> Vec<Test> {
        vec![Test {
            id: 1,
            path: Path::new("./data/5t1.txt"),
            expected: 3
        }]
    }
    
    fn data_path(&self) -> Box<Path> {
        Path::new("./data/5.txt").into()
    }
}

impl DayPart for PartTwo {
    fn solve(&self, data: &str) -> u64 {
        let (ranges, _) = parse(data);
        
        let tree = ranges.into_iter().fold(BinTree::BinLeaf(BinLeaf { fresh: false }), |tree, (start, end)| {
            insert_range(tree, start, end, 0, u64::MAX)
        });

        leaf_count(&tree, 0, u64::MAX)
    }
    
    fn tests(&self) -> Vec<Test> {
        vec![Test {
            id: 1,
            path: Path::new("./data/5t1.txt"),
            expected: 14
        }]
    }
    
    fn data_path(&self) -> Box<Path> {
        Path::new("./data/5.txt").into()
    }
}

enum BinTree {
    BinNode(BinNode),
    BinLeaf(BinLeaf)
}

struct BinNode {
    value: u64,
    left: Box<BinTree>, // x < v
    right: Box<BinTree> // x >= v
}

struct BinLeaf {
    fresh: bool,
}

#[allow(dead_code)]
fn print_tree(tree: &BinTree, indent: usize) {
    for _ in 0..indent { print!("  "); }
    match tree {
        BinTree::BinLeaf(leaf) => {
            println!("- Leaf, fresh: {}", leaf.fresh);
        },
        BinTree::BinNode(node) => {
            println!("- Node, split: {}", node.value);
            print_tree(&node.left, indent + 1);
            print_tree(&node.right, indent + 1);
        }
    }
}

fn in_tree(tree: &BinTree, n: u64) -> bool {
    match tree {
        BinTree::BinLeaf(leaf) => leaf.fresh,
        BinTree::BinNode(node) => {
            if n < node.value { in_tree(&node.left, n) }
            else { in_tree(&node.right, n) }
        }
    }
}

fn leaf_count(tree: &BinTree, tree_start: u64, tree_end: u64) -> u64 {
    match tree {
        BinTree::BinLeaf(leaf) => if leaf.fresh { tree_end - tree_start + 1 } else { 0 },
        BinTree::BinNode(node) => {
            leaf_count(&node.left, tree_start, node.value - 1) + leaf_count(&node.right, node.value, tree_end)
        }
    }
}

fn insert_range(tree: BinTree, start: u64, end: u64, node_start: u64, node_end: u64) -> BinTree {
    // println!("Inserting. {} {} {} {}", start, end, node_start, node_end);
    if node_start >= start && node_end <= end {
        // println!(": Filled.");
        return BinTree::BinLeaf(BinLeaf { fresh: true })
    } // if node is completely within range, mark it as such
    // code reachable => overlap
    // print_tree(&tree, 0);
    match tree {
        BinTree::BinLeaf(leaf) => {
            if node_end < start || node_start > end { BinTree::BinLeaf(leaf) } // if node does not touch range, keep it
            else if leaf.fresh { BinTree::BinLeaf(leaf) }
            else {
                insert_range(if start > node_start {
                    BinTree::BinNode(BinNode {
                        value: start,
                        left: Box::new(BinTree::BinLeaf(BinLeaf { fresh: false })),
                        right: Box::new(BinTree::BinLeaf(BinLeaf { fresh: false })),
                    })
                } else {
                    BinTree::BinNode(BinNode {
                        value: end + 1,
                        left: Box::new(BinTree::BinLeaf(BinLeaf { fresh: false })),
                        right: Box::new(BinTree::BinLeaf(BinLeaf { fresh: false })),
                    })
                }, start, end, node_start, node_end)
            }
        },
        BinTree::BinNode(node) => {
            if node_end < node.value { insert_range(*node.left, start, end, node_start, node.value - 1) } // todo
            else if node_start >= node.value { insert_range(*node.right, start, end, node.value, node_end) } // todo
            else {
                // println!("Subinsertion.");
                let centered = node.value > start && node.value <= end;
                let (left, right) = (
                    insert_range(*node.left, start, end, node_start, node.value - 1),
                    insert_range(*node.right, start, end, node.value, node_end)
                );
                match (left, right) {
                    (BinTree::BinLeaf(left_leaf), BinTree::BinLeaf(right_leaf)) => {
                        // println!("... inserting. {} {} {} {}, {}", start, end, node_start, node_end, node.value);
                        // println!(": Cherry");
                        if left_leaf.fresh == right_leaf.fresh {
                            BinTree::BinLeaf(BinLeaf { fresh: left_leaf.fresh })
                        } else {
                            BinTree::BinNode(BinNode {
                                value: node.value,
                                left: Box::new(BinTree::BinLeaf(left_leaf)),
                                right: Box::new(BinTree::BinLeaf(right_leaf)),
                            })
                        }
                    },
                    (BinTree::BinLeaf(left_leaf), right) => {
                        // println!("... inserting. {} {} {} {}, {}", start, end, node_start, node_end, node.value);
                        // println!(": Left-Leaf");
                        BinTree::BinNode(BinNode {
                            value: if left_leaf.fresh && centered { end + 1 } else { node.value },
                            right: Box::new(if left_leaf.fresh && centered {
                                insert_range(right, start, end, end + 1, node_end)
                            } else {
                                right
                            }),
                            left: Box::new(BinTree::BinLeaf(left_leaf)),
                        })
                    },
                    (left, BinTree::BinLeaf(right_leaf)) => {
                        // println!("... inserting. {} {} {} {}, {}", start, end, node_start, node_end, node.value);
                        // println!(": Right-Leaf");
                        BinTree::BinNode(BinNode {
                            value: if right_leaf.fresh && centered { start } else { node.value },
                            left: Box::new(if right_leaf.fresh && centered {
                                insert_range(left, start, end, node_start, start - 1)
                            } else {
                                left
                            }),
                            right: Box::new(BinTree::BinLeaf(right_leaf)),
                        })
                    },
                    (left, right) => {
                        // println!("... inserting. {} {} {} {}", start, end, node_start, node_end);
                        // println!(": Split");
                        BinTree::BinNode(BinNode {
                            value: node.value,
                            left: Box::new(left),
                            right: Box::new(right),
                        })
                    }
                }
            }
        }
    }
}