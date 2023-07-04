use std::sync::mpsc;

fn main() {
    let (tx, rx) = mpsc::channel();

    tx.send(10).unwrap();
    tx.send(20).unwrap();

    println!("Channel received: {}", rx.recv().unwrap());
    println!("Channel received: {}", rx.recv().unwrap());

    let tx2 = tx.clone();
    tx2.send(100).unwrap();
    println!(
        "Same rx channel received: {} from second transmitter",
        rx.recv().unwrap()
    );
}
