//! This module holds all structures used to represend matching results.

use crate::traits::{MatchStatic, MatchWith, MatchWithInRange};

/// Represents failed pattern matching result.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct MatchFailed(());

/// Generic type that holds result of pattern matching.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[must_use]
pub struct Match<T, U> {
    matched: Option<T>,
    rest: Option<U>,
}

impl<T, U> Match<T, U> {
    /// Constructs a new instance.
    pub const fn new(matched: Option<T>, rest: U) -> Self {
        Self {
            matched,
            rest: Some(rest),
        }
    }

    /// Constructs a new "failed" instance.
    pub const fn failed() -> Self {
        Self {
            matched: None,
            rest: None,
        }
    }

    /// Returns boolean indicating whether the pattern was matched.
    /// This returns true when the pattern didn't match.
    pub const fn is_failed(&self) -> bool {
        matches!(self.rest, None)
    }

    /// Returns `(Option<_>(matched), rest)` wrapped in `Result`, consuming the object.
    /// # Errors
    /// Returns `Err` when matching has failed.
    pub fn take(self) -> Result<(Option<T>, U), MatchFailed> {
        if let Some(rest) = self.rest {
            Ok((self.matched, rest))
        } else {
            Err(MatchFailed(()))
        }
    }

    /// Transforms the "matched" part using the passed function.
    pub fn transform_matched<F, R>(self, f: F) -> Match<R, U>
    where
        F: FnOnce(T) -> R,
    {
        match (self.matched, self.rest) {
            (Some(matched), Some(rest)) => Match::new(Some(f(matched)), rest),
            (None, Some(rest)) => Match::new(None, rest),
            (_, None) => Match::failed(),
        }
    }

    /// Transforms the "rest" part using the passed function.
    pub fn transform_rest<F, R>(self, f: F) -> Match<T, R>
    where
        F: FnOnce(U) -> R,
    {
        match (self.matched, self.rest) {
            (matched, Some(rest)) => Match::new(matched, f(rest)),
            (_, None) => Match::failed(),
        }
    }

    /// Clears match result's "matched" part.
    pub fn clear(mut self) -> Self {
        self.matched = None;
        self
    }

    /// Returns inner state.
    /// This is a short-hand for `to_successful().unwrap()`.
    /// # Panics
    /// This function panics, if the `is_failed` function indicates an "failed" one.
    #[cfg_attr(not(feature = "no_track_caller"), track_caller)]
    pub fn unwrap(self) -> (Option<T>, U) {
        (self.matched, self.rest.unwrap())
    }

    /// Returns inner state.
    /// This is a short-hand for `to_successful().expect("...")`.
    /// # Panics
    /// This function panics, if the `is_failed` function indicates an "failed" one.
    #[cfg_attr(not(feature = "no_track_caller"), track_caller)]
    pub fn expect(self, msg: &str) -> (Option<T>, U) {
        (self.matched, self.rest.expect(msg))
    }

    /// Asserts that a certain condition is met.
    /// The "matched" and "rest" parts are passed by reference.
    pub fn assert<F>(self, f: F) -> Self
    where
        F: FnOnce(&Option<T>, &U) -> bool,
    {
        if let Some(rest) = &self.rest {
            if f(&self.matched, rest) {
                self
            } else {
                Self::failed()
            }
        } else {
            self
        }
    }

    /// Executes the passed function, if matching hasn't failed.
    /// The "matched" and "rest" parts are passed by reference.
    /// This method returns the match result unchanged.
    pub fn execute<F>(self, f: F) -> Self
    where
        F: FnOnce(&Option<T>, &U),
    {
        if let Some(rest) = &self.rest {
            f(&self.matched, rest);

            self
        } else {
            self
        }
    }

    /// Analogue to the `and_then` method but retains the original match index and value while returning a new "rest" part.
    pub fn discarding<F, R>(mut self, f: F) -> Self
    where
        T: Clone,
        F: FnOnce(Option<T>, U) -> R,
        R: Into<Self>,
    {
        if let Some(rest) = self.rest {
            self.rest = f(self.matched.clone(), rest).into().rest;

            if self.is_failed() {
                Self::failed()
            } else {
                self
            }
        } else {
            Self::failed()
        }
    }

    /// Analogue to the `discarding` method but the "matched" part is passed by reference.
    pub fn discarding_ref<F, R>(mut self, f: F) -> Self
    where
        F: FnOnce(&Option<T>, U) -> R,
        R: Into<Self>,
    {
        if let Some(rest) = self.rest {
            self.rest = f(&self.matched, rest).into().rest;

            self
        } else {
            Self::failed()
        }
    }

