pub struct InternalNode{
    pub left: Box<Option<NodeV2>>,
    pub right: Box<Option<NodeV2>>,
    pub total_char_count: usize,
}

pub struct LeafNode{
    pub total_char_count: usize,
    pub content: Vec<char>
}
 
pub enum NodeV2{
    Internal(InternalNode),
    Leaf(LeafNode)
}