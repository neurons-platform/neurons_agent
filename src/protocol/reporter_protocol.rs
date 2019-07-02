
extern crate serde;
extern crate serde_json;

use serde::Serialize;
use serde::Deserialize;
use super::super::utils::json;



#[derive(Debug,Default)]
#[derive(Serialize, Deserialize)]
pub struct Heartbeat {
    // 协议类型
    pub protocol_type:String,
    // 服务器IP
    pub ip:String,
    // agent版本
    pub version:String,
    // 运行系统的名称
    pub system:String,
    // 运行系统的分组
    pub cluster:String,
    // 运行系统的环境: ...
    pub env:String,
}

impl Heartbeat {
    pub fn to_json_string(&self) ->String {
        let json_string = json::struct_to_json_string(&self);
        return json_string;
    }
}


