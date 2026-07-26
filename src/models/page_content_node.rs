pub struct Node{
    pub left: Box<Option<Node>>,
    pub right: Box<Option<Node>>,
    pub total_char_count: usize,
    pub content: Option<Vec<char>>
}

struct InternalNode{
    pub left: Box<Option<NodeV2>>,
    pub right: Box<Option<NodeV2>>,
    pub total_char_count: usize,
}

struct LeafNode{
    pub total_char_count: usize,
    pub content: Option<Vec<char>>
}

pub enum NodeV2{
    Internal(InternalNode),
    Leaf(LeafNode)
}

