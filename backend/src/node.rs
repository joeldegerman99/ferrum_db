use std::fmt::Error;

use crate::node_type::{Key, NodeType};

#[derive(Debug, PartialEq)]
pub struct Node {
    pub node_type: NodeType,
    pub is_root: bool,
}

impl Node {
    pub fn new(node_type: NodeType, is_root: bool) -> Node {
        Node { node_type, is_root }
    }

    pub fn split(&mut self, t: usize) -> Result<(Key, Node), Error> {
        match self.node_type {
            NodeType::Internal(ref mut children, ref mut keys) => {
                let mut sibling_keys = keys.split_off(t - 1);
                let median_key = sibling_keys.remove(0);
                let sibling_children = children.split_off(t);

                Ok((
                    median_key,
                    Node::new(NodeType::Internal(sibling_children, sibling_keys), false),
                ))
            }
            NodeType::Leaf(ref mut pairs) => {
                let siblings = pairs.split_off(t);
                println!("Sibling: {:?}", siblings);

                let median_pair = siblings.get(0).ok_or(Error)?.clone();
                Ok((
                    Key(median_pair.key),
                    Node::new(NodeType::Leaf(siblings), false),
                ))
            }
            NodeType::Unexpected => todo!(),
        }
    }
}
#[cfg(test)]
mod tests {
    use crate::node_type::{Key, KeyValuePair, NodeType, Offset};

    use super::Node;

    #[test]
    fn test_split_internal() {
        let mut internal = Node::new(
            NodeType::Internal(vec![Offset(0)],vec![Key(5), Key(7), Key(10), Key(15), Key(17)]),
            false,
        );

        println!("Internal: {:?}", internal);
        let split = internal.split(4).unwrap();
        println!("Split: {:?}", split);
        println!("Internal: {:?}", internal);

        assert_eq!(split.0, Key(15));
    }
    #[test]
    fn test_split_leaf() {
        let mut leaf = Node::new(
            NodeType::Leaf(vec![
                KeyValuePair {
                    key: 5,
                    value: String::from("Val 5"),
                },
                KeyValuePair {
                    key: 7,
                    value: String::from("Val 7"),
                },
                KeyValuePair {
                    key: 10,
                    value: String::from("Val 10"),
                },
                KeyValuePair {
                    key: 12,
                    value: String::from("Val 12"),
                },
            ]),
            false,
        );

        let split = leaf.split(3).unwrap();

        assert_eq!(
            leaf.node_type,
            NodeType::Leaf(vec![
                KeyValuePair {
                    key: 5,
                    value: String::from("Val 5"),
                },
                KeyValuePair {
                    key: 7,
                    value: String::from("Val 7"),
                },
            ])
        );
        assert_eq!(split.0, Key(10));
        assert_eq!(
            split.1.node_type,
            NodeType::Leaf(vec![
                KeyValuePair {
                    key: 10,
                    value: String::from("Val 10"),
                },
                KeyValuePair {
                    key: 12,
                    value: String::from("Val 12"),
                },
            ])
        );
    }
}
