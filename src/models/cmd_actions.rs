pub enum CMDActions{
    Unknown,
    Read,
    Write,
    Display,
    Exit
}

impl CMDActions {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.trim().to_lowercase().as_str() {
            "read" => Some(Self::Read),
            "write" => Some(Self::Write),
            "exit" => Some(Self::Exit),
            "display" => Some(Self::Display),
            _ => None,
        }
    }
}