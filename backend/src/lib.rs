use std::{
    borrow::{Borrow, BorrowMut},
    cell::RefCell,
    rc::Rc,
};

const MAX: usize = 4;

/// InternalNode holds a pointer to another node
///
/// LeafNode holds the data for a specific key and a linked list structure of the next leaf node (From left to right)
#[derive(Debug)]
pub enum NodeType {
    InternalNode {
        keys: Vec<u32>,
        children: Vec<Rc<Node>>,
    },
    LeafNode {
        keys: Vec<u32>,
        data: Vec<u32>,
        next: Option<Rc<Node>>,
    },
}

#[derive(Debug)]
pub struct Node {
    node_type: NodeType,
}

/// Each node except root can have a maximum of M children and at least ceil(M/2) children.
///
/// Each node can contain a maximum of M – 1 keys and a minimum of ceil(M/2) – 1 keys.
///
/// The root has at least two children and atleast one search key.
#[derive(Debug)]
pub struct BTree {
    root: Rc<Node>,
}

impl BTree {
    fn insert(&mut self, val: u32) {
        todo!()
    }

    pub fn search(&self, key: u32) -> Option<u32> {
        let root = self.root.clone();
        BTree::search_node(&root, key)
    }

    fn search_node(node: &Rc<Node>, key: u32) -> Option<u32> {
        match &node.node_type {
            NodeType::InternalNode { keys, children } => {
                let mut child_index = keys.len();
                for (i, node_key) in keys.iter().enumerate() {
                    if &key < node_key {
                        child_index = i;
                        break;
                    }
                }
                BTree::search_node(&children[child_index], key)
            }
            NodeType::LeafNode { keys, data, .. } => {
                for i in 0..keys.len() {
                    if keys[i] == key {
                        return Some(data[i]);
                    }
                }
                None
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one_leaf_search() {
        let tree = BTree {
            root: Rc::new(Node {
                node_type: NodeType::LeafNode {
                    keys: vec![1],
                    data: vec![1337],
                    next: None,
                },
            }),
        };

        let val = tree.search(1);

        assert_eq!(val, Some(1337));
    }

    #[test]
    fn test_multiple_leaf_search() {
        let left = Rc::new(Node {
            node_type: NodeType::LeafNode {
                keys: vec![10, 15, 20, 28],
                data: vec![100, 150, 200, 280],
                next: None,
            },
        });

        let left_middle = Rc::new(Node {
            node_type: NodeType::LeafNode {
                keys: vec![30, 35, 45, 48],
                data: vec![300, 350, 450, 480],
                next: Some(Rc::clone(&left)),
            },
        });

        let right_middle = Rc::new(Node {
            node_type: NodeType::LeafNode {
                keys: vec![50, 58, 65, 68],
                data: vec![500, 580, 650, 680],
                next: Some(Rc::clone(&left_middle)),
            },
        });

        let right = Rc::new(Node {
            node_type: NodeType::LeafNode {
                keys: vec![70, 85, 90, 92],
                data: vec![700, 850, 900, 920],
                next: Some(Rc::clone(&right_middle)),
            },
        });

        let tree = BTree {
            root: Rc::new(Node {
                node_type: NodeType::InternalNode {
                    keys: vec![30, 50, 70],
                    children: vec![left, left_middle, right_middle, right],
                },
            }),
        };

        let val = tree.search(92);

        assert_eq!(val, Some(920));
    }
}
