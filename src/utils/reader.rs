use std::fs;

pub fn readPath(path: &str) -> String {
    let mut contents = fs::read_to_string(path).expect("Something went wrong reading the file");
    contents.to_string()
}