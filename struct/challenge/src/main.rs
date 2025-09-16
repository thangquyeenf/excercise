
#[derive(Debug, PartialEq)]
struct Book {
  title: String,
  author: String,
  year: u32,
}

#[derive(Debug)]
struct Library {
  books: Vec<Book>,
}

impl Book {
  fn new(title: String, author: String, year: u32) -> Self {
    Self { title, author, year }
  }
}

impl Library {
  fn new() -> Self{
    Self { books: Vec::new() }
  }

  fn with_books(books: Vec<Book>) -> Self {
    Self { books }
  }

  fn add_book(&mut self, book: Book) {
    self.books.push(book);
  }

  fn list_books(&self) {
    println!("List Books:");
    for (i, book) in  self.books.iter().enumerate() {
      println!("\tNo: {}", i + 1);
      println!("\tTitle: {}", book.title);
      println!("\tAuthor: {}", book.author);
      println!("\tYear: {}", book.year);
      println!("======================")
    }
  }

  fn find_by_author(&self, author: &str) -> Vec<&Book> {
    self.books.iter().filter(|b| b.author == author).collect()
    // let mut res: Vec<&Book> = vec![];
    // for book in self.books.iter() {
    //   if book.author == author.to_string() {
    //     res.push(&book);
    //   }
    // }
    // res
  }
}

fn main() {
    let books = vec![
      Book::new(String::from("Book1"), String::from("Author1"), 2018),
      Book::new(String::from("Book2"), String::from("Author2"), 2024),
      Book::new(String::from("Book3"), String::from("Author3"), 1992),
    ];

    let mut library = Library::with_books(books);
    println!("Library: {:?}", library);
    library.add_book(
      Book::new(
        String::from("Book adder"),
        String::from("Author1"),
        2000,
      )
    );
    println!("Library: {:?}", library);
    library.list_books();
    let author = "Author1";
    let res = library.find_by_author(&author);
    println!("Res: {:?}", res);

}
