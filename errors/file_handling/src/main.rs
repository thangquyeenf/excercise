use std::fs::File;
use std::io::{ Read, Write };
use std::io::Error;

fn read_file(path: &str) -> Result<String, Error> {
  let mut file = File::open(path)?;
  let mut contents = String::new();
  match file.read_to_string(&mut contents) {
      Ok(_) => Ok(contents),
      Err(e) => Err(e),
  }
}

fn write_file(path: &str, data: &str) -> Result<(), Error> {
    let mut file = File::create(path)?;
    match file.write_all(data.as_bytes()) {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}

fn main() {
    let path = "example.txt";
    match read_file(path) {
        Ok(contents) => println!("File contents:\n{}", contents),
        Err(e) => println!("Error reading file: {}", e),
    }
    let write_path = "output.txt";
    let data = "Hello, world!";
    match write_file(write_path, data) {
        Ok(_) => println!("Data written to file successfully."),
        Err(e) => println!("Error writing to file: {}", e),
    }
}
