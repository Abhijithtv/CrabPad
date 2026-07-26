pub struct Node{
    pub left: Box<Option<Node>>,
    pub right: Box<Option<Node>>,
    pub total_char_count: usize,
    pub content: Option<Vec<char>>
}