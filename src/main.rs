use std::{sync::Arc, thread};

fn main() {
    let v = Arc::new(vec![10, 20, 30]);
    let mut handles = Vec::new();

    for _ in 1..5 {
        // Arc clones are read only values
        let v = Arc::clone(&v);
        handles.push(thread::spawn(move || {
            let thread_id = thread::current().id();
            println!("{thread_id:?}: {v:?}");
        }))
    }

    handles.into_iter().for_each(|h| h.join().unwrap());
    println!("{v:?}");
}
