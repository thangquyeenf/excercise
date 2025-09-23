// #[derive(Debug)]
trait Resource {
  fn id(&self) -> u32;
  fn title(&self) -> &str;
}

// #[derive(Debug)]
struct BorrowReceipt<'a> {
  resource: &'a dyn Resource,
  due_in_days: u32,
}


// #[derive(Debug)]
trait Borrowable<'a> {
  fn borrow(&'a self, days: u32) -> Result<BorrowReceipt<'a>, String>;
}

#[derive(Debug)]
struct Book {
  id: u32,
  title: String,
}

impl Book {
  fn new(id: u32, title: impl Into<String>) -> Self {
    Self {
      id,
      title: title.into(),
    }
  }
}

#[derive(Debug)]
struct Magazine {
  id: u32,
  title: String,
}

impl Magazine {
  fn new(id: u32, title: impl Into<String>) -> Self {
    Self {
      id,
      title: title.into(),
    }
  }
}

impl Resource for Book {
  fn id(&self) -> u32 {
    self.id
  }
  fn title(&self) -> &str {
    &self.title
  }
}

impl Resource for Magazine {
  fn id(&self) -> u32 {
    self.id
  }
  fn title(&self) -> &str {
    &self.title
  }
}

impl<'a> Borrowable<'a> for Book {
  fn borrow(&'a self, days: u32) -> Result<BorrowReceipt<'a>, String> {
    if days > 30 {
      Err(format!("Cannot borrow book '{}' for more than 30 days", self.title()))
    } else {
      Ok(BorrowReceipt {
        resource: self,
        due_in_days: days,
      })
    }
  }
}

impl<'a> Borrowable<'a> for Magazine {
  fn borrow(&'a self, days: u32) -> Result<BorrowReceipt<'a>, String> {
    if days > 7 {
      Err(format!(
        "Cannot borrow magazine '{}' for more than 7 days",
        self.title()
      ))
    } else {
      Ok(BorrowReceipt {
        resource: self,
        due_in_days: days,
      })
    }
  }
}

fn print_receipt(receipt: &BorrowReceipt) {
  println!(
    "Borrowed '{}' (ID: {}) for {} days",
    receipt.resource.title(),
    receipt.resource.id(),
    receipt.due_in_days
  );
}

fn main() {
    let book = Book::new(1, "The Rust Programming Language");
    let magazine = Magazine::new(1, "The Rustacean Weekly");
    match book.borrow(20) {
      Ok(receipt) => print_receipt(&receipt),
      Err(err) => println!("Error: {}", err),
    }
    match magazine.borrow(10) {
      Ok(receipt) => print_receipt(&receipt),
      Err(err) => println!("Error: {}", err),
    }
}
