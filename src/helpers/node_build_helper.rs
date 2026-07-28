use crate::models::page_content::{node_content_helper, page_content_node::{InternalNode, LeafNode, Node}};

pub fn build_internal_node(left_child: Option<Node>, right_child: Option<Node>) -> Node {
    let left_char_count = node_content_helper::try_get_count(&left_child);
    let right_char_count = node_content_helper::try_get_count(&right_child);
    let node = Node::Internal(InternalNode{
        left: left_child.map(Box::new),
        right: right_child.map(Box::new),
        left_char_count: left_char_count,
        right_char_count: right_char_count,
    });
    return node;
}

pub fn build_leaf_node(content:&[char]) -> Node {
    let leaf_node = Node::Leaf(LeafNode{
        content: content.to_vec()
    });

    return leaf_node;
}