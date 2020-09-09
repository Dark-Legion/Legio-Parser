use crate::Match;

/// Provides interface for matching single "static" pattern.
/// "Static" in this case is rather "not dynamic" (not changing) during the call, than constant.
pub trait MatchStatic<E, T, R>: Sized {
    /// Matches a "static" pattern.
    fn match_static(self, pattern: T) -> Match<R>;
}

impl<E, T> MatchStatic<E, T, Self> for &[E]
where
    E: PartialEq,
    T: AsRef<[E]>,
{
    fn match_static(self, pattern: T) -> Match<Self> {
        let pattern: &[E] = pattern.as_ref();

        if pattern.is_empty() {
            return Match::<Self>::new(Some(&self[..0]), &self);
        }

        if pattern.len() <= self.len() {
            if &self[..pattern.len()] == pattern {
                Match::new(Some(&self[..pattern.len()]), &self[pattern.len()..])
            } else {
                Match::failed()
            }
        } else {
            Match::failed()
        }
    }
}

impl<T> MatchStatic<char, T, Self> for &str
where
    T: AsRef<str>,
{
    fn match_static(self, pattern: T) -> Match<Self> {
        let pattern: &str = pattern.as_ref();

        if pattern.len() <= self.len() {
            if &self[..pattern.len()] == pattern {
                Match::new(Some(&self[..pattern.len()]), &self[pattern.len()..])
            } else {
                Match::failed()
            }
        } else {
            Match::failed()
        }
    }
}

impl<E, T, R, I> MatchStatic<E, T, R> for &mut I
where
    I: ?Sized,
    for<'r> &'r I: MatchStatic<E, T, R>,
{
    fn match_static(self, pattern: T) -> Match<R> {
        (&*self).match_static(pattern)
    }
}
