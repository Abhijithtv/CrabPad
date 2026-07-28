use std::io::{self, Write};

use crate::{managers::file_content_manager};

pub fn handle_insert_cmd(extras:Option<&str>){
    match extras {
        Some(extras) => {
            let args:Vec<&str> = extras.split(' ').collect();
            if args.len() != 2{
                println!("Specify start index and content");
                return;
            }
            let mut content_to_insert:Vec<char> = args[1].chars().collect();
            insert(get_usize(args[0]), &mut content_to_insert);
        }
        None => println!("Specify start index and content"),
    };
}

fn get_usize(size_string:&str)->usize{
    return size_string.parse::<usize>().unwrap();
}

fn insert(start_index:usize, content_to_insert:&mut Vec<char>){
    let mut root = file_content_manager::get_root_node();
    let cur_root = root.take();
    match cur_root{
        Some(x) => {
            x.add_content(start_index, content_to_insert);
            println!("Added successfully");
            io::stdout().flush().unwrap();
        }
        None => println!("Please read a file first"),
    }
}

