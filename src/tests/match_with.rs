use crate::*;

fn match_with_test(data: &[u8]) {
    let _ = data
        .match_static(b"#")
        .match_exact_with(6, |x: u8| x.is_ascii() && (x as char).is_numeric())
        .unwrap();
}

#[test]
fn match_with() {
    match_with_test(b"#125678");
}

#[test]
#[should_panic]
fn match_with_panic() {
    match_with_test(b"#ABCDEF");
}

fn match_with_str_test(data: &str) {
    let _ = data
        .match_static("#")
        .match_exact_with(6, |c: char| c.is_ascii() && c.is_numeric())
        .unwrap();
}

#[test]
fn match_with_str() {
    match_with_str_test("#125678");
}

#[test]
#[should_panic]
fn match_with_str_panic() {
    match_with_str_test("#ABCDEF");
}
