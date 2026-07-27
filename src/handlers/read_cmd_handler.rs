use crate::{helpers::file_helper, managers::file_content_manager, models::read_file_result::ReadFileResult};

pub fn handle_read_cmd(extra: Option<&str>)->Result<ReadFileResult, String>{
    match extra {
        Some(path)=>{
            let path = path.trim();
            let read_res = file_helper::read(path);
            return match read_res {
                Ok(res) => {
                    file_content_manager::build_content_tree(&res);
                    println!("Read Complete");
                    Ok(ReadFileResult::new(path.to_string(), res))
                },
                std::result::Result::Err(err) => std::result::Result::Err(err),
            }
        },
        None => std::result::Result::Err("No path specified".to_string())
    }
}