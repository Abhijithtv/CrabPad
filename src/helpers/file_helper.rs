use std::fs;

pub fn read(path: &str)->Result<String, String>{
    if path.is_empty() {
        return Err("File path cant be empty while reading".to_string());
    }

    let res = fs::read_to_string(path);

    if res.is_err(){
        return  Err(res.err().unwrap().to_string());
    }

    return Ok(res.unwrap());
}