// use std::borrow::Borrow;

// use crate::{
//     node::Node,
//     node_type::{KeyValuePair, NodeType},
// };

// pub struct BTree {
//     order: u32,
//     root: Node,
// }

// impl BTree {
//     pub fn search(&self, key: usize) -> Option<KeyValuePair> {
//         let root = &self.root;
//         self.search_node(root, key)
//     }

//     fn search_node(&self, node: &Node, key: usize) -> Option<KeyValuePair> {
//         match &node.node_type {
//             NodeType::Internal(keys, children) => {
//                 let mut child_index = keys.len();

//                 println!("Keys: {:?}", keys);
//                 println!("Children: {:?}", children);

//                 for (i, node_key) in keys.iter().enumerate() {
//                     if key < node_key.0 {
//                         child_index = i;
//                         break;
//                     }
//                 }
//                 println!("Child index: {:?}", child_index);
//                 self.search_node(&children[child_index], key)
//             }
//             NodeType::Leaf(pairs) => {
//                 println!("Pairs: {:?}", pairs);
//                 if let Some(pair) = pairs.iter().find(|&pair| pair.key == key) {
//                     return Some(pair.clone());
//                 }

//                 None
//             }
//         }
//     }
// }

// #[cfg(test)]
// mod tests {
//     use crate::{node::Node, node_type::{Key, KeyValuePair, NodeType}};

//     use super::BTree;

//     #[test]
//     fn test_search() {
//         let btree = BTree {
//             order: 4,
//             root: Node::new(
//                 NodeType::Internal(
//                     vec![Key(5), Key(7), Key(10), Key(15), Key(17)],
//                     vec![
//                         Node::new(NodeType::Leaf(vec![KeyValuePair { key: 5, value: "5".to_string() }]), false),
//                         Node::new(NodeType::Leaf(vec![KeyValuePair { key: 7, value: "7".to_string() }]), false),
//                         Node::new(NodeType::Leaf(vec![KeyValuePair { key: 10, value: "10".to_string() }]), false),
//                         Node::new(NodeType::Leaf(vec![KeyValuePair { key: 15, value: "15".to_string() }]), false),
//                         Node::new(NodeType::Leaf(vec![KeyValuePair { key: 17, value: "17".to_string() }]), false),
//                     ],
//                 ),
//                 true,
//             ),
//         };

//         let result = btree.search(10).unwrap();
//         assert_eq!(result, KeyValuePair { key: 10, value: "10".to_string() });
//     }
// }
