
use std::sync::mpsc::{Sender, Receiver};
use std::thread;
use std::sync::Arc;
use std::sync::Mutex;

use std::string::String;
use std::fs::OpenOptions;
use std::io::{BufReader, Read, SeekFrom, Seek};
use std::error::Error;
use notify::{RecommendedWatcher, Watcher, RecursiveMode,DebouncedEvent};
use std::sync::mpsc::channel;
use std::time::Duration;
use std::collections::HashMap;
extern crate serde;
extern crate serde_json;
use serde::Serialize;
use serde::Deserialize;


use super::super::db;
use super::super::utils;
use super::super::utils::json;
use super::tail;
use super::super::LOCAL_DB;
use super::super::DB;
use super::super::protocol::message_protocol::{Msg};
use super::super::protocol::collecter_protocol::{CollectReport};

pub enum TailCode {
    Size0,
    ReOpen,
    Unknown,
}

#[derive(Debug,Default)]
#[derive(Serialize, Deserialize,Clone)]
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

#[derive(Debug,Default)]
#[derive(Serialize, Deserialize,Clone)]
pub struct LogWatcher {
    pub path:String,
    pub watchs:Vec<WatchKey>,
}

impl LogWatcher {
    pub fn to_json_string(&self) ->String {
        let json_string = json::struct_to_json_string(&self);
        return json_string;
    }
}




pub struct Counter {
    pub count: Arc<Mutex<HashMap<String,u64>>>,
    pub path:String,
    pub wks:Vec<WatchKey>,
    pub run:Arc<Mutex<bool>>,
    pub mq_sender:MqSender,
    timer_thread: thread::JoinHandle<()>,
    tail_thread: thread::JoinHandle<()>,
    filter_thread: thread::JoinHandle<()>,
}

impl Counter {
    pub fn new(path:String,wks:Vec<WatchKey>) -> Counter {
        let timer_thread = thread::spawn(|| {});
        let tail_thread = thread::spawn(|| {});
        let filter_thread = thread::spawn(|| {});
        let count = Arc::new(Mutex::new(HashMap::new()));
        let run = Arc::new(Mutex::new(true));
        let mut mq_sender = MqSender::default();
        mq_sender.init("agent_log_collect".to_string(),
                       "imagent".to_string(),
                       "69F4568E".to_string());
        Counter {
            count,
            path,
            wks,
            run,
            mq_sender,
            timer_thread,
            tail_thread,
            filter_thread,
        }
    }
    pub fn start(&mut self) {
        // info!("start collecter");
        // self.tail_file();
        self.filter();
        self.timer();
    }

    pub fn stop(&mut self) {
        info!("stop collect");
        // self.run = Arc::new(Mutex::new(false));
        let mut run =self.run.lock().unwrap();
        *run = false;
    }

    fn timer(&mut self) {
        let local_count = self.count.clone();
        let local_sender = self.mq_sender.clone();
        let local_run = self.run.clone();
        let timer_thread = thread::spawn(move || {
            let mut run =    match  local_run.lock() {
                Ok(run) => *run,
                Err(_) => true,
            };
            while run {
                run =  match  local_run.lock() {
                    Ok(run) => *run,
                    Err(_) => true,
                };
                // info!("timer state: {}",run);
                thread::sleep(Duration::from_secs(60));
                info!("timer: {:?}",local_count);
                let ip = DB.get("ip");
                let env = DB.get("env");
                let system = DB.get("system");
                let cluster = DB.get("cluster");
                let time = utils::time::time_stamp_ms();
                // for (k, v) in local_count.lock().iter() {
                let mut count = match local_count.lock() {
                    Ok(count) => count,
                    Err(err) => {
                        error!("timer get count lock fail:{}",err);
                        continue;
                    },
                };
                // for kv in local_count.lock().unwrap().iter() {
                for kv in count.iter() {
                    let (k ,v ) = kv;
                    // println!("kv: {:?}",kv);
                    let collecter_report = CollectReport {
                        time:time,
                        count:v.clone(),
                        system:system.clone(),
                        env:env.clone(),
                        cluster:cluster.clone(),
                        ip:ip.clone(),
                        count_type:k.to_string(),
                    };
                    info!("send msg: {:?}",collecter_report.to_json_string());
                    local_sender.send_msgs(vec![collecter_report.to_json_string()]);
                }
                // local_count.lock().unwrap().clear();
                count.clear();
                // count.clear();
            };
        });
        self.timer_thread = timer_thread;
    }

    pub fn wait(self) {
        self.timer_thread.join().unwrap();
        self.filter_thread.join().unwrap();
        // self.tail_thread.join().expect("tail quit");
    }

