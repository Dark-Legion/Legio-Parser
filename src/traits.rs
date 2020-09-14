//! This module holds all traits that are exposed from the crate.

mod alternatives;
pub use alternatives::*;

mod into_match;
pub use into_match::*;

mod match_fail;
pub use match_fail::*;

mod match_static;
pub use match_static::*;

mod match_static_mapped;
pub use match_static_mapped::*;

mod match_with;
pub use match_with::*;

mod match_with_mapped;
pub use match_with_mapped::*;

mod match_with_in_range;
pub use match_with_in_range::*;

mod match_with_in_range_mapped;
pub use match_with_in_range_mapped::*;
