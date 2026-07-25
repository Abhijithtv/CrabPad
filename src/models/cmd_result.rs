use crate::models::cmd_result_status::CMDResultStatus;

pub struct CMDResult{
    pub message: Option<String>,
    pub status: CMDResultStatus
}

impl CMDResult {
    pub fn new(status: CMDResultStatus, message: Option<String>)->Self{
        Self {
            message, 
            status 
        }
    }
}