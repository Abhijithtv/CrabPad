pub struct DisplayFileResult{
    pub path: String,
    pub content: String
}

impl DisplayFileResult {
    pub fn new(path: String, content: String)-> Self {
        Self {
            path : path, 
            content: content 
        }
    }
}
