
extern crate ini;
use ini::Ini;


pub fn conf_get(conf_name:&str,section:&str,key:&str) -> String {
    let conf = match Ini::load_from_file(conf_name) {
        Ok(conf) => conf,
        Err(_) => return "".to_string(),
    };
    let section = match conf.section(Some(section.to_owned())) {
        Some(section) => section,
        None => return "".to_string(),
    };
    let value = match section.get(key) {
        Some(value) => value,
        None => return "".to_string(),
    };
    return value.clone();
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_conf_get() {
        let value = conf_get("config.ini","redis","url");
        println!("value {}",value);
    }

}

