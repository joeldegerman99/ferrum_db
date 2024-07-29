#[derive(Debug, PartialEq, Clone)]
pub struct Offset(pub usize);

#[derive(Debug, PartialEq, Clone)]
pub struct Key(pub usize);

#[derive(Debug, Eq, Clone)]
pub struct KeyValuePair {
    pub key: usize,
    pub value: String,
}

impl PartialEq for KeyValuePair {
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key
    }
}
/// InternalNode holds a pointer to another node
///
/// LeafNode holds the data for a specific key and a linked list structure of the next leaf node (From left to right)
#[derive(Debug, PartialEq)]
pub enum NodeType {
    Internal(Vec<Offset>, Vec<Key>),
    Leaf(Vec<KeyValuePair>),
    Unexpected,
}


impl From<u8> for NodeType {
    fn from(byte: u8) -> Self {
        match byte {
            0x01 => NodeType::Internal(vec![], vec![]),
            0x02 => NodeType::Leaf(vec![]),
            _ => NodeType::Unexpected,
        }
    }
}

impl From<&NodeType> for u8 {
    fn from(node_type: &NodeType) -> u8 {
        match node_type {
            NodeType::Internal(_, _) => 0x01,
            NodeType::Leaf(_) => 0x02,
            NodeType::Unexpected => 0x00,
        }
    }
    
}
