
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
