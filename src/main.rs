#[derive(Debug)]
enum LinkedList<T> {
    Cons(T, Box<LinkedList<T>>),
    Nil,
}
impl<T> LinkedList<T> {
    fn new(t: T) -> LinkedList<T> {
        LinkedList::Cons(t, Box::new(LinkedList::Nil))
    }

    fn new_with_next(t: T, next: LinkedList<T>) -> LinkedList<T> {
        LinkedList::Cons(t, Box::new(next))
    }
}

fn main() {
    let list: LinkedList<i32> =
        LinkedList::new_with_next(24, LinkedList::new_with_next(12, LinkedList::Nil));
    println!("{list:?}")
}
