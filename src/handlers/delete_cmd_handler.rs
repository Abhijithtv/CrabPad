use crate::managers::file_content_manager;

pub fn handle_delete_cmd(extras:Option<&str>){
    match extras {
        Some(extras) => {
            let args:Vec<&str> = extras.split(' ').collect();
            if args.len() != 2{
                println!("Specify start index and length");
                return;
            }
            delete(get_usize(args[0]), get_usize(args[1]));
        }
        None => println!("Specify start index and length"),
    };
}

fn get_usize(size_string:&str)->usize{
    return size_string.parse::<usize>().unwrap();
}

fn delete(start_index:usize, len:usize){
    let mut root = file_content_manager::get_root_node();
    let cur_root = root.take();
    match cur_root{
        Some(x) => {
            *root = x.delete_content(start_index, len);
            println!("Deletion Completed");
        }
        None => println!("Please read a file first"),
    }
}