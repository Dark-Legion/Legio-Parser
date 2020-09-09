use crate::{MatchStatic, MatchWith, MatchWithInRange};

/// Represents failed pattern matching result.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct MatchFailed(());

/// Generic type that holds result of pattern matching.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[must_use]
pub struct Match<T> {
    matched: Option<T>,
    rest: Option<T>,
}

impl<T> Match<T> {
    /// Constructs a new instance.
    pub const fn new(matched: Option<T>, rest: T) -> Self {
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
    pub fn take(self) -> Result<(Option<T>, T), MatchFailed> {
        if let Some(rest) = self.rest {
            Ok((self.matched, rest))
        } else {
            Err(MatchFailed(()))
        }
    }

    pub fn transform<F, R>(self, mut f: F) -> Match<R>
    where
        F: FnMut(T) -> R,
    {
        match (self.matched, self.rest) {
            (Some(matched), Some(rest)) => Match::new(Some(f(matched)), f(rest)),
            (None, Some(rest)) => Match::new(None, f(rest)),
            (_, None) => Match::failed(),
        }
    }

    /// Returns inner state.
    /// This is a short-hand for `to_successful().unwrap()`.
    /// # Panics
    /// This function panics, if the `is_failed` function indicates an "failed" one.
    #[cfg_attr(not(feature = "no_track_caller"), track_caller)]
    pub fn unwrap(self) -> (Option<T>, T) {
        (self.matched, self.rest.unwrap())
    }

    /// Returns inner state.
    /// This is a short-hand for `to_successful().expect("...")`.
    /// # Panics
    /// This function panics, if the `is_failed` function indicates an "failed" one.
    #[cfg_attr(not(feature = "no_track_caller"), track_caller)]
    pub fn expect(self, msg: &str) -> (Option<T>, T) {
        (self.matched, self.rest.expect(msg))
    }

    /// Asserts that a certain condition is met.
    /// The "matched" and "rest" parts are passed by reference.
    pub fn assert<F>(self, f: F) -> Self
    where
        F: FnOnce(&Option<T>, &T) -> bool,
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
        F: FnOnce(&Option<T>, &T),
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
        F: FnOnce(Option<T>, T) -> R,
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
        F: FnOnce(&Option<T>, T) -> R,
        R: Into<Self>,
    {
        if let Some(rest) = self.rest {
            self.rest = f(&self.matched, rest).into().rest;

            self
        } else {
            Self::failed()
        }
    }

    pub fn alternatives(self) -> AlternativesMatch<T, T> {
        AlternativesMatch::new(self)
    }

    /// Converts current match into a sequence one.
    /// # Notes
    /// This functionality is available only with the `std` feature.
    #[cfg(feature = "std")]
    pub fn into_collecting(self) -> CollectingMatch<T>
    where
        T: Clone,
    {
        CollectingMatch::from(self)
    }
}

impl<E, T, R, U> MatchStatic<E, T, R> for Match<U>
where
    U: MatchStatic<E, T, R>,
{
    fn match_static(self, pattern: T) -> Match<R> {
        if let Some(rest) = self.rest {
            rest.match_static(pattern)
        } else {
            Match::failed()
        }
    }
}

impl<E, F, R, U> MatchWith<E, F, R> for Match<U>
where
    U: MatchWith<E, F, R>,
    F: FnMut(E) -> bool,
{
    fn match_with(self, pattern: F) -> Match<R> {
        if let Some(rest) = self.rest {
            rest.match_with(pattern)
        } else {
            Match::failed()
        }
    }
}

impl<E, N, F, R, U> MatchWithInRange<E, N, F, R> for Match<U>
where
    U: MatchWithInRange<E, N, F, R>,
{
    fn match_min_with(self, minimum: N, pattern: F) -> Match<R> {
        if let Some(rest) = self.rest {
            rest.match_min_with(minimum, pattern)
        } else {
            Match::failed()
        }
    }

    fn match_max_with(self, maximum: N, pattern: F) -> Match<R> {
        if let Some(rest) = self.rest {
            rest.match_max_with(maximum, pattern)
        } else {
            Match::failed()
        }
    }

    fn match_min_max_with(self, minimum: N, maximum: N, pattern: F) -> Match<R> {
        if let Some(rest) = self.rest {
            rest.match_min_max_with(minimum, maximum, pattern)
        } else {
            Match::failed()
        }
    }

    fn match_exact_with(self, count: N, pattern: F) -> Match<R> {
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
pub struct CollectingMatch<T> {
    matches: Vec<T>,
    rest: Option<T>,
}

#[cfg(feature = "std")]
impl<T> CollectingMatch<T> {
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
    pub fn finalize(self) -> Result<(Vec<T>, T), MatchFailed> {
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
    pub fn unwrap(self) -> (Vec<T>, T) {
        self.finalize().unwrap()
    }

    /// Returns inner state.
    /// This is a short-hand for `finalize().expect("...")`.
    /// # Panics
    /// This function panics, if the `is_failed` function indicates an "failed" one.
    #[cfg_attr(not(feature = "no_track_caller"), track_caller)]
    pub fn expect(self, msg: &str) -> (Vec<T>, T) {
        self.finalize().expect(msg)
    }

    /// Asserts that a certain condition is met.
    /// The "matched" and "rest" parts are passed by reference while the index is passed by value.
    pub fn assert<F>(self, f: F) -> Self
    where
        F: FnOnce(Option<&T>, &T) -> bool,
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
        F: FnOnce(Option<&T>, &T),
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
        F: FnOnce(Option<&T>, T) -> R,
        R: Into<Match<T>>,
    {
        if let Some(rest) = self.rest {
            let result: Match<T> = f(self.matches.last(), rest).into();

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
        F: FnMut(Option<&T>, T) -> R,
        R: Into<Match<T>>,
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
        F: FnOnce(Option<&T>, T) -> R,
        R: Into<Match<T>>,
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
impl<T> From<T> for CollectingMatch<T> {
    fn from(rest: T) -> Self {
        Self {
            matches: Vec::new(),
            rest: Some(rest),
        }
    }
}

#[cfg(feature = "std")]
impl<T> From<Match<T>> for CollectingMatch<T> {
    fn from(matched: Match<T>) -> Self {
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

pub struct AlternativesMatch<T, U = T> {
    previous: Match<T>,
    matched: Match<U>,
}

impl<T, U> AlternativesMatch<T, U> {
    pub const fn new(previous: Match<T>) -> Self {
        Self {
            previous,
            matched: Match::failed(),
        }
    }

    pub fn is_matched(&self) -> bool {
        !self.matched.is_failed()
    }

    pub fn add_path<F, R>(mut self, f: F) -> Self
    where
        T: Clone,
        F: FnOnce(Match<T>) -> R,
        R: Into<Match<U>>,
    {
        if self.matched.is_failed() {
            self.matched = f(self.previous.clone()).into();
        }

        self
    }

    pub fn add_path_ref<F, R>(mut self, f: F) -> Self
    where
        F: FnOnce(&Match<T>) -> R,
        R: Into<Match<U>>,
    {
        if self.matched.is_failed() {
            self.matched = f(&self.previous).into();
        }

        self
    }

    pub fn finalize(self) -> Match<U> {
        self.matched
    }
}

impl<T, U> From<AlternativesMatch<T, U>> for Match<U> {
    fn from(matched: AlternativesMatch<T, U>) -> Self {
        matched.matched
    }
}
