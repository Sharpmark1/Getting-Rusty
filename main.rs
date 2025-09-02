struct Book {
    title: String,
    author: String,
    pages: u32,
}

impl Book {
    fn new(title: &str, author: &str, pages: u32) -> Book {
        Book {
            title: title.to_string(),
            author: author.to_string(),
            pages,
        }
    }

    fn summary(&self) -> String {
        // return "Title by Author, X pages"
        format!("{} by {}, {} pages", self.title, self.author, self.pages)
    }
}

struct Library {
    books: Vec<Book>,
}

impl Library {
    fn new() -> Library {
        // create an empty library
        Library {
            books: Vec::new(),
        }
    }

    fn add_book(&mut self, book: Book) {
        // Add book into books vector
        self.books.push(book);
    }

    fn print_books(&self) {
        // loop through books and print summary
        for book in &self.books {
            println!("{}", book.summary());
        }
    }
}

fn main() {
    // create some books
    let book1 = Book::new("Hello Rust", "Drainer wills", 254);
    let book2 = Book::new("Back Alley Rust Your Backend Support", "Dark Rayleigh", 380);
    let book3 = Book::new("Everything Rust Leveling up to Rusty", "Sharpmark Red", 180);

    // create library
    let mut library = Library::new();

    // Add books
    library.add_book(book1);
    library.add_book(book2);
    library.add_book(book3);

    // TODO: print all books
    library.print_books();
}