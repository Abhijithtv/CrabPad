use crate::{constants::file_constant, helpers::node_build_helper::{build_internal_node, build_leaf_node}, models::page_content::{node_content_helper, page_content_node::{InternalNode, LeafNode, Node}}};

pub fn build_tree_proxy(content: &[char]) -> Option<Node> {
    if content.is_empty(){
        None
    }else{
        build_tree(0, content.len()-1, content)
    }
}

pub fn build_tree(l:usize, r:usize, content: &[char]) -> Option<Node> {
    if r < l{
        return None;
    }
    
    if r - l + 1 <= file_constant::CHUNCK_SIZE {
        let leaf_node = build_leaf_node(&content[l..=r]); //include r
        return Some(leaf_node);
    }

    let m = (l+r)/2;
    let left_child = build_tree(l, m, content);
    let right_child = build_tree(m+1, r, content);
    let internal_node = build_internal_node(left_child, right_child);
    return Some(internal_node);
}