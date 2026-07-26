//static mut ROOT_NODE: Option<Node>;

use std::sync::{LazyLock, Mutex};

use crate::{common::{tree_builder}, models::{page_content_node::Node}};

static CONTENT_ROOT: LazyLock<Mutex<Option<Node>>> = LazyLock::new(|| Mutex::new(None));

pub fn build_content_tree(content: &str){
    let chars: Vec<char> = content.chars().collect();
    let new_root = tree_builder::build_tree_proxy(&chars);
    {
        //lock root
        let mut root = get_root_node();
        *root = new_root
    }
}

pub fn get_root_node()->std::sync::MutexGuard<'static, Option<Node>>{
    return CONTENT_ROOT.lock().unwrap();
}