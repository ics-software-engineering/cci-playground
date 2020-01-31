// test for unique::is_unique
#[test]
fn empty() {
    assert_eq!(unique::is_unique(""), true);
}
#[test]
fn utf_unique() {
    assert_eq!(unique::is_unique("🤣😊🤔"), true);
}
#[test]
fn utf_not_unique() {
    assert_eq!(unique::is_unique("🤣🤔🤔"), false);
}
#[test]
fn basic_unique() {
    assert_eq!(unique::is_unique("something"), true);
}
#[test]
fn basic_not_unique() {
    assert_eq!(unique::is_unique("summer"), false);
}
#[test]
fn spaces_not_unique() {
    // contains two spaces; should return false
    assert_eq!(unique::is_unique("A B C"), false);
}
#[test]
fn mixedcase_unique() {
    assert_eq!(unique::is_unique("SomeThing"), true);
}
#[test]
fn mixedcase_not_unique() {
    assert_eq!(unique::is_unique("SomeString"), false);
}

// test for unique::is_unique
#[test]
fn empty_no_ds() {
    assert_eq!(unique::is_unique(""), true);
}
#[test]
fn utf_unique_no_ds() {
    assert_eq!(unique::is_unique("🤣😊🤔"), true);
}
#[test]
fn utf_not_unique_no_ds() {
    assert_eq!(unique::is_unique("🤣🤔🤔"), false);
}
#[test]
fn basic_unique_no_ds() {
    assert_eq!(unique::is_unique("something"), true);
}
#[test]
fn basic_not_unique_no_ds() {
    assert_eq!(unique::is_unique("summer"), false);
}
#[test]
fn spaces_not_unique_no_ds() {
    // contains two spaces; should return false
    assert_eq!(unique::is_unique("A B C"), false);
}
#[test]
fn mixedcase_unique_no_ds() {
    assert_eq!(unique::is_unique("SomeThing"), true);
}
#[test]
fn mixedcase_not_unique_no_ds() {
    assert_eq!(unique::is_unique("SomeString"), false);
}

