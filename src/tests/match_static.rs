use crate::*;

fn match_static_test(data: &[u8]) {
    data.match_static(b"#")
        .and_then(|_, _, rest: &[u8]| rest.match_static(b"12"))
        .and_then(|_, _, rest: &[u8]| rest.match_static(b"34"))
        .and_then(|_, _, rest: &[u8]| rest.match_static(b"56"))
        .unwrap();
}

#[test]
fn match_static() {
    match_static_test(b"#123456");
}

#[test]
#[should_panic]
fn match_static_panic() {
    match_static_test(b"#000000");
}

fn match_static_str_test(data: &str) {
    data.match_static("#")
        .and_then(|_, _, rest: &str| rest.match_static("12"))
        .and_then(|_, _, rest: &str| rest.match_static("34"))
        .and_then(|_, _, rest: &str| rest.match_static("56"))
        .unwrap();
}

#[test]
fn match_static_str() {
    match_static_str_test("#123456");
}

#[test]
#[should_panic]
fn match_static_str_panic() {
    match_static_str_test("#000000");
}
