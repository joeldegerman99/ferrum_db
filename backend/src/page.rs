use crate::{dbms_error::DbmsError, node::Node, page_layout::{ToByte, IS_ROOT_OFFSET, NODE_TYPE_OFFSET, PAGE_SIZE}};

pub struct Page {
    data: Box<[u8; PAGE_SIZE]>,
}

impl Page {
    pub fn new(data: [u8; PAGE_SIZE]) -> Page {
        Page {
            data: Box::new(data),
        }
    }

    pub fn get_data(&self) -> [u8; PAGE_SIZE] {
        *self.data
    }
}

impl TryFrom<&Node> for Page {
    type Error = DbmsError;

    fn try_from(node: &Node) -> Result<Self, Self::Error> {
        let mut data: [u8; PAGE_SIZE] = [0x00; PAGE_SIZE];

        // is root byte (0x00 or 0x01 for false or true)
        data[IS_ROOT_OFFSET] = node.is_root.to_byte();
        // node type byte (0x01 for internal, 0x02 for leaf)
        data[NODE_TYPE_OFFSET] = u8::from(&node.node_type);

        todo!()
    }
}
