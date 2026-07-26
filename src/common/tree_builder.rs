use crate::{constants::file_constant, models::page_content_node::Node};

pub fn build_tree_proxy(content: &[char]) -> Option<Node> {
    if content.is_empty(){
        None
    }else{
        build_tree(0, content.len()-1, content)
    }
}

fn build_tree(l:usize, r:usize, content: &[char]) -> Option<Node> {
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

fn build_internal_node(left_child: Option<Node>, right_child: Option<Node>) -> Node {
    let char_count = get_count(&left_child)  + get_count(&right_child);
    let node = Node{
        left: Box::new(left_child),
        right: Box::new(right_child),
        content: None,
        total_char_count: char_count
    };
    return node;
}

fn build_leaf_node(content:&[char]) -> Node {
    let leaf_node = Node {
            content: Some(content.to_vec()),
            left: Box::new(None),
            right: Box::new(None),
            total_char_count : content.len()
        };
    return leaf_node;
}

fn get_count(node: &Option<Node>)-> usize{
    return node.as_ref().map_or(0, |x| x.total_char_count)
}