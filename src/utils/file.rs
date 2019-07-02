use std::fs::File;
use std::io::prelude::*;


pub fn read_all_from_file(path:&str) -> String {
    let mut file = match File::open(path) {
        Ok(file) => file,
        Err(_) => return "".to_string(),
    };
    let mut contents = String::new();
    match file.read_to_string(&mut contents) {
        Ok(_) => return contents,
        Err(_) => return "".to_string(),
    };
}
