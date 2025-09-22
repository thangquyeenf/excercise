use std::collections::HashMap;

fn word_count(s: &str) -> HashMap<String, usize> {
    let mut word_count = HashMap::new();
    for word in s.split_whitespace() {
        let word = word.to_lowercase();
        *word_count.entry(word).or_insert(0) += 1;
    }
    word_count
}

fn group_by_length(words: Vec<&str>) -> HashMap<usize, Vec<String>> {
    let mut length_map: HashMap<usize, Vec<String>> = HashMap::new();
    for word in words {
        length_map.entry(word.len()).or_insert_with(Vec::new).push(word.to_string());
    }
    length_map
}


struct PhoneBook {
    entries: HashMap<String, String>,
}

impl PhoneBook {
    fn new() -> Self {
        PhoneBook {
            entries: HashMap::new(),
        }
    }

    fn add_entry(&mut self, name: String, number: String) {
        self.entries.insert(name, number);
    }

    fn get_number(&self, name: &str) -> Option<&String> {
        self.entries.get(name)
    }

    fn remove_entry(&mut self, name: &str) {
        self.entries.remove(name);
    }
}

fn main() {
    let text = "Hello world hello Rust";
    let counts = word_count(text);
    println!("Word counts: {:?}", counts);

    let words = vec!["apple", "banana", "pear", "kiwi", "fig", "grape"];
    let grouped = group_by_length(words);
    println!("Grouped by length: {:?}", grouped);
    let mut phone_book = PhoneBook::new();
    phone_book.add_entry("Alice".to_string(), "123-456-7890".to_string());
    phone_book.add_entry("Bob".to_string(), "987-654-3210".to_string());
    println!("Alice's number: {:?}", phone_book.get_number("Alice"));
    // update Alice's number
    phone_book.add_entry("Alice".to_string(), "111-222-3333".to_string());
    println!("Alice's updated number: {:?}", phone_book.get_number("Alice"));
    // remove Alice's entry
    phone_book.remove_entry("Alice");
    println!("Alice's number after removal: {:?}", phone_book.get_number("Alice"));
}
