use crate::{
    result::{MappedMatch, Match},
    traits::MatchStatic,
};

/// Provides interface for matching single "static" pattern.
/// "Static" in this case is rather "not dynamic" (not changing) during the call, than constant.
pub trait MatchStaticMapped<E, T, R, M>: Sized {
    /// Matches a "static" pattern.
    fn match_static_mapped(self, pattern: T, value: M) -> R;
}

impl<E, T, M, R, Q, I> MatchStaticMapped<E, T, MappedMatch<M, R, Q>, Q> for I
where
    Self: MatchStatic<E, T, Match<M, R>>,
{
    fn match_static_mapped(self, pattern: T, value: Q) -> MappedMatch<M, R, Q> {
        self.match_static(pattern).map(value)
    }
}
