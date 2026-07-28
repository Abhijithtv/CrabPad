use std::usize;

use crate::{common::tree_builder, constants::file_constant, helpers::node_build_helper::build_internal_node, models::page_content::{node_content_helper::{self, get_count}, page_content_node::{InternalNode, LeafNode, Node}}};

impl Node {
    pub fn add_content(self, start_index:usize, content_to_insert: &mut Vec<char>)->Option<Box<Node>> {
        match self {
            Node::Internal(mut node) => {
                if start_index < node.left_char_count{ 
                    node.left = Self::add_content(*node.left.unwrap(), start_index, content_to_insert);
                }
                else if start_index >= node.left_char_count{
                    node.right = Self::add_content(*node.right.unwrap(), start_index - node.left_char_count, content_to_insert);
                }
                node.recalulate_count();
                return Some(Box::new(Node::Internal(node)));
            },
            Node::Leaf(mut node) => {  
                if node.can_fit_in_node(content_to_insert.len()){
                    node.insert(start_index, &content_to_insert);
                    return Some(Box::new(Node::Leaf(node)));
                }
                return Some(Box::new(node.insert_with_overflow(start_index, content_to_insert)));
            }
        }
    }
}



impl LeafNode {
    fn can_fit_in_node(&self, len: usize) -> bool {
        return  self.content.len() + len <= file_constant::CHUNCK_SIZE
    }

    fn insert(&mut self, start_index: usize, content_to_insert: &[char]) {
        self.content.splice(
            start_index..start_index,
            content_to_insert.iter().copied()
        );
    }

    fn insert_with_overflow(mut self, start_index: usize, content_to_insert:&mut Vec<char>)->Node{
        //remove content from the start index of leaf node
        let mut content_to_shift:Vec<char> = self.content.drain(start_index..).collect();
        //find free space to use in leaf node
        let content_to_replace_len = usize::min(content_to_insert.len(), file_constant::CHUNCK_SIZE - self.content.len());
        //copy content from value to insert to fill up free space
        let content_to_replace = &content_to_insert[0..content_to_replace_len];
        //insert content to free space
        self.insert(start_index, content_to_replace);
        //add the shifted content from leaf node to pending value to insert
        content_to_insert.append(&mut content_to_shift);
        //build the right tree using the pending char from i = content_to_replace_len
        let right_child = tree_builder::build_tree(content_to_replace_len, content_to_insert.len()-1, content_to_insert);
        //build a new internal node where left = self, right = right child
        let internal_node = build_internal_node(Some(Node::Leaf(self)), right_child);
        return internal_node;
    }
}
