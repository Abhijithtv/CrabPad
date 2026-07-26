//static mut ROOT_NODE: Option<Node>;

use std::sync::{LazyLock, Mutex};

use crate::models::page_content_node::{Node};

static CONTENT_ROOT: LazyLock<Mutex<Option<Node>>> = LazyLock::new(|| Mutex::new(None));
const CHUNCK_SIZE:usize = 50;
//const EXTRA_BUFF_SIZE:usize = 14; 

pub fn build_content_tree(content: String){
    let chars: Vec<char> = content.chars().collect();
    let mut root = CONTENT_ROOT.lock().unwrap();
    *root = build_tree(0, content.len(), &chars);
}

fn build_tree(l:usize, r:usize, content: &Vec<char>) -> Option<Node> {
    if r < l{
        return None;
    }
    
    if r - l <= CHUNCK_SIZE {
        let leaf_node = Node {
            content: Some(content[l..r].to_vec()),
            left: Box::new(None),
            right: Box::new(None),
            total_char_count : r-l+1
        };
        return Some(leaf_node);
    }

    let m = (l+r)/2;

    let mut node = Node{
        left: Box::new(build_tree(l, m, content)),
        right: Box::new(build_tree(m+1, r, content)),
        content: None,
        total_char_count: 0
    };

    node.total_char_count = get_count(&(node.left))  + get_count(&(node.right));
    
    return Some(node);
}

fn get_count(node: &Option<Node>)-> usize{
    return if node.as_ref().is_none() {
        0
    } else {
        node.as_ref().unwrap().total_char_count
    };
}