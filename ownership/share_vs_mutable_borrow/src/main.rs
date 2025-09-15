struct Document {
    content: String,
}

impl Document {
    fn new(content: String) -> Self {
      Self { content }
    }

    fn read_content(&self) -> &str {
      &self.content
    }

    fn append_content(&mut self, val: &str) -> &mut Self {
      self.content.push_str(val);
      self
    }

    fn append(mut self, val: &str) -> Self {
      self.content.push_str(val);
      self
    }
}

fn main() {
    let mut doc = Document::new(String::from("Document -s"));
    println!("Doc content: {}", doc.read_content());
    doc.append_content(" shhsdk");
    println!("Doc content: {}", doc.read_content());

    let docs = Document::new(String::from("Hello"))
      .append(" rUST");

    println!("Doc content: {}", docs.read_content());

}



