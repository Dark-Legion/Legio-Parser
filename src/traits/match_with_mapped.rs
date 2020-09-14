use crate::{
    result::{MappedMatch, Match},
    traits::MatchWith,
};

/// Provides interface for matching single "dynamic" pattern.
/// This is a counter part of [`MatchStatic`].
/// ## Inplementation & usage
/// The forth parameter is a helper parameter which defaults to `()`.
/// It can be used to implement overloading by saving the function parameters, for example.
/// When this trait is used as a super trait, it is **strongly recommented** to put a
/// fully generic type (with no constrains) as the helper parameter.
///
/// [`MatchStatic`]: trait.MatchStatic.html
pub trait MatchWithMapped<F, R, Q, H = ()>: Sized {
    /// Matches a "dynamic" pattern by taking a function instead.
    fn match_with_mapped(self, pattern: F, value: Q) -> R;
}

impl<F, M, R, Q, H, I> MatchWithMapped<F, MappedMatch<M, R, Q>, Q, H> for I
where
    I: MatchWith<F, Match<M, R>, H>,
{
    fn match_with_mapped(self, pattern: F, value: Q) -> MappedMatch<M, R, Q> {
        self.match_with(pattern).map(value)
    }
}
