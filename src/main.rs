use std::fmt::Display;

#[derive(Debug)]
struct Highlight<'a>(&'a str);
impl Display for Highlight<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\"{}\"", self.0)
    }
}

fn main() {
    let phrase = String::from("Here is a really long string");
    let highlight = Highlight(&phrase[10..21]);
    println!("The highlighted text: {highlight}");
}
