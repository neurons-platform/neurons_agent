extern crate nix;
use super::common;
use std::process::Command;
use subprocess::Exec as exec;
use nix::unistd::execvp;
use std::ffi::CString;
use std::env;
use std::path::Path;


pub fn download_url_as_file(url:&str,file:&str) -> bool {
    let output = Command::new("wget")
        .arg(url)
        .arg("-O")
        .arg(file)
        .output();
    match output {
        Ok(_) => return true,
        Err(_) => return false,
    }
}

pub fn change_current_dir(dir:&str) -> bool {
    let root = Path::new(dir);
    env::set_current_dir(&root).is_ok()
}

pub fn execvp_cmd(cmd:&str,arg:&str) -> bool {
    let c_to_print = CString::new(cmd).expect("CString::new failed");
    let e_to_print = CString::new(arg).expect("CString::new failed");

    match execvp(&c_to_print,&[e_to_print]) {
        Ok(_) => {
            return true;
        },
        Err(err) => {
            error!("execvp error: {}",err);
            return false;
        },
    };
}


pub fn clear_disk() -> String {
    let output =  match   Command::new("./script/clean_disk.sh")
        .output() {
            Ok(output) => output.stdout,
            Err(_) => b"".to_vec(),
        };

    String::from_utf8_lossy(&output).to_string()
}

pub fn get_system_cluster() -> String {
    let output =  match   Command::new("./script/get_system_cluster.sh")
        .output() {
            Ok(output) => output.stdout,
            Err(_) => b"".to_vec(),
        };

    String::from_utf8_lossy(&output).to_string()
}

pub fn get_system_name() -> String {
    let output =  match   Command::new("./script/get_system_name.sh")
        .output() {
            Ok(output) => output.stdout,
            Err(_) => b"".to_vec(),
        };

    String::from_utf8_lossy(&output).to_string()
}

pub fn get_hostname() -> String {
    let output = match   Command::new("hostname")
        .output() {
            Ok(output) => output.stdout,
            Err(_) => b"".to_vec(),
        };

    String::from_utf8_lossy(&output).to_string()
}

pub fn get_shell_stdout_str(cmd:&str) -> String {
    let out =  exec::shell(cmd).capture();
    let  o = match out {
        Ok(o) => o,
        Err(error) => {
            panic!("exec shell faile: {:?}", error)
        },
    };
    return o.stdout_str();
}

pub fn get_all_local_ip() -> Vec<String>  {
    let out =  get_shell_stdout_str(r#"ip -o addr | awk '!/^[0-9]*: ?lo|link\/ether/ {gsub("/", " "); print $2" "$4}' | grep -o -E "[0-9]{1,3}(\.[0-9]{1,3}){3}""#);
    let string_vec = common::lines_to_string_vec(&out);
    return string_vec;
}

pub fn get_process(name:&str) -> Vec<String>  {
    let output =    Command::new("ps")
        .arg("-e")
        .arg("-o")
        .arg("command")
        .output()
        .expect("ps command failed to start");
    let process_str = String::from_utf8_lossy(&output.stdout).to_string();
    let vs = process_str.lines();
    let matchs = vs
        .filter(|x| common::is_contain(x,name))
        .collect::<Vec<_>>();

    let mut string_vec: Vec<String> = Vec::new();
    for s in &matchs {
        string_vec.push(s.to_string());
    }
    return string_vec;

}
