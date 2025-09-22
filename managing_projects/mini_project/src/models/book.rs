#[derive(Debug)]
pub struct Book {
    pub id: u32,
    pub title: String,
    pub author: String,
    pub year: u32,
}

impl Book {
    pub fn new(id: u32, title:  impl Into<String>, author: impl Into<String>, year: u32) -> Self {
        Self {
            id,
            title: title.into(),
            author: author.into(),
            year,
        }
    }
}