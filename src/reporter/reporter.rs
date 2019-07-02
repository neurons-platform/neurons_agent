use std::sync::mpsc::{Sender, Receiver};


pub struct Reporter {
    pub recv: Receiver<String>,
}


impl Reporter {
    pub fn start(&self) {
        info!("start reporter");
        while let Ok(n) = self.recv.recv() {
            info!("Received {}", n);
        }
    }
}
