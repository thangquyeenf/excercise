use std::io;
use thiserror::Error;

#[derive(Error, Debug)]
enum LibraryError {
    #[error("File not found")]
    FileNotFound,
    #[error("Invalid format in file")]
    InvalidFormat,
    #[error("I/O Error: {0}")]
    IoError(io::Error),
    #[error("Unexpected error occurred")]
    UnexpectedError,
}

fn load_book_from_file(path: &str) -> Result<Vec<String>, LibraryError> {
    let content = std::fs::read_to_string(path).map_err(|err| {
        if err.kind() == io::ErrorKind::NotFound {
            LibraryError::FileNotFound
        } else {
            LibraryError::IoError(err)
        }
    })?;
    let mut books: Vec<String> = Vec::new();
    for line in content.lines() {
        if line.trim().is_empty() {
            return Err(LibraryError::InvalidFormat);
        }
        books.push(line.to_string());
    }
    if books.is_empty() {
        return Err(LibraryError::UnexpectedError);
    };
    Ok(books)
}

fn get_first_book(path: &str) -> Result<String, LibraryError> {
    match load_book_from_file(path) {
        Ok(books) => Ok(books[0].clone()),
        Err(e) => {
          eprintln!("Critical error: {}. System cannot continue.", e);
          panic!("Library initialization failed: {}", e);
        },
    }
}

fn main() {
    let path = "books.txt";
    match load_book_from_file(path) {
        Ok(books) => {
            println!("Books loaded successfully:");
            for book in books {
                println!("- {}", book);
            }
        }
        Err(e) => println!("Error while loading: {}", e),
    }

    let first_book = get_first_book(path).unwrap();
    println!("First book: {}", first_book);
}
