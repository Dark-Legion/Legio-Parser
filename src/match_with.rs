use crate::Match;

/// Provides interface for matching single "dynamic" pattern.
/// This is a counter part of [`MatchStatic`].
///
/// [`MatchStatic`]: trait.MatchStatic.html
pub trait MatchWith<E, F, R>: Sized {
    /// Matches a "dynamic" pattern by taking a function instead.
    fn match_with(self, pattern: F) -> Match<R>;
}

impl<E, F> MatchWith<E, F, Self> for &[E]
where
    E: Clone,
    F: FnMut(E) -> bool,
{
    fn match_with(self, mut pattern: F) -> Match<Self> {
        for (index, element) in self.iter().enumerate() {
            if !pattern(element.clone()) {
                return Match::new(Some(&self[..index]), &self[index..]);
            }
        }

        Match::new(Some(self), &self[self.len()..])
    }
}

impl<E, F> MatchWith<&E, F, Self> for &[E]
where
    F: FnMut(&E) -> bool,
{
    fn match_with(self, mut pattern: F) -> Match<Self> {
        for (index, element) in self.iter().enumerate() {
            if !pattern(element) {
                return Match::new(Some(&self[..index]), &self[index..]);
            }
        }

        Match::new(Some(self), &self[self.len()..])
    }
}

impl<F> MatchWith<char, F, Self> for &str
where
    F: FnMut(char) -> bool,
{
    fn match_with(self, mut pattern: F) -> Match<Self> {
        for (index, element) in self.char_indices() {
            if !pattern(element) {
                return Match::new(Some(&self[..index]), &self[index..]);
            }
        }

        Match::new(Some(self), &self[self.len()..])
    }
}

impl<F> MatchWith<&char, F, Self> for &str
where
    F: FnMut(&char) -> bool,
{
    fn match_with(self, mut pattern: F) -> Match<Self> {
        for (index, element) in self.char_indices() {
            if !pattern(&element) {
                return Match::new(Some(&self[..index]), &self[index..]);
            }
        }

        Match::new(Some(self), &self[self.len()..])
    }
}

impl<E, F, R, I> MatchWith<E, F, R> for &I
where
    I: MatchWith<E, F, R> + Clone,
{
    fn match_with(self, pattern: F) -> Match<R> {
        self.clone().match_with(pattern)
    }
}
