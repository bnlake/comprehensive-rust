use std::{
    sync::{Arc, Mutex},
    thread::{self, JoinHandle},
    time::Duration,
};

fn spawn_thread(v: &Arc<Mutex<Vec<i32>>>, delay: i32) -> JoinHandle<()> {
    let v = Arc::clone(v);
    thread::spawn(move || {
        thread::sleep(Duration::from_millis(delay as u64));
        let mut guard = v.lock().unwrap();
        guard.push(delay);
    })
}

fn main() {
    let m = Arc::new(Mutex::new(vec![10, 20, 30]));
    let mut handles = Vec::new();

    for i in 1..5 {
        handles.push(spawn_thread(&m, i * 200));
    }

    handles.into_iter().for_each(|h| h.join().unwrap());
    println!("m: {:?}", m.lock().unwrap());
}
