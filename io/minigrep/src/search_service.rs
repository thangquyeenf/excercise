pub fn search<'a>(query: &str, contents: &'a str, ignore_case: &'a bool) -> Vec<&'a str> {
    let query = if *ignore_case {
        query.to_lowercase()
    } else {
        query.to_string()
    };
    contents
        .lines()
        .filter(|line| {
            if *ignore_case {
                line.to_lowercase().contains(&query)
            } else {
                line.contains(&query)
            }
        })
        .collect()
}
