
#[derive(Debug, Clone)]
struct Book<'a> {
  title: &'a str,
  author: &'a str,
}

impl<'a> Book<'a > {
  fn new(title: &'a str, author: &'a str) -> Self {
    Self { title, author }
  }
}

#[derive(Debug)]
struct OwnedBook {
  title: String, 
  author: String,
}

impl OwnedBook {
  fn new(title: String, author: String) -> Self {
    Self { title, author }
  }
}

fn main() {
  let book_ref;
    {
      let title = "MyBook";
      // let author = String::from("author");
      let author = "Author";
      let book = Book::new(title, &author);
      println!("{:?}", book);
      book_ref = book;
    }
    println!("{:?}", book_ref);

    let book;
    {
      let title1 = String::from("Owned Book");
      let author1 = String::from("Ref");
      book = OwnedBook::new(title1, author1);
    }
    println!("{:?}", book);
}