    /// Creates a new alternatives tree with same types as the match result itself.
    pub fn alternatives(self) -> AlternativesMatch<T, U, T, U> {
        AlternativesMatch::new(self)
    }

    /// Creates a new alternatives tree with different types from the match result itself.
    pub fn transforming_alternatives<V, W>(self) -> AlternativesMatch<T, U, V, W> {
        AlternativesMatch::new(self)
    }

    /// Converts current match into a sequence one.
    /// # Notes
    /// This functionality is available only with the `std` feature.
    #[cfg(feature = "std")]
    pub fn into_collecting(self) -> CollectingMatch<T, U>
    where
        T: Clone,
    {
        CollectingMatch::from(self)
    }
}

impl<E, T, M, R, U, V> MatchStatic<E, T, M, R> for Match<U, V>
where
    V: MatchStatic<E, T, M, R>,
{
    fn match_static(self, pattern: T) -> Match<M, R> {
        if let Some(rest) = self.rest {
            rest.match_static(pattern)
        } else {
            Match::failed()
        }
    }
}

impl<F, M, R, H, U, V> MatchWith<F, M, R, H> for Match<U, V>
where
    V: MatchWith<F, M, R, H>,
{
    fn match_with(self, pattern: F) -> Match<M, R> {
        if let Some(rest) = self.rest {
            rest.match_with(pattern)
        } else {
            Match::failed()
        }
    }
}

impl<N, F, M, R, H1, H2, U, V> MatchWithInRange<N, F, M, R, H1, H2> for Match<U, V>
where
    V: MatchWithInRange<N, F, M, R, H1, H2>,
{
    fn match_min_with(self, minimum: N, pattern: F) -> Match<M, R> {
        if let Some(rest) = self.rest {
            rest.match_min_with(minimum, pattern)
        } else {
            Match::failed()
        }
    }

    fn match_max_with(self, maximum: N, pattern: F) -> Match<M, R> {
        if let Some(rest) = self.rest {
            rest.match_max_with(maximum, pattern)
        } else {
            Match::failed()
        }
    }

    fn match_min_max_with(self, minimum: N, maximum: N, pattern: F) -> Match<M, R> {
        if let Some(rest) = self.rest {
            rest.match_min_max_with(minimum, maximum, pattern)
        } else {
            Match::failed()
        }
    }

    fn match_exact_with(self, count: N, pattern: F) -> Match<M, R> {
        if let Some(rest) = self.rest {
            rest.match_exact_with(count, pattern)
        } else {
            Match::failed()
        }
    }
}

/// Abstracts over match results while collecting them in a `Vec`.
/// # Notes
/// This functionality is available only with the `std` feature.
#[cfg(feature = "std")]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[must_use]
pub struct CollectingMatch<T, U> {
    matches: Vec<T>,
    rest: Option<U>,
}

#[cfg(feature = "std")]
impl<T, U> CollectingMatch<T, U> {
    /// Constructs a new "failed" instance.
    pub fn failed() -> Self {
        Self {
            matches: Vec::new(),
            rest: None,
        }
    }

    /// Returns boolean indicating whether the pattern was matched.
    /// This returns true when the pattern didn't match.
    pub fn is_failed(&self) -> bool {
        self.rest.is_none()
    }

    /// Calls to this method indicate that the sequence is completed and the final result should be returned.
    /// # Notes
    /// If any of the matches failed, then the whole sequence is considered failed.
    /// # Errors
    /// Returns `Err` when matching has failed.
    pub fn finalize(self) -> Result<(Vec<T>, U), MatchFailed> {
        match self.rest {
            Some(rest) => Ok((self.matches, rest)),
            None => Err(MatchFailed(())),
        }
    }

    /// Returns inner state.
    /// This is a short-hand for `finalize().unwrap()`.
    /// # Panics
    /// This function panics, if the `is_failed` function indicates an "failed" one.
    #[cfg_attr(not(feature = "no_track_caller"), track_caller)]
    pub fn unwrap(self) -> (Vec<T>, U) {
        self.finalize().unwrap()
    }

    /// Returns inner state.
    /// This is a short-hand for `finalize().expect("...")`.
    /// # Panics
    /// This function panics, if the `is_failed` function indicates an "failed" one.
    #[cfg_attr(not(feature = "no_track_caller"), track_caller)]
    pub fn expect(self, msg: &str) -> (Vec<T>, U) {
        self.finalize().expect(msg)
    }

