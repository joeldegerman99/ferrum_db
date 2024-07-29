use std::mem::size_of;

pub const PAGE_SIZE: usize = 4096;

pub const PTR_SIZE: usize = size_of::<usize>();

pub const IS_ROOT_SIZE: usize = 1;
pub const IS_ROOT_OFFSET: usize = 0;
pub const NODE_TYPE_SIZE: usize = 1;
pub const NODE_TYPE_OFFSET: usize = 1;
pub const PARENT_POINTER_SIZE: usize = PTR_SIZE;

pub const NODE_HEADER_SIZE: usize = IS_ROOT_SIZE + NODE_TYPE_SIZE + PARENT_POINTER_SIZE;

pub trait FromByte {
    fn from_byte(&self) -> bool;
}

pub trait ToByte {
    fn to_byte(&self) -> u8;
}

impl FromByte for u8 {
    fn from_byte(&self) -> bool {
        matches!(self, 0x01)
    }
}

impl ToByte for bool {
    fn to_byte(&self) -> u8 {
        match self {
            true => 0x01,
            false => 0x00,
        }
    }
}
