//! This module holds all structures used to represend matching results.

use crate::traits::{MatchFail, MatchStatic, MatchWith, MatchWithInRange};

/// Represents failed pattern matching result.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct MatchFailed;

/// Representing the new state of the matching result after transforming.
pub enum TransformMatch<T, U> {
    /// Indicates the new state of the matching result is a failure.
    Failed,
    /// Indicates the new state of the matching result consists only of a "rest" part.
    OnlyRest(U),
    /// Indicates the new state of the matching result consists of both "matched" and "rest" parts.
    Full(T, U),
}

/// Representing the new state of the mapped matching result after transforming.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TransformMappedMatch<T, U, V> {
    /// Indicates the new state of the matching result is a failure.
    Failed,
    /// Indicates the new state of the matching result consists only of a "rest" part.
    OnlyRest(U),
    /// Indicates the new state of the matching result consists of all, "matched", "mapped" and "rest", parts.
    Full(T, U, V),
}

impl<T, U> From<TransformMatch<T, U>> for Match<T, U> {
    fn from(data: TransformMatch<T, U>) -> Self {
        use TransformMatch::{Failed, Full, OnlyRest};

        match data {
            Failed => Self::failed(),
            OnlyRest(rest) => Self::new(None, rest),
            Full(matched, rest) => Self::new(Some(matched), rest),
        }
    }
}

