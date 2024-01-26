use std::{thread::sleep, time::Duration};

pub fn exit() -> ! {
    println!("See you soon! 😁");
    sleep(Duration::from_secs(4));
    std::process::exit(0)
}
