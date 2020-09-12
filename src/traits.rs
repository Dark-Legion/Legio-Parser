//! This module holds all traits that are exposed from the crate.

mod into_match;
pub use into_match::*;

mod match_static;
pub use match_static::*;

mod match_with;
pub use match_with::*;

mod match_with_in_range;
pub use match_with_in_range::*;
