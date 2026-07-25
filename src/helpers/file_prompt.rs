pub fn read_file_path() -> String{
    println!("Enter file to read");
    let mut line = String::new();
    let path = std::io::stdin().read_line(&mut line).unwrap().to_string();
    return path;
}