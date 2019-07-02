
extern crate serde;
extern crate serde_json;

use serde::Serialize;
use serde::Deserialize;
use serde_json::Result;

#[derive(Debug,Default)]
#[derive(Serialize, Deserialize)]
pub struct UpdateKey {
    pub key:String,
    pub value:String,
}



#[derive(Debug,Default)]
#[derive(Serialize, Deserialize)]
pub struct UpdateVersion {
    pub version:String,
    pub md5:String,
    pub url:String,
}
