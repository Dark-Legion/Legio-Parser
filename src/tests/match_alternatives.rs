use crate::traits::*;

fn match_alternatives_test(data: &[u8]) {
    let _ = data
        .alternatives::<&[u8], &[u8]>()
        .add_path(|rest| rest.match_static(b"!"))
        .add_path(|rest| rest.match_static(b"@"))
        .add_path(|rest| rest.match_static(b"#"))
        .finalize()
        .alternatives::<&[u8], &[u8]>()
        .add_path(|rest| rest.match_static(b"01"))
        .add_path(|rest| rest.match_static(b"12"))
        .add_path(|rest| rest.match_static(b"23"))
        .finalize()
        .alternatives::<&[u8], &[u8]>()
        .add_path(|rest| rest.match_static(b"23"))
        .add_path(|rest| rest.match_static(b"34"))
        .add_path(|rest| rest.match_static(b"45"))
        .finalize()
        .alternatives::<&[u8], &[u8]>()
        .add_path(|rest| rest.match_static(b"45"))
        .add_path(|rest| rest.match_static(b"56"))
        .add_path(|rest| rest.match_static(b"67"))
        .finalize()
        .unwrap();
}

#[test]
fn match_alternatives() {
    match_alternatives_test(b"#123456");
}

#[test]
#[should_panic]
fn match_alternatives_panic() {
    match_alternatives_test(b"#012340");
}

fn match_alternatives_str_test(data: &str) {
    let _ = data
        .alternatives::<&str, &str>()
        .add_path(|matched| matched.match_static("!"))
        .add_path(|matched| matched.match_static("@"))
        .add_path(|matched| matched.match_static("#"))
        .finalize()
        .alternatives::<&str, &str>()
        .add_path(|rest| rest.match_static("01"))
        .add_path(|rest| rest.match_static("12"))
        .add_path(|rest| rest.match_static("23"))
        .finalize()
        .alternatives::<&str, &str>()
        .add_path(|rest| rest.match_static("23"))
        .add_path(|rest| rest.match_static("34"))
        .add_path(|rest| rest.match_static("45"))
        .finalize()
        .alternatives::<&str, &str>()
        .add_path(|rest| rest.match_static("45"))
        .add_path(|rest| rest.match_static("56"))
        .add_path(|rest| rest.match_static("67"))
        .finalize()
        .unwrap();
}

#[test]
fn match_alternatives_str() {
    match_alternatives_str_test("#123456");
}

#[test]
#[should_panic]
fn match_alternatives_str_panic() {
    match_alternatives_str_test("#000000");
}
