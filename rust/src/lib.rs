
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
    let char_set: HashSet<_> = s.to_uppercase().chars().collect();
    let chars_list: Vec<_> = s.chars().collect();

    // println!("{:?} {:?}", char_set, chars_list);
    match char_set.len() == chars_list.len(){
        c=> c
    }
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
    let mut chars_list: Vec<_> = s.chars().collect();
    chars_list.sort();
    let has_duplicates = chars_list[1..]
        .iter()
        .zip(&chars_list[..chars_list.len()])
        .any(|x| x.0 == x.1);
    match has_duplicates{
        c=> c
    }
}


