use crate::{Match, SuccessfulMatch};

/// Provides interface for matching single "static" pattern.
/// "Static" in this case is rather "not dynamic" (not changing) during the call, than constant.
pub trait MatchStatic<'object, E, T, R> {
    /// Matches a "static" pattern.
    fn match_static(&'object self, pattern: T) -> Match<R>;
}

impl<'object, E, T> MatchStatic<'object, E, T, &'object Self> for [E]
where
    E: PartialEq,
    T: AsRef<[E]>,
{
    fn match_static(&'object self, pattern: T) -> Match<&'object Self> {
        let pattern: &[E] = pattern.as_ref();

        if pattern.is_empty() {
            return SuccessfulMatch::<&'object Self>::new(0, &self[..0], self).into();
        }

        if pattern.len() <= self.len() {
            if &self[..pattern.len()] == pattern {
                SuccessfulMatch::new(0, &self[..pattern.len()], &self[pattern.len()..]).into()
            } else {
                Match::failed()
            }
        } else {
            Match::failed()
        }
    }
}

impl<'object, 'pattern> MatchStatic<'object, char, &'pattern Self, &'object Self> for str {
    fn match_static(&'object self, pattern: &'pattern Self) -> Match<&'object Self> {
        if pattern.len() <= self.len() {
            if &self[..pattern.len()] == pattern {
                SuccessfulMatch::new(0, &self[..pattern.len()], &self[pattern.len()..]).into()
            } else {
                Match::failed()
            }
        } else {
            Match::failed()
        }
    }
}

impl<'object, E, T, R, I> MatchStatic<'object, E, T, R> for &I
where
    I: MatchStatic<'object, E, T, R> + ?Sized,
{
    fn match_static(&'object self, pattern: T) -> Match<R> {
        (**self).match_static(pattern)
    }
}

impl<'object, E, T, R, I> MatchStatic<'object, E, T, R> for &mut I
where
    I: MatchStatic<'object, E, T, R> + ?Sized,
{
    fn match_static(&'object self, pattern: T) -> Match<R> {
        (**self).match_static(pattern)
    }
}