    fn filter(&mut self) {
        let local_count = self.count.clone();
        let local_path = self.path.clone();
        let local_wks = self.wks.clone();
        let local_run = self.run.clone();
        let filter_thread = thread::spawn(move || {
            // 10m
            // let mut buffer = vec![0; 10240000].into_boxed_slice();
            // 5m
            // let mut buffer = vec![0; 5120000].into_boxed_slice();
            // 1m
            let mut run = match  local_run.lock() {
                Ok(run) => *run,
                Err(_) => true,
            };
            while run {
                // let mut buffer = Box::new([0u8; 10240000]);

                // info!("filter read");
                let mut buffer = vec![0; 1024000].into_boxed_slice();
                run =  match  local_run.lock() {
                    Ok(run) => *run,
                    Err(_) => true,
                };
                // info!("collect state: {}",run);
                let file = match OpenOptions::new().read(true).open(&local_path) {
                    Err (_) => {
                        error!("read file error");
                        thread::sleep(Duration::from_secs(1));
                        continue;
                    },
                    Ok(file) => file
                };
                let f_metadata = match file.metadata() {
                    Err(_) => {
                        error!("metadata error");
                        thread::sleep(Duration::from_secs(1));
                        continue;
                    },
                    Ok(data) => data
                };
                let f_size = f_metadata.len();
                if f_size == 0 {
                    // error!("size 0 error");
                    thread::sleep(Duration::from_secs(1));
                    continue;
                }
                let mut reader = BufReader::new(file);
                let start_byte = f_size;

                // info!("start byte: {}",start_byte);

                match reader.seek(SeekFrom::Start(start_byte)){
                    Err(err) => {
                        error!("reader seek error:{}",err);
                        thread::sleep(Duration::from_secs(1));
                        continue;
                    },
                    Ok(_) => {},
                };

                thread::sleep(Duration::from_secs(1));
                // 10m
                // let read_byte = match reader.read_exact(&mut buffer) {
                match reader.read(&mut *buffer) {
                    Err(err) => {
                        error!("read byte error:{}",err);
                        continue;
                    },
                    Ok(b) => {
                        if b == 0 {
                            // error!("read byte zero");
                            continue ;
                        }
                        b
                    },
                };

                for wk in local_wks.iter() {
                    // for line in std::str::from_utf8(&*buffer).unwrap().to_string().lines() {
                    for line in String::from_utf8_lossy(&*buffer).lines() {
                        if wk.is_match(line) {
                            // *local_count.lock().unwrap().entry(wk.msg.to_string()).or_insert(0) += 1;
                            let mut count = match local_count.lock() {
                                Ok(count) => count,
                                Err(err) => {
                                    error!("filter get count lock fail {}",err);
                                    continue;
                                },
                            };
                            *count.entry(wk.msg.to_string()).or_insert(0) += 1;
                        }

                    }
                }


            };
        });
        self.filter_thread = filter_thread;
    }

    fn tail_file(&mut self)  {
        let local_count = self.count.clone();
        let local_path = self.path.clone();
        let local_wks = self.wks.clone();

        let tail_thread = thread::spawn(move || {
            're_tail: loop {
                let path = local_path.clone();
                thread::sleep(Duration::from_secs(1));
                info!("start tail");
                let file = match OpenOptions::new().read(true).open(&path) {
                    Err (_) => {
                        error!("read file error");
                        continue;
                    },
                    Ok(file) => file
                };
                let f_metadata = match file.metadata() {
                    Err(_) => {
                        error!("metadata error");
                        continue;
                    },
                    Ok(data) => data
                };
                let f_size = f_metadata.len();
                if f_size == 0 {
                    error!("size 0 error");
                    continue;
                }
                let mut reader = BufReader::new(file);


                let (tx, rx) = channel();
                let mut watcher: RecommendedWatcher = match Watcher::new(tx,Duration::from_secs(1)) {
                    Ok(w) => w,
                    Err(_) => {
                        error!("new watcher error");
                        continue;
                    },
                };
                match watcher.watch(&path, RecursiveMode::NonRecursive) {
                    Ok(_) => {},
                    Err(_) =>  {
                        error!("watch error");
                        continue;
                    },
                };
                let mut start_byte = f_size;
                let mut buf_str = String::new();

                loop {
                    match rx.recv() {
                        Err(_) => {
                            error!("recv event");
                            break 're_tail;
                        },
                        Ok(msg) => {
                            info!("event: {:?}",msg);
                            match msg {
                                DebouncedEvent::Write(_) => {},
                                DebouncedEvent::NoticeWrite(_) => continue,
                                _ => {
                                    // println!("msg event");
                                    // break 're_tail;
                                    continue 're_tail;
                                },
                            }
                            match reader.seek(SeekFrom::Start(start_byte)){
                                Err(_) => {
                                    // println!("seek error");
                                    // break 're_tail;
                                    continue 're_tail;
                                },
                                Ok(_) => {
                                    // start_byte,
                                    // println!("x is {:?}",x);
                                },
                            };
                            // reader.take
                            let mut buffer = [0; 2048];
                            // let read_byte = match reader.read_to_string(&mut buf_str){
                                let read_byte = match reader.read(&mut buffer) {
                                Err(_) => {
                                    // println!("read error");
                                    // break 're_tail;
                                    continue 're_tail;
                                },
                                Ok(b) => {
                                    // println!("b is {:?}",b);
                                    if b == 0 {
                                        // println!("b is zero");
                                        // break 're_tail;
                                        continue 're_tail;
                                    }
                                    b
                                },
                            };
                            start_byte += read_byte as u64;

                            for wk in local_wks.iter() {
                                // for line in buf_str.lines() {
                                for line in std::str::from_utf8(&buffer).unwrap().to_string().lines() {
                                    if wk.is_match(line) {
                                        // println!("matchd");
                                        *local_count.lock().unwrap().entry(wk.msg.to_string()).or_insert(0) += 1;
                                        // *local_count.lock().unwrap().entry(wk.watch_key.to_string()).or_insert(0) += 1;
                                        // println!("{:?}",counter);
                                    }

                                }
                            }
                            // buf_str.clear();
                            // buf_str = String::new();
                            // buffer = [0; 5];
                        }
                    }
                }
            }
        });
        self.tail_thread = tail_thread;
    }

}


