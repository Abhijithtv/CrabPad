use crate::{handlers::cmd_handlers, interaction_medium::abstractions::starter_base, models::cmd_result_status::CMDResultStatus};

pub struct ConsoleWindow {

}

impl starter_base::Startable for ConsoleWindow {
    fn start(&self) {
        println!("<<<<<<<<<<<<<Starting::Console Window>>>>>>>>>>>");
        read_cmds();
        println!("<<<<<<<<<<<<<Stoping::Console Window>>>>>>>>>>>>");
    }
}

impl Default for ConsoleWindow {
    fn default() -> Self {
        Self {
        }
    }
}

fn read_cmds(){
    let mut cmd_buffer = String::new();
    
    loop{
        std::io::stdin().read_line(&mut cmd_buffer).unwrap();
        let cmd: &str = cmd_buffer.trim();
        let res = cmd_handlers::handle(cmd);
        match res.status{
            CMDResultStatus::Err => eprintln!("{}",res.message.unwrap_or("Some error occurred".to_string())),
            _ => {}
        } 
        cmd_buffer.clear();
    }
}

