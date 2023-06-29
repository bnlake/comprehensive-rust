fn main() {
    let a: [i32; 8] = [10, 20, 30, 40, 50, 60, 70, 80];
    println!("a: {a:?}");

    // Slices borrow data from the sliced type
    let slice = &a[2..5];
    println!("slice: {slice:?}");
}
