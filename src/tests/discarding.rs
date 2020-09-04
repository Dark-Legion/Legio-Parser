use crate::*;

fn match_with_test(data: &[u8]) {
    let _ = data
        .match_static(b"#")
        .execute(|_, &matched: &&[u8], _| assert_eq!(matched, b"#"))
        .match_exact_with(2, |x: u8| x.is_ascii() && (x as char).is_numeric())
        .discarding(|_, _, rest: &[u8]| rest.match_with(|byte: u8| byte == b' '))
        .execute(|_, &matched: &&[u8], _| assert_eq!(matched, b"12"))
        .match_exact_with(2, |x: u8| x.is_ascii() && (x as char).is_numeric())
        .discarding(|_, _, rest: &[u8]| rest.match_with(|byte: u8| byte == b' '))
        .execute(|_, &matched: &&[u8], _| assert_eq!(matched, b"56"))
        .match_exact_with(2, |x: u8| x.is_ascii() && (x as char).is_numeric())
        .discarding(|_, _, rest: &[u8]| rest.match_with(|byte: u8| byte == b' '))
        .execute(|_, &matched: &&[u8], _| assert_eq!(matched, b"78"))
        .unwrap();
}

#[test]
fn match_with() {
    match_with_test(b"#12 56 78");
}

#[test]
#[should_panic]
fn match_with_panic() {
    match_with_test(b"#AB CD EF");
}

fn match_with_str_test(data: &str) {
    let _ = data
        .match_static("#")
        .execute(|_, &matched: &&str, _| assert_eq!(matched, "#"))
        .match_exact_with(2, |c: char| c.is_ascii() && c.is_numeric())
        .discarding(|_, _, rest: &str| rest.match_with(|c: char| c.is_whitespace()))
        .execute(|_, &matched: &&str, _| assert_eq!(matched, "12"))
        .match_exact_with(2, |c: char| c.is_ascii() && c.is_numeric())
        .discarding(|_, _, rest: &str| rest.match_with(|c: char| c.is_whitespace()))
        .execute(|_, &matched: &&str, _| assert_eq!(matched, "56"))
        .match_exact_with(2, |c: char| c.is_ascii() && c.is_numeric())
        .discarding(|_, _, rest: &str| rest.match_with(|c: char| c.is_whitespace()))
        .execute(|_, &matched: &&str, _| assert_eq!(matched, "78"))
        .unwrap();
}

#[test]
fn match_with_str() {
    match_with_str_test("#12 56 78");
}

#[test]
#[should_panic]
fn match_with_str_panic() {
    match_with_str_test("#AB CD EF");
}
