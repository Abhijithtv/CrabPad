//static mut ROOT_NODE: Option<Node>;

use std::sync::{LazyLock, Mutex};

use crate::{common::{tree_builder}, models::{page_content_node::Node}};

//static CONTENT_ROOT: LazyLock<Mutex<Option<Node>>> = LazyLock::new(|| Mutex::new(None));
static CONTENT_ROOT_V2: LazyLock<Mutex<Option<Node>>> = LazyLock::new(|| Mutex::new(None));


// pub fn build_content_tree(content: &str){
//     let chars: Vec<char> = content.chars().collect();
//     let mut root = CONTENT_ROOT.lock().unwrap();
//     *root = tree_builder::build_tree_proxy(&chars);
// }

pub fn build_content_tree(content: &str){
    let chars: Vec<char> = content.chars().collect();
    let mut root = CONTENT_ROOT_V2.lock().unwrap();
    *root = tree_builder::build_tree_proxy(&chars);
}