#[derive(Debug, PartialEq)]
enum Status {
    Draft,
    Published,
}

#[derive(Debug)]
struct Article {
    title: String,
    content: String,
    status: Status,
}

impl Article {
    fn new(title: String, content: String, status: Status) -> Self {
        Self {
            title,
            content,
            status,
        }
    }

    fn pushlish(&mut self) {
        self.status = Status::Published;
    }

    fn is_draft(&self) -> bool {
        self.status == Status::Draft
    }
}

fn main() {
    let mut article = Article::new(
        String::from("Article"),
        String::from("Content of article"),
        Status::Draft,
    );
    println!("Is Draft?: {}", article.is_draft());
    article.pushlish();
    println!("Is Draft: {}", article.is_draft());
}
