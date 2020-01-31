use std::collections::HashSet;

/// Returns true if the string slice s has all unique characters.
/// returns false otherwise
/// # Arguments
/////
/// * `s` - A string slice to test for unique characters
///
/// # Remarks
/// function supports UTF chars and any other character regardless
/// of case, if applicable.
///
pub fn is_unique(s: &str) -> bool {
    s.chars().count() == s.chars().collect::<HashSet<_>>().len()
}

/// Returns true if the string slice s has all unique characters.
/// returns false otherwise.
/// # Arguments
/////
/// * `s` - A string slice to test for unique characters
///
/// # Remarks
/// Assumes that we can sort the string.
/// Returns true if any consecutive characters in the sorted string match and false otherwise.
///
pub fn is_unique_no_ds(s: &str) -> bool {
    let mut chars: Vec<_> = s.chars().collect();
    chars.sort();

    for i in 1..chars.len() {
        if chars[i] == chars[i - 1] {
            return false;
        }
    }

    true
}

// It's idiomatic in rust to include the tests alongside the module that is being tested.
// https://doc.rust-lang.org/book/ch11-01-writing-tests.html
#[cfg(test)]
mod tests {
    use crate::{is_unique, is_unique_no_ds};

    // test for is_unique
    #[test]
    fn empty() {
        assert_eq!(is_unique(""), true);
    }
    #[test]
    fn utf_unique() {
        assert_eq!(is_unique("🤣😊🤔"), true);
    }
    #[test]
    fn utf_not_unique() {
        assert_eq!(is_unique("🤣🤔🤔"), false);
    }
    #[test]
    fn basic_unique() {
        assert_eq!(is_unique("something"), true);
    }
    #[test]
    fn basic_not_unique() {
        assert_eq!(is_unique("summer"), false);
    }
    #[test]
    fn spaces_not_unique() {
        // contains two spaces; should return false
        assert_eq!(is_unique("A B C"), false);
    }
    #[test]
    fn mixedcase_unique() {
        assert_eq!(is_unique("SomeThing"), true);
    }
    #[test]
    fn mixedcase_not_unique() {
        assert_eq!(is_unique("SomeString"), false);
    }

    // test for is_unique
    #[test]
    fn empty_no_ds() {
        assert_eq!(is_unique_no_ds(""), true);
    }
    #[test]
    fn utf_unique_no_ds() {
        assert_eq!(is_unique_no_ds("🤣😊🤔"), true);
    }
    #[test]
    fn utf_not_unique_no_ds() {
        assert_eq!(is_unique_no_ds("🤣🤔🤔"), false);
    }
    #[test]
    fn basic_unique_no_ds() {
        assert_eq!(is_unique_no_ds("something"), true);
    }
    #[test]
    fn basic_not_unique_no_ds() {
        assert_eq!(is_unique_no_ds("summer"), false);
    }
    #[test]
    fn spaces_not_unique_no_ds() {
        // contains two spaces; should return false
        assert_eq!(is_unique_no_ds("A B C"), false);
    }
    #[test]
    fn mixedcase_unique_no_ds() {
        assert_eq!(is_unique_no_ds("SomeThing"), true);
    }
    #[test]
    fn mixedcase_not_unique_no_ds() {
        assert_eq!(is_unique_no_ds("SomeString"), false);
    }
}
