use mini_project::{utils::log, Book, Library, Member};

fn main() {
    log("Starting the library system...");
    let mut library = Library {
        books: vec![
            Book {
                id: 1,
                title: "1984".into(),
                author: "George Orwell".into(),
                year: 1949,
                is_borrowed: false,
            },
            Book {
                id: 2,
                title: "To Kill a Mockingbird".into(),
                year: 1960,
                author: "Harper Lee".into(),
                is_borrowed: false,
            },
        ],
        members: vec![
            Member {
                id: 1,
                name: "Alice".into(),
                borrowed_books: vec![],
            },
            Member {
                id: 2,
                name: "Bob".into(),
                borrowed_books: vec![],
            },
        ],
    };
    log("Library system initialized.");
    match library.borrow_book(1, 1) {
        Ok(_) => println!("Book borrowed successfully."),
        Err(e) => println!("Error borrowing book: {}", e),
    }
    library.list_borrowed_books(1);
    match library.return_book(1, 10) {
        Ok(_) => println!("Book returned successfully."),
        Err(e) => println!("Error returning book: {}", e),
    }
    library.list_borrowed_books(1);
    match library.borrow_book(2, 1) {
        Ok(_) => println!("Book borrowed successfully."),
        Err(e) => println!("Error borrowing book: {}", e),
    }
    log("Library system operations completed.");
}
