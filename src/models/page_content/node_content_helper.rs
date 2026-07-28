use crate::models::page_content::page_content_node::Node;

    pub fn try_get_count(node: &Option<Node>)-> usize{
        return node.as_ref().map_or(0, |x| get_count(x));
    }

    pub fn try_get_count_for_box_node(node: &Option<Box<Node>>) -> usize{
        return  node.as_ref().map_or(0, |x|get_count(x));
    }

    pub fn get_count(node: &Node) -> usize{
        match node {
            Node::Internal(x) => x.left_char_count + x.right_char_count,
            Node::Leaf(x) => x.content.len(),
        }
    }
