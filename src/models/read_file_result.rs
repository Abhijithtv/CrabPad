pub struct ReadFileResult{
    pub path: String,
    pub content: String
}
impl ReadFileResult {
    pub fn new(path: String, content: String)-> Self {
        Self {
            path : path, 
            content: content 
        }
    }
}
