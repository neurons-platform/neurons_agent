
extern crate serde;
extern crate serde_json;

use serde::Serialize;
use serde::Deserialize;
use serde_json::{Result, Value};


use std::sync::mpsc::{Sender};
use std::collections::HashMap;
use std::thread;
use std::time::Duration;
use super::super::utils;
use super::super::db;
use super::super::protocol::receiver_protocol::{UpdateKey,UpdateVersion};
use super::super::protocol::collecter_protocol::{UpdateCollecter};
use super::super::utils::json::{Struct};
use super::super::LOCAL_DB;
use super::super::DB;







pub struct Receiver {
  pub  channels_tx: HashMap<String,Sender<String>>,
}

fn local_db_update_key(updateKey:UpdateKey) {
    DB.set(&updateKey.key,&updateKey.value);
}

fn local_db_set_kv(key:String,value:String) {
    DB.set(&key,&value);
}

fn do_update_key(protocol:&str) {
    let p:UpdateKey = match utils::json::json_to_struct(&protocol) {
        Struct::S(p) => {
            info!("recv: {:?}",p);
            p
        },
        Struct::None => return,
    };
    local_db_update_key(p);
}

fn do_update_collecter(protocol:&str) {
    let p:UpdateCollecter = match utils::json::json_to_struct(&protocol) {
        Struct::S(p) => {
            info!("recv: {:?}",p);
            p
        },
        Struct::None => return,
    };
    info!("recv protocol: {:?}",p);
    local_db_set_kv(p.key.clone(),p.value_to_json_string().clone());
}

fn do_update_version(protocol:&str) {
    let p:UpdateVersion = match utils::json::json_to_struct(&protocol) {
        Struct::S(p) => {
            info!("recv: {:?}",p);
            p
        },
        Struct::None => return,
    };
    utils::os::download_url_as_file(&p.url,"../agent.tar.gz");
    utils::os::execvp_cmd("./update.sh","");
}

fn get_redis_url() -> String  {
    return utils::conf::conf_get("config.ini","redis","url");
}

fn get_redis_password() -> String {
    return utils::conf::conf_get("config.ini","redis","password");
}

impl Receiver {
    pub fn start(&self) {
        info!("start receiver");
        let redis = utils::redis::Redis{
            addr: get_redis_url(),
            password: get_redis_password(),
        };

        let ip = DB.get("ip");

        loop {
            thread::sleep(Duration::from_secs(1));
            let msg = redis.rpop(&ip);
            // println!("redis recv msg: {:?}",msg);
            let v: Value = match serde_json::from_str(&msg) {
                Ok(v) => v,
                Err(_) => continue,
            };
            match v["type"].as_str().unwrap() {
                "updateVersion" => {
                    let protocol = v["protocol"].to_string();
                    do_update_version(&protocol);
                },
                "restart" => {
                    utils::os::execvp_cmd("./restart.sh","");
                },
                "updateKey" => {
                    let protocol = v["protocol"].to_string();
                    do_update_key(&protocol);
                },
                "updateCollecter" => {
                    let protocol = v["protocol"].to_string();
                    // println!("recv: {:?}",protocol);
                    do_update_collecter(&protocol);
                    self.send_msg("collecter_tx","reload");
                },
                _ => continue,
            };


            // self.send_msg("worker_tx","jingminglang");
            // thread::sleep(Duration::from_secs(1));
        }
    }
    pub fn send_msg(&self,to: &str,msg: &str) -> bool {
        match  self.channels_tx.get(to) {
            Some(c) => {
                &c.send(msg.to_string());
                return true;
            },
            _ => return false,
        }
    }
}
