use crate::{handlers::read_cmd_handler, managers::file_content_manager, models::{cmd_actions::CMDActions, cmd_result::CMDResult, cmd_result_status::CMDResultStatus}};

pub fn handle(cmd: &str)->CMDResult{
    let action_and_extras: Option<(&str, Option<&str>)> = split_action_and_extras(cmd);

    return match action_and_extras {
        Some(action_and_extras) => process_cmd(action_and_extras.0, action_and_extras.1),
        None => CMDResult::new(CMDResultStatus::Err, Some("Oops!....Please enter a value".to_string())),
    }
}

fn split_action_and_extras(raw_cmd: &str) -> Option<(&str, Option<&str>)> {
    if raw_cmd.is_empty() {
        return None;
    }

    match raw_cmd.find(' ') {
        Some(i) => Some((&raw_cmd[..i], Some(&raw_cmd[i + 1..]))),
        None => Some((raw_cmd, None)),
    }
}

fn process_cmd(action:&str, extras:Option<&str>) -> CMDResult{
    let action = CMDActions::from_str(action);

    return match action {
        Some(CMDActions::Read) => process_read_cmd(extras),
        Some(CMDActions::Exit) => process_exit_cmd(),
        Some(_) => todo!(),
        None => process_unknown_cmd()   
    }
}

fn process_exit_cmd() -> CMDResult {
    return CMDResult::new(CMDResultStatus::Exit, None);
}

fn process_unknown_cmd() -> CMDResult {
    return CMDResult::new(CMDResultStatus::Err, Some("Unkown-CMD".to_string()))
}

fn process_read_cmd(extras: Option<&str>) -> CMDResult{
    let res = read_cmd_handler::handle_read_cmd(extras);

    match res {
        Ok(res) =>{
            CMDResult::new(CMDResultStatus::Success, None)
        },
        Err(err) => CMDResult::new(CMDResultStatus::Err, Some(err)),
    }
}