    /// Asserts that a certain condition is met.
    /// The "matched" and "rest" parts are passed by reference while the index is passed by value.
    pub fn assert<F>(self, f: F) -> Self
    where
        F: FnOnce(Option<&T>, &U) -> bool,
    {
        if let Some(rest) = &self.rest {
            if f(self.matches.last(), rest) {
                self
            } else {
                Self::failed()
            }
        } else {
            Self::failed()
        }
    }

    /// Executes the passed function, if matching hasn't failed.
    /// The "matched" and "rest" parts are passed by reference while the index is passed by value.
    pub fn execute<F>(self, f: F) -> Self
    where
        F: FnOnce(Option<&T>, &U),
    {
        if let Some(rest) = &self.rest {
            f(self.matches.last(), rest);

            self
        } else {
            Self::failed()
        }
    }

    /// Executes the matching function once
    pub fn single<F, R>(mut self, f: F) -> Self
    where
        F: FnOnce(Option<&T>, U) -> R,
        R: Into<Match<T, U>>,
    {
        if let Some(rest) = self.rest {
            let result: Match<T, U> = f(self.matches.last(), rest).into();

            if result.is_failed() {
                Self::failed()
            } else {
                self.rest = result.rest;

                if let Some(matched) = result.matched {
                    self.matches.push(matched);
                }

                self
            }
        } else {
            Self::failed()
        }
    }

    /// Executes the matching function `count` times unless matching has failed.
    pub fn repeat<N, F, R>(mut self, mut count: N, mut f: F) -> Self
    where
        N: PartialEq<usize> + core::ops::SubAssign<usize>,
        F: FnMut(Option<&T>, U) -> R,
        R: Into<Match<T, U>>,
    {
        loop {
            if let Some(rest) = self.rest {
                if count == 0 {
                    self.rest = Some(rest);

                    break self;
                }

                self.rest = f(self.matches.last(), rest).into().rest;

                count -= 1;
            } else {
                break Self::failed();
            }
        }
    }

    /// Analogue to the `and_then` method but retains the original match index and value while returning a new "rest" part.
    pub fn discarding<F, R>(mut self, f: F) -> Self
    where
        F: FnOnce(Option<&T>, U) -> R,
        R: Into<Match<T, U>>,
    {
        if let Some(rest) = self.rest {
            self.rest = f(self.matches.last(), rest).into().rest;

            self
        } else {
            Self::failed()
        }
    }
}

#[cfg(feature = "std")]
impl<T, U> From<U> for CollectingMatch<T, U> {
    fn from(rest: U) -> Self {
        Self {
            matches: Vec::new(),
            rest: Some(rest),
        }
    }
}

#[cfg(feature = "std")]
impl<T, U> From<Match<T, U>> for CollectingMatch<T, U> {
    fn from(matched: Match<T, U>) -> Self {
        Self {
            matches: if let Some(matched) = matched.matched {
                vec![matched]
            } else {
                Vec::new()
            },
            rest: matched.rest,
        }
    }
}

/// Represents alternatives matching tree.
/// ## Notes
/// When one of the matching branches does *not* fail, all the rest will be skipped as this structure short-circuits when a matching branch is successful.
pub struct AlternativesMatch<T, U, V, W> {
    previous: Match<T, U>,
    matched: Match<V, W>,
}

impl<T, U, V, W> AlternativesMatch<T, U, V, W> {
    /// Creates new instance.
    #[must_use]
    pub const fn new(previous: Match<T, U>) -> Self {
        Self {
            previous,
            matched: Match::failed(),
        }
    }

    /// Returns true whenever any of the already defined branches has matched.
    pub fn is_matched(&self) -> bool {
        !self.matched.is_failed()
    }

    /// Adds a separate matching branch.
    pub fn add_path<F, R>(mut self, f: F) -> Self
    where
        Match<T, U>: Clone,
        F: FnOnce(Match<T, U>) -> R,
        R: Into<Match<V, W>>,
    {
        if self.matched.is_failed() {
            self.matched = f(self.previous.clone()).into();
        }

        self
    }

    /// Adds a separate matching branch by passing the match by reference.
    pub fn add_path_ref<F, R>(mut self, f: F) -> Self
    where
        F: FnOnce(&Match<T, U>) -> R,
        R: Into<Match<V, W>>,
    {
        if self.matched.is_failed() {
            self.matched = f(&self.previous).into();
        }

        self
    }

    /// Merges branches back into a linear match result.
    pub fn finalize(self) -> Match<V, W> {
        self.matched
    }
}
