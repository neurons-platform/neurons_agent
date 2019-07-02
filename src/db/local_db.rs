
extern crate sled;
use super::super::utils;
use sled::{Db};

pub struct LocalDB {
    pub path:String,
    pub db:Db,
}

impl LocalDB {
    pub fn new(path:String) -> LocalDB {
        let db = sled::Db::start_default(&path).unwrap();
        LocalDB {
            path,
            db,
        }
    }
    pub fn set(&self,key:&str,value:&str) -> bool {
        // println!("set db");
        // let t = sled::Db::start_default(&self.path).unwrap();
        match self.db.set(key, value.as_bytes().to_vec()) {
            Ok(_) => return true,
            Err(_) => return false,
        };
    }

    pub fn del() {
    }

    pub fn get(&self,key:&str) -> String {
        // println!("get db");
        let r = match self.db.get(key) {
            Ok(r) => {
                match r {
                    Some(r) => r.to_vec(),
                    None => b"".to_vec(),
                }
            },
            Err(_) => b"".to_vec(),
        };
        let s = utils::string::vec_u8_to_string(&r);
        return s;
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set() {
    }

}
