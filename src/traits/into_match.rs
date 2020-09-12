use crate::result::Match;

/// Equivalent to the standard library's `Into<T>`.
pub trait IntoMatch: Sized {
    /// Creates new empty [`Match<T, Self>`] instance.
    fn into_match<T>(self) -> Match<T, Self> {
        Match::new(None, self)
    }
}

impl<T> IntoMatch for T {}

impl<T, U> From<U> for Match<T, U> {
    fn from(data: U) -> Self {
        data.into_match()
    }
}
