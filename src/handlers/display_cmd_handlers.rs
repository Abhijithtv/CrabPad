use std::io::{self, Write};

use crate::{managers::file_content_manager};

pub fn handle_display_cmd(){
    {
        let root = file_content_manager::get_root_node();
        match root.as_ref(){
            Some(x) => {
                x.display();
                print!("\n");
                io::stdout().flush().unwrap();
            }
            None => println!("Please read a file first"),
        }
    }
    
}

