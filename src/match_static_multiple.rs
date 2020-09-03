use crate::{Match, MatchStatic};

/// Provides interface for matching multiple "static" patterns.
/// This trait provides general abstraction over [`MatchStatic`] for multiple patterns.
/// See [`MatchStatic`] for more information.
///
/// [`MatchStatic`]: trait.MatchStatic.html
pub trait MatchStaticMultiple<'object, E, T, R> {
    /// Matches multiple "static" patterns.
    fn match_static_multiple(&'object self, pattern: T) -> Match<R>;
}

impl<'object, 'pattern, E, T, I> MatchStaticMultiple<'object, E, T, &'object [E]> for I
where
    E: 'pattern + PartialEq + Clone,
    T: AsRef<[&'pattern [E]]>,
    I: MatchStatic<'object, E, &'pattern [E], &'object [E]> + ?Sized,
{
    fn match_static_multiple(&'object self, patterns: T) -> Match<&'object [E]> {
        for (index, &pattern) in patterns.as_ref().iter().enumerate() {
            if let Ok(mut matched) = self.match_static(pattern).into_successful() {
                matched.set_index(index);

                return matched.into();
            }
        }

        Match::failed()
    }
}

impl<'object, 'pattern, T> MatchStaticMultiple<'object, char, T, &'object str> for str
where
    T: AsRef<[&'pattern str]>,
{
    fn match_static_multiple(&'object self, pattern: T) -> Match<&'object Self> {
        for (index, pattern) in pattern.as_ref().iter().enumerate() {
            if let Ok(mut matched) = self.match_static(pattern).into_successful() {
                matched.set_index(index);

                return matched.into();
            }
        }

        Match::failed()
    }
}
