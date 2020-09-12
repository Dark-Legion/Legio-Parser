use crate::result::Match;

/// Provides interface for matching single "dynamic" pattern.
/// This is a counter part of [`MatchStatic`].
/// ## Inplementation & usage
/// The forth parameter is a helper parameter which defaults to `()`.
/// It can be used to implement overloading by saving the function parameters, for example.
/// When this trait is used as a super trait, it is **strongly recommented** to put a
/// fully generic type (with no constrains) as the helper parameter.
///
/// [`MatchStatic`]: trait.MatchStatic.html
pub trait MatchWith<F, M, R, H = ()>: Sized {
    /// Matches a "dynamic" pattern by taking a function instead.
    fn match_with(self, pattern: F) -> Match<M, R>;
}

impl<E, F> MatchWith<F, Self, Self, E> for &[E]
where
    E: Clone,
    F: FnMut(E) -> bool,
{
    fn match_with(self, mut pattern: F) -> Match<Self, Self> {
        for (index, element) in self.iter().enumerate() {
            if !pattern(element.clone()) {
                return Match::new(Some(&self[..index]), &self[index..]);
            }
        }

        Match::new(Some(self), &self[self.len()..])
    }
}

impl<E, F> MatchWith<F, Self, Self, &E> for &[E]
where
    F: FnMut(&E) -> bool,
{
    fn match_with(self, mut pattern: F) -> Match<Self, Self> {
        for (index, element) in self.iter().enumerate() {
            if !pattern(element) {
                return Match::new(Some(&self[..index]), &self[index..]);
            }
        }

        Match::new(Some(self), &self[self.len()..])
    }
}

impl<F> MatchWith<F, Self, Self, char> for &str
where
    F: FnMut(char) -> bool,
{
    fn match_with(self, mut pattern: F) -> Match<Self, Self> {
        for (index, element) in self.char_indices() {
            if !pattern(element) {
                return Match::new(Some(&self[..index]), &self[index..]);
            }
        }

        Match::new(Some(self), &self[self.len()..])
    }
}

impl<F> MatchWith<F, Self, Self, &char> for &str
where
    F: FnMut(&char) -> bool,
{
    fn match_with(self, mut pattern: F) -> Match<Self, Self> {
        for (index, element) in self.char_indices() {
            if !pattern(&element) {
                return Match::new(Some(&self[..index]), &self[index..]);
            }
        }

        Match::new(Some(self), &self[self.len()..])
    }
}
