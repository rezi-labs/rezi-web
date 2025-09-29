use regex::Regex;

lazy_static::lazy_static! {
    static ref LINK_REGEX: Regex = Regex::new(r"https?://[^\s]+").unwrap();
}

/// Removes HTTP and HTTPS links from text, replacing them with nothing
pub fn remove_links(input: &str) -> String {
    LINK_REGEX.replace_all(input, "").to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_links_basic() {
        let text = "Check out https://example.com for more info";
        let result = remove_links(text);
        assert_eq!(result, "Check out  for more info");
    }

    #[test]
    fn test_remove_links_multiple() {
        let text = "Visit https://example.com and http://test.org today";
        let result = remove_links(text);
        assert_eq!(result, "Visit  and  today");
    }

    #[test]
    fn test_remove_links_no_links() {
        let text = "This has no links in it";
        let result = remove_links(text);
        assert_eq!(result, "This has no links in it");
    }
}
