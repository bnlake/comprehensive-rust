use std::panic;

fn main() {
    let result = panic::catch_unwind(|| {
        println!("Hello!");
    });
    assert!(result.is_ok());

    let result = panic::catch_unwind(|| {
        panic!("Oh no!");
    });
    assert!(result.is_err());

    println!("We're still running though");
}
