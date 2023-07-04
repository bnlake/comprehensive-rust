use std::{thread, time::Duration};

fn main() {
    thread::spawn(|| {
        for i in 0..10 {
            println!(
                "Count in thread {}: {}",
                thread::current().name().unwrap_or("separate thread"),
                i
            );
            thread::sleep(Duration::from_secs(1));
        }
    });

    for i in 0..10 {
        println!(
            "Count in thread {}: {}",
            thread::current().name().unwrap(),
            i
        );
        thread::sleep(Duration::from_secs(1));
    }
}
