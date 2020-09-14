use crate::result::{AlternativesMatch, MappedAlternativesMatch};

/// Equivalent to the standard library's `Into<T>`.
pub trait Alternatives: Sized {
    /// Creates a new alternatives tree with same types as the match result itself.
    fn alternatives<T, U>(self) -> AlternativesMatch<Self, T, U> {
        AlternativesMatch::new(self)
    }

    /// Creates a new mapped alternatives tree with same types as the match result itself.
    fn mapped_alternatives<T, U, V>(self) -> MappedAlternativesMatch<Self, T, U, V> {
        MappedAlternativesMatch::new(self)
    }
}

impl<T> Alternatives for T {}