impl<T, U, V> From<TransformMappedMatch<T, U, V>> for MappedMatch<T, U, V> {
    fn from(data: TransformMappedMatch<T, U, V>) -> Self {
        use TransformMappedMatch::{Failed, Full, OnlyRest};

        match data {
            Failed => Self::failed(),
            OnlyRest(rest) => Self::new(None, rest),
            Full(matched, rest, mapped) => Self::new(Some((matched, mapped)), rest),
        }
    }
}

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
            Err(MatchFailed)
        }
    }

    /// Returns a reference to the "matched" part.
    pub fn matched(&self) -> Option<&T> {
        self.matched.as_ref()
    }

    /// Returns a reference to the "matched" part.
    pub fn rest(&self) -> Option<&U> {
        self.rest.as_ref()
    }

    /// Tranforms the whole matching result using the passed function.
    /// ## Notes
    /// The passed function gets executed only when matching has not failed.
    /// In contrast to the `transform_full` method, this method does not require "matched" part.
    pub fn transform<F, M, R>(self, f: F) -> Match<M, R>
    where
        F: FnOnce(Option<T>, U) -> TransformMatch<M, R>,
    {
        if let Some(rest) = self.rest {
            f(self.matched, rest).into()
        } else {
            Match::failed()
        }
    }

    /// Tranforms the whole matching result using the passed function.
    /// ## Notes
    /// The passed function gets executed only when matching has not failed and has "matched" part.
    /// If it doesn't have "matched" part or matching has failed, this function forwards a failed matching result.
    pub fn transform_full<F, M, R>(self, f: F) -> Match<M, R>
    where
        F: FnOnce(T, U) -> TransformMatch<M, R>,
    {
        if let Some((matched, rest)) = self.matched.zip(self.rest) {
            f(matched, rest).into()
        } else {
            Match::failed()
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

    /// Converts the matching result into a mapped one.
    pub fn map<V>(self, value: V) -> MappedMatch<T, U, V> {
        match self.rest {
            Some(rest) => MappedMatch::new(self.matched.zip(Some(value)), rest),
            None => MappedMatch::failed(),
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

    /// Keeps the original "matched" part and value while assigning the new "rest" part.
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

    /// If matching fails, the failure is ignored and the original one is forwarded. Otherwise, the matching result is forwarded.
    pub fn optional<F, R>(self, f: F) -> Self
    where
        T: Clone,
        U: Clone,
        F: FnOnce(Option<T>, U) -> R,
        R: Into<Self>,
    {
        if let Some(rest) = self.rest.clone() {
            let result: Self = f(self.matched.clone(), rest).into();

            if result.is_failed() {
                self
            } else {
                result
            }
        } else {
            Self::failed()
        }
    }

    /// Analogue to the `optional` method but the "matched" part is passed by reference.
    pub fn optional_ref<F, R>(self, f: F) -> Self
    where
        F: FnOnce(&Option<T>, &U) -> R,
        R: Into<Self>,
    {
        if let Some(rest) = &self.rest {
            let result: Self = f(&self.matched, rest).into();

            if result.is_failed() {
                self
            } else {
                result
            }
        } else {
            Self::failed()
        }
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

impl<T, U> MatchFail for Match<T, U> {
    fn failed() -> Self {
        Self {
            matched: None,
            rest: None,
        }
    }
}

impl<E, T, R, U, V> MatchStatic<E, T, R> for Match<U, V>
where
    R: MatchFail,
    V: MatchStatic<E, T, R>,
{
    fn match_static(self, pattern: T) -> R {
        if let Some(rest) = self.rest {
            rest.match_static(pattern)
        } else {
            R::failed()
        }
    }
}

impl<F, R, H, U, V> MatchWith<F, R, H> for Match<U, V>
where
    R: MatchFail,
    V: MatchWith<F, R, H>,
{
    fn match_with(self, pattern: F) -> R {
        if let Some(rest) = self.rest {
            rest.match_with(pattern)
        } else {
            R::failed()
        }
    }
}

impl<N, F, R, H1, H2, U, V> MatchWithInRange<N, F, R, H1, H2> for Match<U, V>
where
    R: MatchFail,
    V: MatchWithInRange<N, F, R, H1, H2>,
{
    fn match_min_with(self, minimum: N, pattern: F) -> R {
        if let Some(rest) = self.rest {
            rest.match_min_with(minimum, pattern)
        } else {
            R::failed()
        }
    }

    fn match_max_with(self, maximum: N, pattern: F) -> R {
        if let Some(rest) = self.rest {
            rest.match_max_with(maximum, pattern)
        } else {
            R::failed()
        }
    }

    fn match_min_max_with(self, minimum: N, maximum: N, pattern: F) -> R {
        if let Some(rest) = self.rest {
            rest.match_min_max_with(minimum, maximum, pattern)
        } else {
            R::failed()
        }
    }

    fn match_exact_with(self, count: N, pattern: F) -> R {
        if let Some(rest) = self.rest {
            rest.match_exact_with(count, pattern)
        } else {
            R::failed()
        }
    }
}

/// Generic type that holds result of pattern matching with a value mapped to it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[must_use]
pub struct MappedMatch<T, U, V> {
    matched: Option<(T, V)>,
    rest: Option<U>,
}

impl<T, U, V> MappedMatch<T, U, V> {
    /// Constructs a new instance.
    pub const fn new(matched: Option<(T, V)>, rest: U) -> Self {
        Self {
            matched,
            rest: Some(rest),
        }
    }

    /// Returns boolean indicating whether the pattern was matched.
    /// This returns true when the pattern didn't match.
    pub const fn is_failed(&self) -> bool {
        matches!(self.rest, None)
    }

    /// Returns `(Option<_>(matched, mapped), rest)` wrapped in `Result`, consuming the object.
    /// # Errors
    /// Returns `Err` when matching has failed.
    pub fn take(self) -> Result<(Option<(T, V)>, U), MatchFailed> {
        if let Some(rest) = self.rest {
            Ok((self.matched, rest))
        } else {
            Err(MatchFailed)
        }
    }

    /// Returns a reference to the "matched" part.
    pub fn matched(&self) -> Option<&T> {
        if let Some((matched, _)) = &self.matched {
            Some(matched)
        } else {
            None
        }
    }

    /// Returns a reference to the "matched" part.
    pub fn rest(&self) -> Option<&U> {
        self.rest.as_ref()
    }

    /// Returns a reference to the "matched" part.
    pub fn mapped(&self) -> Option<&V> {
        if let Some((_, mapped)) = &self.matched {
            Some(mapped)
        } else {
            None
        }
    }

    /// Tranforms the whole matching result using the passed function.
    /// ## Notes
    /// The passed function gets executed only when matching has not failed.
    /// In contrast to the `transform_full` method, this method does not require "matched" and "mapped" parts.
    pub fn transform<F, M, R, Q>(self, f: F) -> MappedMatch<M, R, Q>
    where
        F: FnOnce(Option<(T, V)>, U) -> TransformMappedMatch<M, R, Q>,
    {
        if let Some(rest) = self.rest {
            f(self.matched, rest).into()
        } else {
            MappedMatch::failed()
        }
    }

    /// Tranforms the whole matching result using the passed function.
    /// ## Notes
    /// The passed function gets executed only when matching has not failed and has "matched" and "mapped" parts.
    /// If it doesn't have "matched" and "mapped" parts or matching has failed, this function forwards a failed matching result.
    pub fn transform_full<F, M, R, Q>(self, f: F) -> MappedMatch<M, R, Q>
    where
        F: FnOnce(T, U, V) -> TransformMappedMatch<M, R, Q>,
    {
        if let Some(((matched, mapped), rest)) = self.matched.zip(self.rest) {
            f(matched, rest, mapped).into()
        } else {
            MappedMatch::failed()
        }
    }

    /// Transforms the "matched" part using the passed function.
    pub fn transform_matched<F, R>(self, f: F) -> MappedMatch<R, U, V>
    where
        F: FnOnce(T) -> R,
    {
        match (self.matched, self.rest) {
            (Some((matched, mapped)), Some(rest)) => {
                MappedMatch::new(Some((f(matched), mapped)), rest)
            }
            (None, Some(rest)) => MappedMatch::new(None, rest),
            (_, None) => MappedMatch::failed(),
        }
    }

    /// Transforms the "rest" part using the passed function.
    pub fn transform_rest<F, R>(self, f: F) -> MappedMatch<T, R, V>
    where
        F: FnOnce(U) -> R,
    {
        match (self.matched, self.rest) {
            (matched, Some(rest)) => MappedMatch::new(matched, f(rest)),
            (_, None) => MappedMatch::failed(),
        }
    }

    /// Transforms the "mapped" part using the passed function.
    pub fn transform_mapped<F, R>(self, f: F) -> MappedMatch<T, U, R>
    where
        F: FnOnce(V) -> R,
    {
        match (self.matched, self.rest) {
            (Some((matched, mapped)), Some(rest)) => {
                MappedMatch::new(Some((matched, f(mapped))), rest)
            }
            (None, Some(rest)) => MappedMatch::new(None, rest),
            (_, None) => MappedMatch::failed(),
        }
    }

    /// Separates the match result and it's mapped value, returning `Match` and calling the passed function with the mapped value
    /// in case the matching is successful.
    pub fn unmap<F>(self, f: F) -> Match<T, U>
    where
        F: FnOnce(V),
    {
        match (self.matched, self.rest) {
            (Some((matched, mapped)), Some(rest)) => {
                f(mapped);
                Match::new(Some(matched), rest)
            }
            (None, Some(rest)) => Match::new(None, rest),
            (_, None) => Match::failed(),
        }
    }

    /// Clears match result's "matched" and "mapped" parts.
    pub fn clear(mut self) -> Self {
        self.matched = None;
        self
    }

    /// Returns inner state.
    /// This is a short-hand for `to_successful().unwrap()`.
    /// # Panics
    /// This function panics, if the `is_failed` function indicates an "failed" one.
    #[cfg_attr(not(feature = "no_track_caller"), track_caller)]
    pub fn unwrap(self) -> (Option<(T, V)>, U) {
        (self.matched, self.rest.unwrap())
    }

    /// Returns inner state.
    /// This is a short-hand for `to_successful().expect("...")`.
    /// # Panics
    /// This function panics, if the `is_failed` function indicates an "failed" one.
    #[cfg_attr(not(feature = "no_track_caller"), track_caller)]
    pub fn expect(self, msg: &str) -> (Option<(T, V)>, U) {
        (self.matched, self.rest.expect(msg))
    }

    /// Asserts that a certain condition is met.
    /// The "matched" and "rest" parts are passed by reference.
    pub fn assert<F>(self, f: F) -> Self
    where
        F: FnOnce(&Option<(T, V)>, &U) -> bool,
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
        F: FnOnce(&Option<(T, V)>, &U),
    {
        if let Some(rest) = &self.rest {
            f(&self.matched, rest);

            self
        } else {
            self
        }
    }

    /// Keeps the original "matched" and "matched" parts and value while assigning the new "rest" part.
    pub fn discarding<F, R>(mut self, f: F) -> Self
    where
        (T, V): Clone,
        F: FnOnce(Option<(T, V)>, U) -> R,
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

    /// Analogue to the `discarding` method but the "matched" and "mapped" part is passed by reference.
    pub fn discarding_ref<F, R>(mut self, f: F) -> Self
    where
        F: FnOnce(&Option<(T, V)>, U) -> R,
        R: Into<Self>,
    {
        if let Some(rest) = self.rest {
            self.rest = f(&self.matched, rest).into().rest;

            self
        } else {
            Self::failed()
        }
    }

    /// If matching fails, the failure is ignored and the original one is forwarded. Otherwise, the matching result is forwarded.
    pub fn optional<F, R>(self, f: F) -> Self
    where
        (T, V): Clone,
        U: Clone,
        F: FnOnce(Option<(T, V)>, U) -> R,
        R: Into<Self>,
    {
        if let Some(rest) = self.rest.clone() {
            let result: Self = f(self.matched.clone(), rest).into();

            if result.is_failed() {
                self
            } else {
                result
            }
        } else {
            Self::failed()
        }
    }

    /// Analogue to the `optional` method but the "matched" and "mapped" parts are passed by reference.
    pub fn optional_ref<F, R>(self, f: F) -> Self
    where
        F: FnOnce(&Option<(T, V)>, &U) -> R,
        R: Into<Self>,
    {
        if let Some(rest) = &self.rest {
            let result: Self = f(&self.matched, rest).into();

            if result.is_failed() {
                self
            } else {
                result
            }
        } else {
            Self::failed()
        }
    }

    /// Converts current match into a sequence one.
    /// # Notes
    /// This functionality is available only with the `std` feature.
    #[cfg(feature = "std")]
    pub fn into_collecting<F>(self, f: F) -> CollectingMatch<T, U>
    where
        T: Clone,
        F: FnOnce(V),
    {
        CollectingMatch::from(self.unmap(f))
    }
}

impl<T, U, V> MatchFail for MappedMatch<T, U, V> {
    fn failed() -> Self {
        Self {
            matched: None,
            rest: None,
        }
    }
}

impl<E, T, R, U, V, Q> MatchStatic<E, T, R> for MappedMatch<U, V, Q>
where
    R: MatchFail,
    V: MatchStatic<E, T, R>,
{
    fn match_static(self, pattern: T) -> R {
        if let Some(rest) = self.rest {
            rest.match_static(pattern)
        } else {
            R::failed()
        }
    }
}

impl<F, R, H, U, V, Q> MatchWith<F, R, H> for MappedMatch<U, V, Q>
where
    R: MatchFail,
    V: MatchWith<F, R, H>,
{
    fn match_with(self, pattern: F) -> R {
        if let Some(rest) = self.rest {
            rest.match_with(pattern)
        } else {
            R::failed()
        }
    }
}

impl<N, F, R, H1, H2, U, V, Q> MatchWithInRange<N, F, R, H1, H2> for MappedMatch<U, V, Q>
where
    R: MatchFail,
    V: MatchWithInRange<N, F, R, H1, H2>,
{
    fn match_min_with(self, minimum: N, pattern: F) -> R {
        if let Some(rest) = self.rest {
            rest.match_min_with(minimum, pattern)
        } else {
            R::failed()
        }
    }

    fn match_max_with(self, maximum: N, pattern: F) -> R {
        if let Some(rest) = self.rest {
            rest.match_max_with(maximum, pattern)
        } else {
            R::failed()
        }
    }

    fn match_min_max_with(self, minimum: N, maximum: N, pattern: F) -> R {
        if let Some(rest) = self.rest {
            rest.match_min_max_with(minimum, maximum, pattern)
        } else {
            R::failed()
        }
    }

    fn match_exact_with(self, count: N, pattern: F) -> R {
        if let Some(rest) = self.rest {
            rest.match_exact_with(count, pattern)
        } else {
            R::failed()
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
            None => Err(MatchFailed),
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

    /// Discards the result of the matching while keeping only the "rest" part.
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

    /// If matching fails, the failure is ignored and the original one is forwarded. Otherwise, the matching result is forwarded.
    pub fn optional<F, R>(self, f: F) -> Self
    where
        U: Clone,
        F: FnOnce(Option<&T>, U) -> R,
        R: Into<Self>,
    {
        if let Some(rest) = self.rest.clone() {
            let result: Self = f(self.matches.last(), rest).into();

            if result.is_failed() {
                self
            } else {
                result
            }
        } else {
            Self::failed()
        }
    }

    /// Analogue to the `optional` method but the "rest" part is passed by reference.
    pub fn optional_ref<F, R>(self, f: F) -> Self
    where
        F: FnOnce(Option<&T>, &U) -> R,
        R: Into<Self>,
    {
        if let Some(rest) = &self.rest {
            let result: Self = f(self.matches.last(), rest).into();

            if result.is_failed() {
                self
            } else {
                result
            }
        } else {
            Self::failed()
        }
    }
}

#[cfg(feature = "std")]
impl<T, U> MatchFail for CollectingMatch<T, U> {
    fn failed() -> Self {
        Self {
            matches: Vec::new(),
            rest: None,
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
#[must_use]
pub struct AlternativesMatch<T, U, V> {
    previous: T,
    matched: Match<U, V>,
}

impl<T, U, V> AlternativesMatch<T, U, V> {
    /// Creates new instance.
    pub const fn new(previous: T) -> Self {
        Self {
            previous,
            matched: Match {
                matched: None,
                rest: None,
            },
        }
    }

    /// Returns true whenever any of the already defined branches has matched.
    pub fn is_matched(&self) -> bool {
        !self.matched.is_failed()
    }

    /// Adds a separate matching branch.
    pub fn add_path<F, R>(mut self, f: F) -> Self
    where
        T: Clone,
        F: FnOnce(T) -> R,
        R: Into<Match<U, V>>,
    {
        if self.matched.is_failed() {
            self.matched = f(self.previous.clone()).into();
        }

        self
    }

    /// Adds a separate matching branch by passing the match by reference.
    pub fn add_path_ref<F, R>(mut self, f: F) -> Self
    where
        F: FnOnce(&T) -> R,
        R: Into<Match<U, V>>,
    {
        if self.matched.is_failed() {
            self.matched = f(&self.previous).into();
        }

        self
    }

    /// Merges branches back into a linear match result.
    pub fn finalize(self) -> Match<U, V> {
        self.matched
    }
}

/// Represents alternatives matching tree.
/// ## Notes
/// When one of the matching branches does *not* fail, all the rest will be skipped as this structure short-circuits when a matching branch is successful.
#[must_use]
pub struct MappedAlternativesMatch<T, U, V, W> {
    previous: T,
    matched: MappedMatch<U, V, W>,
}

impl<T, U, V, W> MappedAlternativesMatch<T, U, V, W> {
    /// Creates new instance.
    pub const fn new(previous: T) -> Self {
        Self {
            previous,
            matched: MappedMatch {
                matched: None,
                rest: None,
            },
        }
    }

    /// Returns true whenever any of the already defined branches has matched.
    pub fn is_matched(&self) -> bool {
        !self.matched.is_failed()
    }

    /// Adds a separate matching branch.
    pub fn add_path<F, R>(mut self, f: F) -> Self
    where
        T: Clone,
        F: FnOnce(T) -> R,
        R: Into<MappedMatch<U, V, W>>,
    {
        if self.matched.is_failed() {
            self.matched = f(self.previous.clone()).into();
        }

        self
    }

    /// Adds a separate matching branch by passing the match by reference.
    pub fn add_path_ref<F, R>(mut self, f: F) -> Self
    where
        F: FnOnce(&T) -> R,
        R: Into<MappedMatch<U, V, W>>,
    {
        if self.matched.is_failed() {
            self.matched = f(&self.previous).into();
        }

        self
    }

    /// Merges branches back into a linear match result.
    pub fn finalize(self) -> MappedMatch<U, V, W> {
        self.matched
    }
}
