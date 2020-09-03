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
//! * `no_track_caller`
//!     * Disables the `#[track_caller]` attributes within the library.
//!     * This is required for compilation below version 1.46 of Rust.

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

/// This module re-exports all essential types and all (public) traits.
///
/// Traits that are inaccessible are used only for implementations, leaving traits free for new implementations.
pub mod prelude {
    pub use crate::{Match, MatchFailed, MatchStatic, MatchWith, MatchWithInRange};
}

mod match_result;
pub use match_result::*;

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
    mod discarding;
    mod match_static;
    mod match_static_multiple;
    mod match_with;

    #[cfg(feature = "std")]
    mod std {
        mod collecting_match;
    }
}
