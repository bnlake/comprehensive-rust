fn main() {
    // &str is a reference to a string slice
    // Writing literal strings returns a reference to a string slice
    let s1: &str = "World";
    println!("String 1: {s1}");

    // String is a string "buffer"
    // The String type is a value stored on the stack
    // It maintains 3 parts of information about the string:
    //      1. A pointer to some bytes
    //      2. A length
    //      3. A capacity
    let mut s2 = String::from("Hello");
    println!("String 2: {s2}");
    s2.push_str(s1);
    println!("String 2: {s2}");

    // A slice can be taken of a slice
    let s3: &str = &s1[0..2];
    println!("String 3: {s3}");
    
    // A slice can be taken from a string buffer
    let s4: &str = &s2[5..];
    println!("String 4: {s4}");
}
