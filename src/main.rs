
mod test;
mod receiver;
mod collecter;
mod worker;
mod reporter;
mod utils;
mod db;
mod protocol;
use utils::time as T;
use utils::json as J;
use std::thread;
use std::sync::mpsc::channel;
use std::sync::mpsc::{Sender, Receiver};
use std::time::Duration;
use std::collections::HashMap;

use std::io::{stdin, stdout, Write};
use std::process::{Child, Command, Stdio};



extern crate job_scheduler;
use job_scheduler::{JobScheduler, Job};

extern crate clap;
use clap::{App,Arg};
static  VERSION:&str = "0.5";
static LOCAL_DB:&str = "adb";

#[macro_use]
extern crate log;
extern crate env_logger;
extern crate log4rs;

use log::LevelFilter;
use log4rs::append::file::FileAppender;
use log4rs::encode::pattern::PatternEncoder;
use log4rs::config::{Appender, Config, Root};


#[macro_use]
extern crate lazy_static;



lazy_static! {
    pub static ref DB: db::local_db::LocalDB = db::local_db::LocalDB::new(LOCAL_DB.to_string());
}

fn start_init() {
    let ip = match utils::os::get_all_local_ip().get(0) {
        Some(ip) => ip.to_string(),
        None => "".to_string(),
    };
    let system_name = utils::os::get_system_name();
    if !system_name.is_empty() {
        DB.set("system",&system_name);
    }

    let system_cluster = utils::os::get_system_cluster();
    if !system_cluster.is_empty() {
        DB.set("cluster",&system_cluster);
    }

    DB.set("ip",&ip);
    DB.set("version",&VERSION);
}


fn start_server() {
    let (worker_tx, worker_rx): (Sender<String>, Receiver<String>) = channel();
    let (reporter_tx, reporter_rx): (Sender<String>, Receiver<String>) = channel();
    let (collecter_tx, collecter_rx): (Sender<String>, Receiver<String>) = channel();


    let mut channels_tx = HashMap::new();
    channels_tx.insert("worker_tx".to_string(),worker_tx.clone());
    channels_tx.insert("reporter_tx".to_string(),reporter_tx.clone());
    channels_tx.insert("collecter_tx".to_string(),collecter_tx.clone());


    let receiver_thread = thread::spawn(move || {
        let receiver = receiver::receiver::Receiver {
            channels_tx: channels_tx.clone(),
        };
        receiver.start();
    });

    let worker_thread = thread::spawn(move || {
        let worker = worker::worker::Worker {
            recv: worker_rx,
        };
        worker.start();
    });

    let reporter_thread = thread::spawn(move || {
        let reporter = reporter::reporter::Reporter {
            recv: reporter_rx,
        };
        reporter.start();
    });

    let collecter_thread = thread::spawn(move || {
        let collecter = collecter::collecter::Collecter {
            recv: collecter_rx,
        };
        collecter.start();
    });

    let crontab_thread = thread::spawn(move || {
        let mut crontab = worker::crontab::Crontab {
            job: JobScheduler::new(),
        };

        let mut mq_sender = MqSender::default();
        mq_sender.init("agent_report".to_string(),
                       "imagent".to_string(),
                       "69F4568E".to_string());

        // 定时更新系统信息
        crontab.add_job(
            Job::new("0 1/5 * * * *".parse().unwrap(), move || {
                let system_name = utils::os::get_system_name();
                if !system_name.is_empty() {
                    DB.set("system",&system_name);
                }

                let system_cluster = utils::os::get_system_cluster();
                if !system_cluster.is_empty() {
                    DB.set("cluster",&system_cluster);
                }
            })
        );

        // 自动清理日志
        crontab.add_job(
            Job::new("0 * * * * *".parse().unwrap(), move || {
                utils::os::clear_disk();
            })
        );

        // 5分钟发一次心跳
        crontab.add_job(
            Job::new("0 1/5 * * * *".parse().unwrap(), move || {
                let ip = DB.get("ip");
                let system = DB.get("system");
                let version = DB.get("version");
                let cluster = DB.get("cluster");
                let env = DB.get("env");
                let heartbeat_protocol = protocol::reporter_protocol::Heartbeat {
                    protocol_type:"heartbeat".to_string(),
                    ip: ip.clone(),
                    version: version.clone(),
                    system: system.clone(),
                    cluster: cluster.clone(),
                    env: env.clone(),
                };
                mq_sender.send_msgs(vec![heartbeat_protocol.to_json_string()]);
                // println!("send heartbeat");
                info!("heartbeat: {:?}",heartbeat_protocol);
            })
        );
        crontab.start();
    });

    worker_thread.join().unwrap();
    reporter_thread.join().unwrap();
    receiver_thread.join().unwrap();
    collecter_thread.join().unwrap();
    crontab_thread.join().unwrap();
}

fn get_log_path() -> String  {
    return utils::conf::conf_get("config.ini","log","path");
}


fn log_init()  {
    // env_logger::init();

    let logfile = match FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{l} - {m}\n")))
        .build(get_log_path()) {
            Ok(logfile) => logfile,
            Err(err) => {
                println!("logfile err: {:?}",err);
                return
            },
        };

    let config = match Config::builder()
        .appender(Appender::builder().build("logfile", Box::new(logfile)))
        .build(Root::builder()
               .appender("logfile")
               .build(LevelFilter::Info)) {
            Ok(config) => config,
            Err(err) => {
                println!("log err: {:?}",err);
                return
            },
        };

    match log4rs::init_config(config) {
        Ok(_) => {},
        Err(err) => {
            println!("init log err: {:?}",err);
            return
        }
    };
}



fn main() {

    // env_logger::init();
    log_init();

    let matches = App::new("neurons_agent")
        .version(VERSION)
        .about("AiOps agent and log collecter")
        .author("jimila")
        .arg(Arg::with_name("db")
             .help("db mod")
             .short("d")
             .long("db"))
        .get_matches();


    if matches.is_present("db") {
        println!("start db mod");
        loop {
            print!("> ");
            stdout().flush().unwrap();
            let mut input = String::new();
            stdin().read_line(&mut input).unwrap();

            let command = input.trim();
            println!("{}",command);

        }
    }

    start_init();
    start_server();
}


