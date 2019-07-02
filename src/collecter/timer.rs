
extern crate timer;
extern crate chrono;

use timer::Timer;
use chrono::Duration;
use std::thread;

fn x() {
    println!("hello");
}

fn start() {
    let timer = Timer::new();
    let mut f = 1;
    // let f = RefCell::new(0);

    // let guard = timer.schedule_repeating(Duration::seconds(1), x);
    f = f + 1;
    let guard = timer.schedule_repeating(Duration::seconds(1), move || {
        println!("{}",f);
    });
    f = f + 1;
    println!("start: {}",f);

    // give some time so we can see hello printed
    // you can execute any code here
    thread::sleep(::std::time::Duration::new(10, 0));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_start_timer() {
        start();
    }

}

