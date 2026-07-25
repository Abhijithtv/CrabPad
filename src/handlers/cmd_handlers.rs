use crate::models::{cmd_actions::CMDActions, cmd_result::CMDResult, cmd_result_status::CMDResultStatus};

pub fn handle(cmd: &str)->CMDResult{
    let parts: Vec<&str> = cmd.split(' ').collect();

    if parts.is_empty(){
        let res = CMDResult{
            message: Some("Oops!....Please enter a value".to_string()),
            status: CMDResultStatus::Err,
        };
        return res;
    }

    let action = CMDActions::from_str(parts[0]);

    //todo - quit and other actions
    match action {
        Some(CMDActions::Read) => println!("reading"),
        Some(CMDActions::Write) => println!("writing"),
        Some(_)| None => println!("Ignored cmd - {}", parts[0]),    
    }


    let res: CMDResult = CMDResult{
            message: Some("No issue".to_string()),
            status: CMDResultStatus::Success,
    };
    return res;
}