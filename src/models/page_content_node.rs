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

impl Node {
    pub fn display(&self){
        match self {
            Node::Internal(node) => {
                try_display(&node.left);
                try_display(&node.right);
            },
            Node::Leaf(node) => {
                for c in &node.content{
                    print!("{}", c);
                }
            }
        }
    }
}

fn try_display(node: &Box<Option<Node>>){
    match node.as_ref(){
        Some(node) => node.display(),
        None => {},
    }
}