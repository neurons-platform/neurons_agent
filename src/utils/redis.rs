extern crate redis;
use redis::Commands;

pub struct Redis {
    pub addr: String,
    pub password: String,
}

impl Redis {
    pub fn brpop(&self,key:&str,timeout:u32) -> String {
        let url: String = format!("redis://:{}@{}/", self.password, self.addr);
        let client = match redis::Client::open(&url[..]) {
            Ok(client) => client,
            Err(err) => {
                println!("redis connect err:{:?}",err);
                return "".to_string();
            },
        };
        match client.get_connection() {
            Ok(con) => {
                match redis::cmd("BRPOP").arg(key).arg(timeout).query(&con) {
                    Ok(r) => {
                        let (_,v): (String,String) = r;
                        return v;
                    } ,
                    Err(err) => {
                        println!("redis brpop query err:{:?}",err);
                        return "".to_string();
                    },
                }
            },
            Err(err) => {
                println!("redis get connection err:{:?}",err);
                return "".to_string();
            },
        };
    }

    pub fn rpop(&self,key:&str) -> String {
        let url: String = format!("redis://:{}@{}/", self.password, self.addr);
        let client = match redis::Client::open(&url[..]) {
            Ok(client) => client,
            Err(_) => return "".to_string(),
        };
        match client.get_connection() {
            Ok(con) => {
                match  redis::cmd("RPOP").arg(key).query(&con) {
                    Ok(v) => return v,
                    Err(_) => return "".to_string(),
                };
            },
            Err(_) => return "".to_string(),
        };
    }

    pub fn get(&self,key:&str) -> String {
        let url: String = format!("redis://:{}@{}/", self.password, self.addr);
        let client = match redis::Client::open(&url[..]) {
            Ok(client) => client,
            Err(_) => return "".to_string(),
        };
        match client.get_connection() {
            Ok(con) => {
                match  redis::cmd("GET").arg(key).query(&con) {
                    Ok(v) => return v,
                    Err(_) => return "".to_string(),
                };
            },
            Err(_) => return "".to_string(),
        };
    }

    pub fn set(&self,key:&str,value:&str) -> bool {
        let url: String = format!("redis://:{}@{}/", self.password, self.addr);
        let client = match redis::Client::open(&url[..]) {
            Ok(client) => client,
            Err(_) => return false,
        };
        match client.get_connection() {
            Ok(con) => {
                redis::cmd("SET").arg(key).arg(value).execute(&con);
                return true;
            },
            Err(_) => return false,
        };
    }
}


