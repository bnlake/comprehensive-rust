use std::thread;

fn main() {
    let message: String = "Hello World!".into();
    thread::scope(|scope| {
        scope.spawn(|| {
            println!("Message accessed in scoped thread: {}", message);
        });
    });

    println!("We can still use it here: {}", message);
}
