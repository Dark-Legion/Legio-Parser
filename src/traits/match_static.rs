use crate::{result::Match, traits::MatchFail};

/// Provides interface for matching single "static" pattern.
/// "Static" in this case is rather "not dynamic" (not changing) during the call, than constant.
pub trait MatchStatic<E, T, R>: Sized {
    /// Matches a "static" pattern.
    fn match_static(self, pattern: T) -> R;
}

impl<E, T, U> MatchStatic<U, T, Match<Self, Self>> for &[E]
where
    E: PartialEq<U>,
    T: AsRef<[U]>,
{
    fn match_static(self, pattern: T) -> Match<Self, Self> {
        let pattern: &[U] = pattern.as_ref();

        if pattern.is_empty() {
            return Match::<Self, Self>::new(Some(&self[..0]), &self);
        }

        let len: usize = self.len().min(pattern.len());

        if &self[..len] == pattern {
            Match::new(Some(&self[..len]), &self[len..])
        } else {
            Match::failed()
        }
    }
}

impl<T> MatchStatic<char, T, Match<Self, Self>> for &str
where
    T: AsRef<str>,
{
    fn match_static(self, pattern: T) -> Match<Self, Self> {
        let pattern: &str = pattern.as_ref();

        let len: usize = self.len().min(pattern.len());

        if &self[..len] == pattern {
            Match::new(Some(&self[..len]), &self[len..])
        } else {
            Match::failed()
        }
    }
}
