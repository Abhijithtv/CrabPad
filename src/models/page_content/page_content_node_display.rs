use crate::models::page_content::{node_content_helper, page_content_node::Node};

impl Node {
    pub fn display(&self, start_index:Option<usize>, len:Option<usize>){
        let start_index: usize = start_index.unwrap_or(0); //from 0st char
        let len: usize = len.unwrap_or(node_content_helper::get_count(&self)); //until last char

        if node_content_helper::get_count(&self) > len{
            println!("Please ensure length is correct")
        }
        
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
}
