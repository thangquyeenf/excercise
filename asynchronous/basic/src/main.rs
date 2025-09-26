use trpl::Runtime;

async fn fetch_data() -> Option<String> {
    trpl::sleep(std::time::Duration::from_secs(1)).await;
    // Simulate an asynchronous operation
    Some("Fetched Data".to_string())
}

fn main() {
    let rt = Runtime::new().unwrap();
    let result = rt.block_on(fetch_data());
    match result {
        Some(data) => println!("Success: {}", data),
        None => println!("Failed to fetch data"),
    }
}