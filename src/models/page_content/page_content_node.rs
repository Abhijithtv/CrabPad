use std::ops::{Deref, DerefMut};

use crate::models::page_content::node_content_helper;

pub struct InternalNode{
    pub left: Option<Box<Node>>,
    pub right: Option<Box<Node>>,
    pub left_char_count: usize, //todo - make it option
    pub right_char_count: usize
}

pub struct LeafNode{
    pub content: Vec<char>
}
 
pub enum Node{
    Internal(InternalNode),
    Leaf(LeafNode)
}

impl InternalNode {
    pub fn should_delete_internal_node(&self) -> bool{
        return self.left.is_none() && self.right.is_none();
    }
    
    pub fn recalulate_count(&mut self){
        let left_count = node_content_helper::try_get_count_for_box_node(&self.left);
        let right_count = node_content_helper::try_get_count_for_box_node(&self.right);
        self.left_char_count = left_count;
        self.right_char_count = right_count;
    }
}