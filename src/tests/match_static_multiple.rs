use crate::*;

fn match_static_multiple_test(data: &[u8]) {
    const PATTERN_GROUPS: &[&[&[u8]]] = &[&[b"12", b"34"], &[b"34", b"56"], &[b"56", b"78"]];

    let _ = data
        .match_static(b"#")
        .and_then(|_, _, rest: &[u8]| rest.match_static_multiple(PATTERN_GROUPS[0]))
        .and_then(|_, _, rest: &[u8]| rest.match_static_multiple(PATTERN_GROUPS[1]))
        .and_then(|_, _, rest: &[u8]| rest.match_static_multiple(PATTERN_GROUPS[2]))
        .unwrap();
}

#[test]
fn match_static_multiple() {
    match_static_multiple_test(b"#343478");
}

#[test]
#[should_panic]
fn match_static_multiple_panic() {
    match_static_multiple_test(b"#123789");
}

fn match_static_multiple_str_test(data: &str) {
    let _ = data
        .match_static_multiple(&["!", "@", "#"])
        .and_then(|_, _, rest: &str| rest.match_static_multiple(&["12", "34"]))
        .and_then(|_, _, rest: &str| rest.match_static_multiple(&["34", "56"]))
        .and_then(|_, _, rest: &str| rest.match_static_multiple(&["56", "78"]))
        .unwrap();
}

#[test]
fn match_static_multiple_str() {
    match_static_multiple_str_test("#125678");
}

#[test]
#[should_panic]
fn match_static_multiple_str_panic() {
    match_static_multiple_str_test("#123789");
}
