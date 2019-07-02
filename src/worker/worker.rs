
use std::sync::mpsc::{Sender, Receiver};


pub struct Worker {
   pub recv: Receiver<String>,
}


impl Worker {
    pub fn start(&self) {
        info!("start worker");
        while let Ok(n) = self.recv.recv() {
            info!("Received {}", n);
        }
    }
}
