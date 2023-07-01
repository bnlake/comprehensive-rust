struct Book {
    title: String,
    year: u16,
}
impl Book {
    fn new(title: &str, year: u16) -> Self {
        Book {
            title: String::from(title),
            year,
        }
    }
}

struct Library {
    books: Vec<Book>,
}
impl Library {
    fn new() -> Self {
        Library { books: vec![] }
    }

    fn len(&self) -> usize {
        self.books.len()
    }

    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    fn add_book(&mut self, book: Book) {
        self.books.push(book);
    }

    fn print_books(&self) {
        for (i, book) in self.books.iter().enumerate() {
            println!("{}: {} ({})", i, book.title, book.year);
        }
    }

    fn oldest_book(&self) -> Option<&Book> {
        if self.is_empty() {
            None
        } else {
            Some(
                self.books
                    .iter()
                    .reduce(|acc, b| if acc.year <= b.year { acc } else { b })
                    .unwrap(),
            )
        }
    }
}

fn main() {
    let mut library = Library::new();

    println!(
        "The library is empty: library.is_empty() -> {}",
        library.is_empty()
    );

    library.add_book(Book::new("Lord of the Rings", 1954));
    library.add_book(Book::new("Alice's Adventures in Wonderland", 1865));

    println!(
        "The library is no longer empty: library.is_empty() -> {}",
        library.is_empty()
    );

    library.print_books();

    match library.oldest_book() {
        Some(book) => println!("The oldest book is {}", book.title),
        None => println!("The library is empty!"),
    }

    println!("The library has {} books", library.len());
    library.print_books();
}
