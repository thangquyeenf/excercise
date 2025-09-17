#[derive(Debug, PartialEq)]
struct Book {
    id: u32,
    title: String,
    author: String,
    year: u32,
}

#[derive(Debug)]
struct Member {
    id: u32,
    name: String,
    borrowed_books: Vec<u32>,
}

#[derive(Debug)]
struct Library {
    books: Vec<Book>,
    members: Vec<Member>,
}

impl Book {
    fn new(id: u32, title: impl Into<String>, author: impl Into<String>, year: u32) -> Self {
        Self {
            id,
            title: title.into(),
            author: author.into(),
            year,
        }
    }
}

impl Member {
    fn new(id: u32, name: impl Into<String>) -> Self {
        Self {
            id,
            name: name.into(),
            borrowed_books: Vec::new(),
        }
    }

    fn borrow_book(&mut self, book_id: &u32) {
        if !self.borrowed_books.contains(&book_id) {
            self.borrowed_books.push(*book_id);
        }
    }

    fn return_book(&mut self, book_id: u32) {
        self.borrowed_books.retain(|&x| x != book_id)
    }
}

impl Library {
    fn new() -> Self {
        Self {
            books: Vec::new(),
            members: Vec::new(),
        }
    }

    fn with_books(books: Vec<Book>) -> Self {
        Self {
            books,
            members: Vec::new(),
        }
    }

    fn add_book(&mut self, book: Book) {
        self.books.push(book);
    }

    fn list_books(&self) {
        println!("List Books:");
        for (i, book) in self.books.iter().enumerate() {
            println!(
                "\tNo: {}\n\tTitle: {}\n\tAuthor: {}\n\tYear: {}\n------------------------",
                i + 1,
                book.title,
                book.author,
                book.year
            );
        }
    }

    fn find_by_author(&self, author: &str) -> Vec<&Book> {
        self.books.iter().filter(|b| b.author == author).collect()
    }

    fn remove_book(&mut self, id: u32) {
        self.books.retain(|b| b.id != id)
    }

    fn update_book(
        &mut self,
        id: u32,
        title: Option<String>,
        author: Option<String>,
        year: Option<u32>,
    ) -> bool {
        if let Some(book) = self.books.iter_mut().find(|x| x.id == id) {
            if let Some(t) = title {
                book.title = t;
            }
            if let Some(a) = author {
                book.author = a;
            }
            if let Some(y) = year {
                book.year = y;
            }
            true
        } else {
            false
        }
    }

    fn count_by_author(&self, author: &str) -> usize {
        self.books.iter().filter(|x| x.author == author).count()
    }

    fn is_contain_book(&self, book_id: u32) -> bool {
        self.books.iter().any(|b| b.id == book_id)
    }

    fn find_book(&self, book_id: u32) -> Option<&Book> {
        self.books.iter().find(|x| x.id == book_id)
    }
}

impl Library {
    fn add_member(&mut self, member: Member) {
        if !self.is_member(&member.id) {
            self.members.push(member)
        } else {
            println!("Member with id: {} already exist!", member.id)
        }
    }

    fn is_member(&self, member_id: &u32) -> bool {
        self.members.iter().any(|m| m.id == *member_id)
    }

    fn borrow_book(&mut self, member_id: u32, book_id: u32) -> bool {
        if !self.is_contain_book(book_id) {
            println!("Book not found!");
            return false;
        }
        if let Some(member) = self.members.iter_mut().find(|x| x.id == member_id) {
            member.borrow_book(&book_id);
            true
        } else {
            println!("User is not a member of the library!");
            false
        }
    }

    fn return_book(&mut self, member_id: u32, book_id: u32) -> bool {
        if !self.is_contain_book(book_id) {
            println!("Book not found!");
            return false;
        }
        if let Some(member) = self.members.iter_mut().find(|x| x.id == member_id) {
            member.return_book(book_id);
            true
        } else {
            println!("User is not a member of the library!");
            false
        }
    }

    fn find_member(&self, member_id: u32) -> Option<&Member> {
        self.members.iter().find(|&x| x.id == member_id)
    }

    fn list_borrowed_books(&self, member_id: u32) {
        if let Some(member) = self.find_member(member_id) {
            println!("Borrowed books by {}:", member.name);
            for book_id in &member.borrowed_books {
                if let Some(book) = self.find_book(*book_id) {
                    println!(
                        "\tTitle: {}\n\tAuthor: {}\n\tYear: {}\n------------------------",
                        book.title, book.author, book.year
                    );
                }
            }
        } else {
            println!("User is not a member of the library!");
            return;
        }
    }

    fn return_list_borrowed_books(&self, member_id: u32) -> Option<Vec<&Book>> {
        if let Some(member) = self.find_member(member_id) {
            Some(
                self.books
                    .iter()
                    .filter(|x| member.borrowed_books.contains(&x.id))
                    .collect(),
            )
        } else {
            println!("User is not a member of the library!");
            return None;
        }
    }

    fn find_books_before_year(&self, year: u32) -> Vec<&Book> {
        self.books.iter().filter(|x| x.year < year).collect()
    }
}

fn main() {
    let books = vec![
        Book::new(1, "Book1", "Author1", 2018),
        Book::new(2, "Book2", "Author2", 2024),
        Book::new(3, "Book3", "Author3", 1992),
    ];

    let mut library = Library::with_books(books);

    library.add_book(Book::new(4, "Book Added", "Author1", 2000));
    println!("📚 Initial library: {:?}", library.books);

    // --- CRUD Example ---
    println!(
        "\n🔍 Find books by Author1: {:?}",
        library.find_by_author("Author1")
    );
    println!(
        "📊 Count books by Author1: {}",
        library.count_by_author("Author1")
    );

    library.update_book(1, Some("Updated Book1".to_string()), None, Some(2020));
    println!("\n✏️ After update: {:?}", library.find_book(1));

    library.remove_book(2);
    println!("📚 After removal: {:?}", library.books);

    // --- Member & Borrow Example ---
    let mut member = Member::new(1, "Alice");
    library.add_member(member);

    library.borrow_book(1, 1);
    library.borrow_book(1, 4);

    library.list_borrowed_books(1);

    library.return_book(1, 1);
    println!("\n📦 After returning book 1:");
    library.list_borrowed_books(1);

    // --- Advanced Query ---
    println!("\n📚 Books before year 2000:");
    for book in library.find_books_before_year(2000) {
        println!("{:?}", book);
    }
}
