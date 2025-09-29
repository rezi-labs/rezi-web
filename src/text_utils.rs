use regex::Regex;

lazy_static::lazy_static! {
    static ref LINK_REGEX: Regex = Regex::new(r"https?://[^\s]+").unwrap();
}

/// Removes HTTP and HTTPS links from text, replacing them with nothing
pub fn remove_links(input: &str) -> String {
    LINK_REGEX.replace_all(input, "").to_string()
}

/// Removes all brackets [], braces {}, and parentheses () from text
pub fn remove_unclosed_parens_after_brackets(input: &str) -> String {
    use regex::Regex;

    lazy_static::lazy_static! {
        static ref BRACKETS_BRACES_PARENS_PATTERN: Regex = Regex::new(r"[\[\]{}()]").unwrap();
    }

    BRACKETS_BRACES_PARENS_PATTERN
        .replace_all(input, "")
        .to_string()
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

    #[test]
    fn test_remove_unclosed_parens_after_brackets_basic() {
        let text = "Some text with [content] in brackets";
        let result = remove_unclosed_parens_after_brackets(text);
        assert_eq!(result, "Some text with content in brackets");
    }

    #[test]
    fn test_remove_unclosed_parens_after_brackets_matched() {
        let text = "Text with [brackets] and {braces}";
        let result = remove_unclosed_parens_after_brackets(text);
        assert_eq!(result, "Text with brackets and braces");
    }

    #[test]
    fn test_remove_unclosed_parens_after_brackets_multiple() {
        let text = "Multiple [items] and {objects} here [more]";
        let result = remove_unclosed_parens_after_brackets(text);
        assert_eq!(result, "Multiple items and objects here more");
    }

    #[test]
    fn test_remove_unclosed_parens_after_brackets_no_brackets() {
        let text = "No brackets or braces here, just regular text";
        let result = remove_unclosed_parens_after_brackets(text);
        assert_eq!(result, "No brackets or braces here, just regular text");
    }

    #[test]
    fn test_remove_unclosed_parens_after_brackets_multiple_brackets() {
        let text = "Mixed [square] and {curly} brackets together";
        let result = remove_unclosed_parens_after_brackets(text);
        assert_eq!(result, "Mixed square and curly brackets together");
    }

    #[test]
    fn test_remove_all_bracket_types() {
        let text = "Remove (parentheses) and [brackets] and {braces}";
        let result = remove_unclosed_parens_after_brackets(text);
        assert_eq!(result, "Remove parentheses and brackets and braces");
    }

    #[test]
    fn test_remove_empty_brackets_braces() {
        let text = "Empty [] and {} should be removed too";
        let result = remove_unclosed_parens_after_brackets(text);
        assert_eq!(result, "Empty  and  should be removed too");
    }

    #[test]
    fn test_version_format() {
        let text = "0.14.3( (2025-09-29)";
        let result = remove_unclosed_parens_after_brackets(text);
        assert_eq!(result, "0.14.3 2025-09-29");
    }
}