pub struct Collecter {
    pub recv: Receiver<String>,
}


impl Collecter {
    pub fn start(&self) {
        // let (tx, rx) = channel();
        // let run = Arc::new(Mutex::new(true));
        // thread::spawn(move || {
        //     loop {
        //         let msg = rx.recv().unwrap();
        //         let run = Arc::clone(&run);
        //         match msg {
        //             Msg::Start(msg) => {
        //                 *run.lock().unwrap() = true;
        //                 while *run.lock().unwrap() {
        //                 };
        //             },
        //             Msg::Stop(msg) => {
        //                 *run.lock().unwrap() = false;
        //             },
        //         };
        //     };
        // });


        're_start:     loop {
            info!("start collecter");
            let log_watchers = load_log_watchers();
            // info!("log watchers: {:?}",log_watchers);
            let mut collecters = vec![];
            for w in log_watchers.clone() {
                let mut  counter = Counter::new(w.path,w.watchs);
                counter.start();
                collecters.push(counter);
            }

            // for counter in collecters {
            //     counter.wait();
            // }


            while let Ok(n) = self.recv.recv() {
                match &n[..] {
                    "reload" => {
                        for mut counter in collecters {
                            counter.stop();
                        };
                        continue 're_start;
                    },
                    _ => {
                        println!("Received {}", n);
                    },
                }
            }
            // for w in log_watchers.clone() {
            //     collecters.push(thread::spawn(move || {
            //         let mut  counter = Counter::new(w.path,w.watchs);
            //         counter.start();
            //         counter.wait();
            //     }));
            // }
            // for child in collecters {
            //     let _ = child.join();
            // }
        }

        // let log_watchers = tail::load_log_watchers();
        // let mut collecters = vec![];
        // for w in log_watchers.clone() {
        //     collecters.push(thread::spawn(move || {
        //         tail::tail_file(&w.path,w.watchs);
        //     }));
        // }
        // while let Ok(n) = self.recv.recv() {
        //     println!("Received {}", n);
        // }
        // for child in collecters {
        //     let _ = child.join();
        // }

    }
}

pub fn load_log_watchers() -> Vec<LogWatcher> {
    let mut log_wather_vec: Vec<LogWatcher> = Vec::new();
    // let c = utils::file::read_all_from_file("/home/jimila/Desktop/project/im/neurons_agent/test.txt");


    let c = DB.get("collecter");
    info!("collecter: {:?}",c);

    log_wather_vec = match  serde_json::from_str(&c) {
        Ok(v) => v,
        Err(_) => return log_wather_vec,
    };
    return log_wather_vec;
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_start_tail_counter() {
        println!("start collecter");
        let log_watchers = load_log_watchers();
        let mut collecters = vec![];
        for w in log_watchers.clone() {
            collecters.push(thread::spawn(move || {
                // tail::tail_file(&w.path,w.watchs);
                let mut  counter = Counter::new(w.path,w.watchs);
                counter.start();
                counter.wait();
            }));
        }
        for child in collecters {
            let _ = child.join();
        }
    }
}
