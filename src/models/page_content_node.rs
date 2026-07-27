pub struct InternalNode{
    pub left: Option<Box<Node>>,
    pub right: Option<Box<Node>>,
    pub left_char_count: usize,
    pub right_char_count: usize
}

pub struct LeafNode{
    pub content: Vec<char>
}
 
pub enum Node{
    Internal(InternalNode),
    Leaf(LeafNode)
}

impl Node {
    pub fn display(&self, start_index:Option<usize>, len:Option<usize>){
        let start_index: usize = start_index.unwrap_or(0); //from 0st char
        let len: usize = len.unwrap_or(Self::get_count(&self)); //until last char

        match self {
            Node::Internal(node) => {
                if len + start_index <= node.left_char_count{
                    Self::try_display(&node.left, start_index, len);
                }
                else if start_index >= node.left_char_count{
                    Self::try_display(&node.right, start_index - node.left_char_count, len);
                }
                else {
                    Self::try_display(&node.left, start_index, node.left_char_count - start_index);
                    Self::try_display(&node.right, 0, len - (node.left_char_count - start_index));
                }
            },
            Node::Leaf(node) => {
                for c in &node.content[start_index..start_index+len]{
                    print!("{}", c);
                }
            }
        }
    }

    fn try_display(node: &Option<Box<Node>>, start_index:usize, len:usize){
        match node.as_ref(){
            Some(node) => node.display(Some(start_index), Some(len)),
            None => {},
        }
    }

    pub fn try_get_count(node: &Option<Node>)-> usize{
        return node.as_ref().map_or(0, |x| Self::get_count(x));
    }

    pub fn get_count(node: &Node) -> usize{
        match node {
            Node::Internal(x) => x.left_char_count + x.right_char_count,
            Node::Leaf(x) => x.content.len(),
        }
    }
}


