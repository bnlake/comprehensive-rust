#[derive(Debug)]
struct Point(i32, i32);

/// I believe variables that share the same lifetime specifier
/// is the same as telling the compiler that variable a, variable b
/// and the result of this function must all live at least the same
/// length at application runtime
fn left_most<'a>(a: &'a Point, b: &'a Point) -> &'a Point {
    if a.0 < b.0 {
        a
    } else {
        b
    }
}

fn main() {
    let a = Point(12, 53);
    let b = Point(82, 14);
    let left_point = left_most(&a, &b);
    println!("The lest most point is {left_point:?}");
}
