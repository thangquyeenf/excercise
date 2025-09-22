use mini_project::{ Book, Library, Member, utils::log };

fn main() {
    log("Starting the library system...");
    let mut library = Library {
        books: vec![
            Book { id: 1, title: "1984".into(), author: "George Orwell".into(), year: 1949 },
            Book { id: 2, title: "To Kill a Mockingbird".into(), year: 1960, author: "Harper Lee".into() },
        ],
        members: vec![
            Member { id: 1, name: "Alice".into(), borrowed_books: vec![] },
            Member { id: 2, name: "Bob".into(), borrowed_books: vec![] },
        ],
    };
    log("Library system initialized.");
    library.borrow_book(1, 1);
    library.list_borrowed_books(1);
    library.return_book(1, 1);
    library.list_borrowed_books(1);
    log("Library system operations completed.");
}
