#[derive(Debug)]
pub struct Member {
    pub id: u32,
    pub name: String,
    pub borrowed_books: Vec<u32>,
}

impl Member {
    pub fn new(id: u32, name: impl Into<String>) -> Self {
        Self {
            id,
            name: name.into(),
            borrowed_books: Vec::new(),
        }
    }

    pub fn borrow_book(&mut self, book_id: &u32) {
        if !self.borrowed_books.contains(&book_id) {
            self.borrowed_books.push(*book_id);
        }
    }

    pub fn return_book(&mut self, book_id: u32) -> Result<(), String> {
        if !self.borrowed_books.contains(&book_id) {
            return Err("This book is not borrowed by the member.".to_string());
        }
        self.borrowed_books.retain(|&x| x != book_id);
        Ok(())
    }
}
