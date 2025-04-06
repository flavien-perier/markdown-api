use regex::Regex;

pub fn is_valid_markdown_filename(filename: &str) -> bool {
    let re = Regex::new(r"^[a-zA-Z0-9._\-]+\.md$").unwrap();
    re.is_match(filename)
}
