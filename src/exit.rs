use std::{thread, time::Duration};

pub fn exit() -> ! {
    println!("See you soon! 😁");
    thread::sleep(Duration::from_secs(4));
    std::process::exit(0)
}
