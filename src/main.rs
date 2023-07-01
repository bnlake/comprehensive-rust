#[derive(Debug)]

struct Point(i32, i32);

#[derive(Debug, Clone, Copy)]
struct CopyPoint(i32, i32);

fn main() {
    let p1 = Point(2, 5);
    let p2 = p1;
    // The following line would throw a compile time error
    // This is because p1 is not a native type that implements the Copy trait
    // therefore it is moved into p2
    // println!("{p1:?}")

    let p3 = CopyPoint(2, 5);
    let p4 = p3;
    // Now it's ok because the value is implicitly copied into p4
    println!("{p4:?}")
}
