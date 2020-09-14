use crate::{
    result::{MappedMatch, Match},
    traits::MatchWithInRange,
};

/// Provides interface for matching single "dynamic" pattern.
/// This is a counter part of [`MatchStatic`].
/// ## Inplementation & usage
/// The fifth parameters are helper parameter which defaults to `()`.
/// They can be used to implement overloading by saving the function parameters, for example.
/// When this trait is used as a super trait, it is **strongly recommented** to put a
/// fully generic type (with no constrains) as the helper parameter.
///
/// [`MatchStatic`]: trait.MatchStatic.html
pub trait MatchWithInRangeMapped<N, F, R, Q, H1 = (), H2 = ()> {
    /// Matches a "dynamic" pattern by taking a function instead with taking into account a minimum amount.
    fn match_min_with_mapped(self, minimum: N, pattern: F, value: Q) -> R;

    /// Matches a "dynamic" pattern by taking a function instead with taking into account a maximum amount.
    fn match_max_with_mapped(self, maximum: N, pattern: F, value: Q) -> R;

    /// Matches a "dynamic" pattern by taking a function instead with taking into account a minimum and maximum amount.
    fn match_min_max_with_mapped(self, minimum: N, maximum: N, pattern: F, value: Q) -> R;

    /// Matches a "dynamic" pattern by taking a function instead with taking into account a exact amount.
    fn match_exact_with_mapped(self, count: N, pattern: F, value: Q) -> R;
}

impl<N, F, M, R, Q, H1, H2, I> MatchWithInRangeMapped<N, F, MappedMatch<M, R, Q>, Q, H1, H2> for I
where
    I: MatchWithInRange<N, F, Match<M, R>, H1, H2>,
{
    fn match_min_with_mapped(self, minimum: N, pattern: F, value: Q) -> MappedMatch<M, R, Q> {
        self.match_min_with(minimum, pattern).map(value)
    }

    fn match_max_with_mapped(self, maximum: N, pattern: F, value: Q) -> MappedMatch<M, R, Q> {
        self.match_max_with(maximum, pattern).map(value)
    }

    fn match_min_max_with_mapped(
        self,
        minimum: N,
        maximum: N,
        pattern: F,
        value: Q,
    ) -> MappedMatch<M, R, Q> {
        self.match_min_max_with(minimum, maximum, pattern)
            .map(value)
    }

    fn match_exact_with_mapped(self, count: N, pattern: F, value: Q) -> MappedMatch<M, R, Q> {
        self.match_exact_with(count, pattern).map(value)
    }
}
