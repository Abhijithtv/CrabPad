pub struct InternalNode{
    pub left: Box<Option<Node>>,
    pub right: Box<Option<Node>>,
    pub total_char_count: usize,
}

pub struct LeafNode{
    pub total_char_count: usize,
    pub content: Vec<char>
}
 
pub enum Node{
    Internal(InternalNode),
    Leaf(LeafNode)
}