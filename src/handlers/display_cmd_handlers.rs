use std::io::{self, Write};

use crate::{managers::file_content_manager};

pub fn handle_display_cmd(extras:Option<&str>){
    match extras {
        Some(extras) => {
            let args:Vec<&str> = extras.split(' ').collect();
            if args.len() != 2{
                println!("Specify start index and length");
                return;
            }
            display(Some(get_usize(args[0])), Some(get_usize(args[1])));
        }
        None => display(None, None),
    };
}

fn get_usize(size_string:&str)->usize{
    return size_string.parse::<usize>().unwrap();
}

fn display(start_index:Option<usize>, len:Option<usize>){
    let root = file_content_manager::get_root_node();
    match root.as_ref(){
        Some(x) => {
            x.display(start_index, len);
            print!("\n");
            io::stdout().flush().unwrap();
        }
        None => println!("Please read a file first"),
    }
}

