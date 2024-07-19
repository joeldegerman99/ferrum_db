use std::{
    borrow::{Borrow, BorrowMut},
    cell::RefCell,
    rc::Rc,
};

const MAX: usize = 3;


/// InternalNode holds a pointer to another node
/// 
/// LeafNode holds the data for a specific key and a linked list structure of the next leaf node (From left to right)
#[derive(Debug)]
enum NodeType {
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
struct Node {
    node_type: NodeType,
}

/// Each node except root can have a maximum of M children and at least ceil(M/2) children.
///
/// Each node can contain a maximum of M – 1 keys and a minimum of ceil(M/2) – 1 keys.
///
/// The root has at least two children and atleast one search key.
#[derive(Debug)]
struct BTree {
    root: Rc<Node>,
}

impl BTree {
    fn insert(&mut self, val: u32) {
        todo!()
    }

    fn search(&self, key: u32) -> Option<u32> {
        let root = self.root.clone();
        BTree::search_node(&root, key)
    }

    fn search_node(node: &Rc<Node>, key: u32) -> Option<u32> {
        match &node.node_type {
            NodeType::InternalNode { keys, children } => {
                println!("Keys: {:?}", keys);
                let mut child_index = keys.len();
                println!("Child index: {}", child_index);
                for (i, node_key) in keys.iter().enumerate() {
                    println!("Node key: {}", node_key);
                    println!("Key: {}", key);

                    if &key < node_key {
                        child_index = i;
                        println!("New child index (lt): {}", child_index);
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
        let second = Rc::new(Node {
            node_type: NodeType::LeafNode {
                keys: vec![3],
                data: vec![1338],
                next: None,
            },
        });
        let first = Rc::new(Node {
            node_type: NodeType::LeafNode {
                keys: vec![1],
                data: vec![1337],
                next: Some(Rc::clone(&second)),
            },
        });

        let tree = BTree {
            root: Rc::new(Node {
                node_type: NodeType::InternalNode {
                    keys: vec![2],
                    children: vec![first, second],
                },
            }),
        };
        println!("Tree: {tree:?}");

        let val = tree.search(3);

        assert_eq!(val, Some(1338));
    }
}
