
trait Summary {
    fn summarize(&self) -> String;
}

#[derive(Debug)]
struct Book {
    title: String,
    author: String,
}

impl Summary for Book {
    fn summarize(&self) -> String {
        format!("{} by {}", self.title, self.author)
    }
}

#[derive(Debug)]
struct Movie {
    title: String,
    director: String,
}

impl Summary for Movie {
    fn summarize(&self) -> String {
        format!("{} directed by {}", self.title, self.director)
    }
}

fn print_summary<T: Summary>(item: &T) {
    println!("Summary: {}", item.summarize());
}

fn print_summary_v2(item: &impl Summary) {
    println!("Summary: {}", item.summarize());
}

fn main() {
    let book = Book {
        title: String::from("1984"),
        author: String::from("George Orwell"),
    };
    let movie = Movie {
        title: String::from("Inception"),
        director: String::from("Christopher Nolan"),
    };
    print_summary(&book);
    print_summary_v2(&movie);
}
