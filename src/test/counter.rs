
use std::collections::HashMap;
use std::thread;
use std::time::Duration;
use std::sync::Arc;
use std::sync::Mutex;





pub struct Counter {
    pub count: Arc<Mutex<HashMap<String,u64>>>,
    timer_thread: thread::JoinHandle<()>,
    inc_thread: thread::JoinHandle<()>,
}

impl Counter {
    pub fn new() -> Counter {
        let timer_thread = thread::spawn(|| {});
        let inc_thread = thread::spawn(|| {});
        let count = Arc::new(Mutex::new(HashMap::new()));
        Counter {
            count,
            timer_thread,
            inc_thread,
        }
    }
    pub fn start(&mut self) {
        self.inc();
        self.timer();
    }
    fn timer(&mut self) {
        let local_self = self.count.clone();
        let timer_thread = thread::spawn(move || {
            loop {
                // println!("hi number  from the spawned thread!");
                // *local_self.lock().unwrap().entry("a".to_string()).or_insert(0);
                thread::sleep(Duration::from_secs(60));
                println!("{:?}",local_self);
                local_self.lock().unwrap().clear();
            };
        });
        self.timer_thread = timer_thread;
    }
    fn inc(&mut self) {
        let local_self = self.count.clone();
        let inc_thread = thread::spawn(move || {
            loop {
                *local_self.lock().unwrap().entry("a".to_string()).or_insert(0) += 1;
                // println!("{:?}",local_self);
                thread::sleep(Duration::from_secs(1));
            };
        });
        self.inc_thread = inc_thread;
    }
    pub fn wait(self) {
        self.timer_thread.join().unwrap();
        self.inc_thread.join().unwrap();
    }
}





#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_counter() {
        let mut counter = Counter::new();
        counter.start();
        println!("start timer");
        counter.wait();
    }

}
