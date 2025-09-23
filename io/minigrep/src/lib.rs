mod config;
pub use config::*;

mod search_service;
pub use search_service::*;

mod output_service;
pub use output_service::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_sensitive() {
        let query = "duct";
        let contents = "\nRust:
safe, fast, productive.
Pick three.
Duct tape.";
        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents, &false)
        );
    }
}
