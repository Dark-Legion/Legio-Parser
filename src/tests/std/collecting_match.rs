use crate::*;

fn collecting_match_test(data: &[u8]) {
    data.match_static(b"#")
        .into_collecting()
        .repeat(3, |_, rest: &[u8]| {
            rest.match_exact_with(2, |byte: u8| (byte as char).is_numeric())
        })
        .finalize()
        .unwrap();
}

#[test]
fn collecting_match() {
    collecting_match_test(b"#123456");
}

#[test]
#[should_panic]
fn collecting_match_panic() {
    collecting_match_test(b"#ABCDEF");
}

fn collecting_match_str_test(data: &str) {
    data.match_static("#")
        .into_collecting()
        .repeat(3, |_, rest: &str| {
            rest.match_exact_with(2, |c: char| c.is_numeric())
        })
        .finalize()
        .unwrap();
}

#[test]
fn collecting_match_str() {
    collecting_match_str_test("#123456");
}

#[test]
#[should_panic]
fn collecting_match_str_panic() {
    collecting_match_str_test("#ABCDEF");
}

fn collecting_match_repeat_test(data: &[u8]) {
    data.match_static(b"#")
        .into_collecting()
        .repeat(3, |_, rest: &[u8]| {
            rest.match_exact_with(2, |byte: u8| (byte as char).is_numeric())
        })
        .finalize()
        .unwrap();
}

#[test]
fn collecting_match_repeat() {
    collecting_match_repeat_test(b"#123456");
}

#[test]
#[should_panic]
fn collecting_match_repeat_panic() {
    collecting_match_repeat_test(b"#ABCDEF");
}

fn collecting_match_repeat_str_test(data: &str) {
    data.match_static("#")
        .into_collecting()
        .repeat(3, |_, rest: &str| {
            rest.match_exact_with(2, |c: char| c.is_numeric())
        })
        .finalize()
        .unwrap();
}

#[test]
fn collecting_match_repeat_str() {
    collecting_match_repeat_str_test("#123456");
}

#[test]
#[should_panic]
fn collecting_match_repeat_str_panic() {
    collecting_match_repeat_str_test("#ABCDEF");
}
