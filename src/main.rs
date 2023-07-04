use std::{sync::mpsc, thread, time::Duration};

fn main() {
    let (tx, rx) = mpsc::sync_channel(0);

    thread::spawn(move || {
        let thread_id = thread::current().id();
        for i in 0..10 {
            tx.send(i).unwrap();
            println!("Thread {:?} sent message {}", thread_id, i);
        }
    });
    for message in rx.iter() {
        println!("Received message {} from a publisher", message);
        thread::sleep(Duration::from_secs(1));
    }
}
