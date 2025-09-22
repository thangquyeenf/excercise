use crate::models::{Book, Member};

#[derive(Debug)]
pub struct Library {
    pub books: Vec<Book>,
    pub members: Vec<Member>,
}

impl Library {
    pub fn new() -> Self {
        Self {
            books: Vec::new(),
            members: Vec::new(),
        }
    }

    pub fn with_books(books: Vec<Book>) -> Self {
        Self {
            books,
            members: Vec::new(),
        }
    }

    pub fn add_book(&mut self, book: Book) {
        self.books.push(book);
    }

    pub fn list_books(&self) {
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

    pub fn find_by_author(&self, author: &str) -> Vec<&Book> {
        self.books.iter().filter(|b| b.author == author).collect()
    }

    pub fn remove_book(&mut self, id: u32) {
        self.books.retain(|b| b.id != id)
    }

    pub fn update_book(
        &mut self,
        id: u32,
        title: Option<String>,
        author: Option<String>,
        year: Option<u32>,
        is_borrowed: Option<bool>,
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
            if let Some(ib) = is_borrowed {
                book.is_borrowed = ib;
            }
            true
        } else {
            false
        }
    }

    pub fn count_by_author(&self, author: &str) -> usize {
        self.books.iter().filter(|x| x.author == author).count()
    }

    pub fn is_contain_book(&self, book_id: u32) -> bool {
        self.books.iter().any(|b| b.id == book_id)
    }

    pub fn find_book(&self, book_id: u32) -> Option<&Book> {
        self.books.iter().find(|x| x.id == book_id)
    }

    fn is_borrowed(&self, book_id: u32) -> bool {
        self.members
            .iter()
            .any(|m| m.borrowed_books.contains(&book_id))
    }

    // --- Member methods ---
    pub fn add_member(&mut self, member: Member) {
        if !self.is_member(&member.id) {
            self.members.push(member)
        } else {
            println!("Member with id: {} already exist!", member.id)
        }
    }

    pub fn is_member(&self, member_id: &u32) -> bool {
        self.members.iter().any(|m| m.id == *member_id)
    }

    pub fn borrow_book(&mut self, member_id: u32, book_id: u32) -> Result<(), String> {
        if !self.is_contain_book(book_id) {
            return Err("Book not found!".to_string());
        }

        if self.is_borrowed(book_id) {
            return Err("Book is already borrowed!".to_string());
        }

        if let Some(member) = self.members.iter_mut().find(|x| x.id == member_id) {
            member.borrow_book(&book_id);
            self.update_book(book_id, None, None, None, Some(true));
            Ok(())
        } else {
            return Err("User is not a member of the library!".to_string());
        }
    }

    pub fn return_book(&mut self, member_id: u32, book_id: u32) -> Result<(), String> {
        if !self.is_contain_book(book_id) {
            return Err("Book not found!".to_string());
        }
        if let Some(member) = self.members.iter_mut().find(|x| x.id == member_id) {
            match member.return_book(book_id) {
                Ok(_) => {
                    self.update_book(book_id, None, None, None, Some(false));
                    Ok(())
                }
                Err(e) => return Err(e),
            }
        } else {
            return Err("User is not a member of the library!".to_string());
        }
    }

    pub fn find_member(&self, member_id: u32) -> Option<&Member> {
        self.members.iter().find(|&x| x.id == member_id)
    }

    pub fn list_borrowed_books(&self, member_id: u32) {
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
        }
    }

    pub fn return_list_borrowed_books(&self, member_id: u32) -> Option<Vec<&Book>> {
        if let Some(member) = self.find_member(member_id) {
            Some(
                self.books
                    .iter()
                    .filter(|x| member.borrowed_books.contains(&x.id))
                    .collect(),
            )
        } else {
            println!("User is not a member of the library!");
            None
        }
    }

    pub fn find_books_before_year(&self, year: u32) -> Vec<&Book> {
        self.books.iter().filter(|x| x.year < year).collect()
    }
}
