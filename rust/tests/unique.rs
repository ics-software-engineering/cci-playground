#[test]
fn empty() {
    assert_eq!(unique::is_unique(""), true);
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
fn mixedcase_unique() {
    assert_eq!(unique::is_unique("SomeThing"), true);
}

#[test]
fn mixedcase_not_unique() {
    assert_eq!(unique::is_unique("SomeString"), false);
}

#[test]
fn utf_unique() {
    assert_eq!(unique::is_unique("🤣😊🤔"), true);
}
#[test]
fn utf_not_unique() {
    assert_eq!(unique::is_unique("🤣🤔🤔"), false);
}
