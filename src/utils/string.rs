
use std::str;


pub fn vec_u8_to_string(vc:&Vec<u8>) -> String {
    let s = match str::from_utf8(vc) {
        Ok(v) => v,
        Err(_) => "",
    };
    return s.to_string();
}
