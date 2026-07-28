use crate::models::page_content::{node_content_helper, page_content_node::Node};

impl Node {
    pub fn delete_content(self, start_index:usize, len:usize)->Option<Box<Node>> {
        if node_content_helper::get_count(&self) > len{
            println!("Please ensure length is correct")
        }

        if self.should_delete_entire_node(start_index, len){
            return None
        }

        match self {
            Node::Internal(mut node) => {
                if len + start_index <= node.left_char_count{ 
                    node.left = Self::delete_content(*node.left.unwrap(), start_index, len);
                    //todo - count
                }
                else if start_index >= node.left_char_count{
                    node.right = Self::delete_content(*node.right.unwrap(), start_index - node.left_char_count, len);
                }
                else {
                    node.left = Self::delete_content(*node.left.unwrap(), start_index, node.left_char_count - start_index);
                    node.right = Self::delete_content(*node.right.unwrap(), 0, len - (node.left_char_count - start_index));
                }

                if node.should_delete_internal_node(){
                    return None;
                }
                else{
                   node.recalulate_count();
                   return Some(Box::new(Node::Internal(node)));
                }
            },
            Node::Leaf(mut node) => {            
                node.content.drain(start_index..start_index+len-1);
                return Some(Box::new(Node::Leaf(node)));
            }
        }
    }

    fn should_delete_entire_node(&self, start_index:usize, len:usize)->bool{
        if start_index != 0{
            return false;
        }
        match self {
            Node::Internal(internal_node) => {
                return internal_node.left_char_count + internal_node.right_char_count == len;
            },
            Node::Leaf(leaf_node) => {
                return leaf_node.content.len() == len;
            },
        }
    }
}
