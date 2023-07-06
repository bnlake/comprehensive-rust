use std::sync::Mutex;

fn main() {
    let m = Mutex::new(vec![10, 20, 30]);
    println!("m: {:?}", m.lock().unwrap());
    {
        let mut guard = m.lock().unwrap();
        guard.push(40);
    }
    println!("m: {:?}", m.lock().unwrap());
}
