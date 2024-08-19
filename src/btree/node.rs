use zerocopy::{AsBytes, ByteSlice, FromBytes, LayoutVerified};

pub const NODE_TYPE_LEAF: [u8; 8] = *b"LEAF    ";
pub const NODE_TYPE_BRANCH: [u8; 8] = *b"BRANCH  ";

#[derive(Debug, FromBytes, AsBytes)]
#[repr(C)]
pub struct Header {
    pub node_type: [u8; 8],
}

pub struct Node<B> {
    pub header: LayoutVerified<B, Header>,
    pub body: B,
}

impl<B: ByteSlice> Node<B> {
    pub fn new(bytes: B) -> Self {
        let (header, body) = LayoutVerified::new_from_prefix(bytes).expect("node must be aligned");
        Self { header, body }
    }
}