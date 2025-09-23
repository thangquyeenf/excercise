#[derive(Debug)]
pub struct Config {
    pub query: String,
    pub filename: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() != 3 {
            return Err("Usage: minigrep <filename> <query>");
        }
        let query = args[1].clone();
        let filename = args[2].clone();
        let ignore_case = std::env::var("IGNORE_CASE").is_ok();
        Ok(Config {
            query,
            filename,
            ignore_case,
        })
    }
}
