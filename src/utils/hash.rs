extern crate md5;


pub fn get_string_md5(string:String) -> String {
    let digest = md5::compute(&string);
    return format!("{:x}", digest);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_string_md5() {
        let s =  get_string_md5("jingminglang".to_string());
        println!("{}",s);
    }

}
