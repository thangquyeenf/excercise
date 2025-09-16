#[derive(Debug)]
struct Book {
    title: String,
    author: String,
    published: Option<u32>,
}

impl Book {
    fn new(title: String, author: String, published: Option<u32>) -> Self {
        Self {
            title,
            author,
            published,
        }
    }

    fn print_info(&self) {
        println!("Info:");
        println!("Title: {}", self.title);
        println!("Author: {}", self.author);
        if let Some(p) = self.published {
            println!("Published: {}", p);
        } else {
            println!("Unpublished");
        }
    }
}

fn main() {
    let book = Book::new(
        String::from("Rust book"),
        String::from("Rust Foudation"),
        Some(2018),
    );
    book.print_info();
}
