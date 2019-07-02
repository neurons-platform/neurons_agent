
extern crate getopts;
extern crate notify;
extern crate serde;
extern crate serde_json;
use serde::Serialize;
use serde::Deserialize;
use serde_json::Result;



use std::string::String;
use std::fs::OpenOptions;
use std::io::{BufReader, Read, SeekFrom, Seek};
use std::error::Error;
use notify::{RecommendedWatcher, Watcher, RecursiveMode,DebouncedEvent};
use std::sync::mpsc::channel;
use std::time::Duration;
use std::collections::HashMap;


use super::super::utils;




pub enum TailCode {
    Size0,
    ReOpen,
    Unknown,
}

#[derive(Serialize, Deserialize, Debug,Clone)]
pub struct LogWatcher {
    pub path:String,
    pub watchs:Vec<WatchKey>,
}

#[derive(Serialize, Deserialize, Debug,Clone)]
pub struct WatchKey {
    pub watch_key: String,
    pub msg: String,
}

impl WatchKey {
    fn is_match(&self ,content: &str) -> bool {
        if content.contains(&self.watch_key) {
            return true;
        }
        return false;
    }
}

pub struct LogReport {
    time: u64,
    count: u32,
    system: String,
    ip: String,
    msg: String,
}


pub fn load_log_watchers() -> Vec<LogWatcher> {
    let mut log_wather_vec: Vec<LogWatcher> = Vec::new();
    let c = utils::file::read_all_from_file("/home/jimila/Desktop/project/im/neurons_agent/test.txt");
    log_wather_vec = match  serde_json::from_str(&c) {
        Ok(v) => v,
        Err(_) => return log_wather_vec,
    };
    return log_wather_vec;
}




pub fn tail_file(path: &String,wks:Vec<WatchKey>) -> TailCode {
    let mut counter:HashMap<String,u64> = HashMap::new();
    // *counter.entry("a".to_string()).or_insert(0) += 1;

    let file = match OpenOptions::new().read(true).open(path) {
        Err (_) => {
            return TailCode::Unknown;
        },
        Ok(file) => file
    };
    let f_metadata = match file.metadata(){
        Err(_) => {
            return TailCode::Unknown;
        },
        Ok(data) => data
    };
    let f_size = f_metadata.len();
    if f_size == 0 {
        return TailCode::Size0;
    }
    let mut reader = BufReader::new(file);


    let (tx, rx) = channel();
    let mut watcher: RecommendedWatcher = match Watcher::new(tx,Duration::from_secs(1)) {
        Ok(w) => w,
        Err(err) => {panic!(err)}
    };
    match watcher.watch(path, RecursiveMode::NonRecursive) {
        Ok(_) => {},
        Err(err) => {panic!(err)}
    };
    let mut start_byte = f_size;
    let mut buf_str = String::new();


    loop {
        match rx.recv() {
            Err(e) => println!("watch error: {:?}", e),
            Ok(msg) => {
                match msg {
                    DebouncedEvent::Write(_) => {},
                    DebouncedEvent::NoticeWrite(_) => {},
                    _ => {return TailCode::ReOpen},
                }
                // println!("msg : {:?}",msg);
                match reader.seek(SeekFrom::Start(start_byte)){
                    Err(why) => panic!("Cannot move offset! offset:{} cause:{}", start_byte, Error::description(&why)),
                    Ok(_) => start_byte
                };
                let read_byte = match reader.read_to_string(&mut buf_str){
                    Err(why) => panic!("Cannot read offset byte! offset:{} cause:{}",start_byte, Error::description(&why)),
                    Ok(b) => b
                };
                start_byte += read_byte as u64;

                // counter.clear();

                for wk in wks.iter() {
                    handle_line(buf_str.clone(),wk,&mut counter);
                }
                buf_str.clear();
            }
        }
    }
}


fn handle_line(disp_str: String ,wk: &WatchKey,counter:&mut HashMap<String,u64>){
    for line in disp_str.lines() {
        let matchd = wk.is_match(line);
        if matchd {
            *counter.entry(wk.watch_key.to_string()).or_insert(0) += 1;
            println!("{:?}",counter);
        }

    }
}

