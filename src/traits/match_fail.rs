/// Builder for failing matching results.
pub trait MatchFail: Sized {
    /// Constructs a new "failed" instance.
    fn failed() -> Self;
}
