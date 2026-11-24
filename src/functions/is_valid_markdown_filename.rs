use regex::Regex;
use std::sync::LazyLock;

static MARKDOWN_FILENAME_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^[a-zA-Z0-9_\-][a-zA-Z0-9._\-]*\.md$").unwrap());

pub fn is_valid_markdown_filename(filename: &str) -> bool {
    filename.len() <= 255 && MARKDOWN_FILENAME_REGEX.is_match(filename)
}

#[cfg(test)]
mod tests {
    use super::is_valid_markdown_filename;

    #[test]
    fn accepts_regular_filenames() {
        assert!(is_valid_markdown_filename("test.md"));
        assert!(is_valid_markdown_filename("test-blog_2.md"));
    }

    #[test]
    fn rejects_unsafe_filenames() {
        assert!(!is_valid_markdown_filename("../secret.md"));
        assert!(!is_valid_markdown_filename("..md"));
        assert!(!is_valid_markdown_filename(".hidden.md"));
        assert!(!is_valid_markdown_filename("dir/test.md"));
        assert!(!is_valid_markdown_filename("test.txt"));
        assert!(!is_valid_markdown_filename(&format!("{}.md", "a".repeat(300))));
    }
}
