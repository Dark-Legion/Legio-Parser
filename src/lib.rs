//! # Crate
//! This crate provides interfaces for parsing by pattern matching.
//!
//! # No unsafe code!
//! This crate forbids the usage of `unsafe` code within the crate, providing only safe interfaces.
//!
//! # Default features
//! * `std`
//!
//! # Features
//! * `std`
//!     * Provides interfaces for pattern matching that use the standard library.
//!     * Opt-out of this feature to use limited version relying only on `libcore`.
//!     * **Note**: Opting-out will limit some functionalities.

#![cfg_attr(not(feature = "std"), no_std)]
#![forbid(
    warnings,
    unused,
    missing_docs,
    unsafe_code,
    clippy::all,
    clippy::pedantic,
    clippy::cargo
)]

mod match_result;
pub use match_result::*;

/// This module re-exports all essential types and all (public) traits.
///
/// Traits that are inaccessible are used only for implementations, leaving traits free for new implementations.
pub mod prelude {
    pub use crate::{Match, MatchFailed, MatchStatic, MatchWith, MatchWithInRange};
}

mod match_static;
pub use match_static::*;

mod match_static_multiple;
pub use match_static_multiple::*;

mod match_with;
pub use match_with::*;

mod match_with_in_range;
pub use match_with_in_range::*;

#[cfg(test)]
mod tests {
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

    fn match_static_multiple_test(data: &[u8]) {
        const PATTERN_GROUPS: &[&[&[u8]]] = &[&[b"12", b"34"], &[b"34", b"56"], &[b"56", b"78"]];

        data.match_static(b"#")
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
        data.match_static_multiple(&["!", "@", "#"])
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

    fn match_with_test(data: &[u8]) {
        data.match_static(b"#")
            .and_then(|_, _, rest: &[u8]| {
                rest.match_exact_with(6, |x: u8| x.is_ascii() && (x as char).is_numeric())
            })
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
        data.match_static("#")
            .and_then(|_, _, rest: &str| {
                rest.match_exact_with(6, |c: char| c.is_ascii() && c.is_numeric())
            })
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

    #[cfg(feature = "std")]
    fn collecting_match_test(data: &[u8]) {
        data.match_static(b"#")
            .into_collecting()
            .and_then(|_, _, rest: &[u8]| {
                rest.match_exact_with(2, |byte: u8| (byte as char).is_numeric())
            })
            .and_then(|_, _, rest: &[u8]| {
                rest.match_exact_with(2, |byte: u8| (byte as char).is_numeric())
            })
            .and_then(|_, _, rest: &[u8]| {
                rest.match_exact_with(2, |byte: u8| (byte as char).is_numeric())
            })
            .finalize()
            .unwrap();
    }

    #[test]
    #[cfg(feature = "std")]
    fn collecting_match() {
        collecting_match_test(b"#123456");
    }

    #[test]
    #[should_panic]
    #[cfg(feature = "std")]
    fn collecting_match_panic() {
        collecting_match_test(b"#ABCDEF");
    }

    #[cfg(feature = "std")]
    fn collecting_match_str_test(data: &str) {
        data.match_static("#")
            .into_collecting()
            .and_then(|_, _, rest: &str| rest.match_exact_with(2, |c: char| c.is_numeric()))
            .and_then(|_, _, rest: &str| rest.match_exact_with(2, |c: char| c.is_numeric()))
            .and_then(|_, _, rest: &str| rest.match_exact_with(2, |c: char| c.is_numeric()))
            .finalize()
            .unwrap();
    }

    #[test]
    #[cfg(feature = "std")]
    fn collecting_match_str() {
        collecting_match_str_test("#123456");
    }

    #[test]
    #[should_panic]
    #[cfg(feature = "std")]
    fn collecting_match_str_panic() {
        collecting_match_str_test("#ABCDEF");
    }

    #[cfg(feature = "std")]
    fn collecting_match_repeat_test(data: &[u8]) {
        data.match_static(b"#")
            .into_collecting()
            .and_then_repeat(3, |_, _, rest: &[u8]| {
                rest.match_exact_with(2, |byte: u8| (byte as char).is_numeric())
            })
            .finalize()
            .unwrap();
    }

    #[test]
    #[cfg(feature = "std")]
    fn collecting_match_repeat() {
        collecting_match_repeat_test(b"#123456");
    }

    #[test]
    #[should_panic]
    #[cfg(feature = "std")]
    fn collecting_match_repeat_panic() {
        collecting_match_repeat_test(b"#ABCDEF");
    }

    #[cfg(feature = "std")]
    fn collecting_match_repeat_str_test(data: &str) {
        data.match_static("#")
            .into_collecting()
            .and_then_repeat(3, |_, _, rest: &str| {
                rest.match_exact_with(2, |c: char| c.is_numeric())
            })
            .finalize()
            .unwrap();
    }

    #[test]
    #[cfg(feature = "std")]
    fn collecting_match_repeat_str() {
        collecting_match_repeat_str_test("#123456");
    }

    #[test]
    #[should_panic]
    #[cfg(feature = "std")]
    fn collecting_match_repeat_str_panic() {
        collecting_match_repeat_str_test("#ABCDEF");
    }
}
