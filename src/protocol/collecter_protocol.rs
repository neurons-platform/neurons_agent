

extern crate serde;
extern crate serde_json;

use serde::Serialize;
use serde::Deserialize;
use serde_json::Result;

use super::super::collecter::collecter::{LogWatcher};
use super::super::utils::json;

#[derive(Debug,Default)]
#[derive(Serialize, Deserialize)]
pub struct UpdateCollecter {
    pub key:String,
    pub value:Vec<LogWatcher>,
}

impl UpdateCollecter {
    pub fn to_json_string(&self) ->String {
        let json_string = json::struct_to_json_string(&self);
        return json_string;
    }
    pub fn value_to_json_string(&self) -> String {
        let json_string = json::struct_to_json_string(&self.value);
        return json_string;
    }
}

#[derive(Debug,Default,Clone)]
#[derive(Serialize, Deserialize)]
pub struct  CollectReport {
    pub time:u64,
    pub count:u64,
    pub system:String,
    pub env:String,
    pub cluster:String,
    pub ip:String,
    pub count_type:String,
}

impl CollectReport {
    pub fn to_json_string(&self) ->String {
        let json_string = json::struct_to_json_string(&self);
        return json_string;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_update_collecter_impl() {
        let update_collecter = UpdateCollecter::default();
        let json_string = update_collecter.to_json_string();
        println!("{}",json_string);
    }

    #[test]
    fn test_value_to_json_string() {
        let update_collecter = UpdateCollecter::default();
        let json_string = update_collecter.value_to_json_string();
        println!("{}",json_string);
    }
}

