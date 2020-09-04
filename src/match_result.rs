use crate::{MatchStatic, MatchStaticMultiple, MatchWith, MatchWithInRange};

/// Represents failed pattern matching result.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct MatchFailed(());

/// Generic type that holds result of pattern matching.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[must_use]
pub struct SuccessfulMatch<T> {
    index: usize,
    matched: T,
    rest: T,
}

impl<T> SuccessfulMatch<T> {
    /// Constructs a new instance.
    pub const fn new(index: usize, matched: T, rest: T) -> Self {
        Self {
            index,
            matched,
            rest,
        }
    }

    /// Returns the index of the matched pattern.
    pub const fn index(&self) -> usize {
        self.index
    }

    pub(crate) fn set_index(&mut self, index: usize) {
        self.index = index;
    }

    /// Returns reference to the matched part of the result.
    pub const fn matched(&self) -> &T {
        &self.matched
    }

    /// Returns reference to the part that is left of the result.
    pub const fn rest(&self) -> &T {
        &self.rest
    }

    /// Returns `(index, matched, rest)` tuple, consuming the object.
    pub fn take(self) -> (usize, T, T) {
        (self.index, self.matched, self.rest)
    }

    /// Asserts that a certain condition is met.
    /// The "matched" and "rest" parts are passed by reference while the index is passed by value.
    pub fn assert<F>(self, f: F) -> Match<T>
    where
        F: FnOnce(usize, &T, &T) -> bool,
    {
        if f(self.index, self.matched(), self.rest()) {
            self.into()
        } else {
            Match::failed()
        }
    }

    /// Executes the passed function unconditionally.
    /// The "matched" and "rest" parts are passed by reference while the index is passed by value.
    pub fn execute<F>(self, f: F) -> Self
    where
        for<'context> F: FnOnce(usize, &'context T, &'context T),
    {
        f(self.index, self.matched(), self.rest());

        self
    }

    /// Analogue to the `and_then` method but retains the original match index and value while returning a new "rest" part.
    pub fn discarding<F, R>(self, f: F) -> Match<T>
    where
        T: Clone,
        F: FnOnce(usize, T, T) -> R,
        R: Into<Match<T>>,
    {
        if let Ok((_, _, rest)) =
            Into::<Match<T>>::into(f(self.index, self.matched.clone(), self.rest)).take()
        {
            Self::new(self.index, self.matched, rest).into()
        } else {
            Match::failed()
        }
    }

    /// Non-failing variant of the `discarding` method.
    pub fn discarding_non_failing<F, R>(self, f: F) -> Self
    where
        T: Clone,
        F: FnOnce(usize, T, T) -> R,
        R: Into<Self>,
    {
        let rest: T = Into::<Self>::into(f(self.index, self.matched.clone(), self.rest)).rest;

        Self::new(self.index, self.matched, rest)
    }

    /// Analogue to the `discarding` method but the "matched" part is passed by reference.
    pub fn discarding_ref<F, R>(self, f: F) -> Match<T>
    where
        F: FnOnce(usize, &T, T) -> R,
        R: Into<Match<T>>,
    {
        if let Ok((_, _, rest)) =
            Into::<Match<T>>::into(f(self.index, &self.matched, self.rest)).take()
        {
            Self::new(self.index, self.matched, rest).into()
        } else {
            Match::failed()
        }
    }

    /// Non-failing variant of the `discarding_ref` method.
    pub fn discarding_ref_non_failing<F, R>(self, f: F) -> Self
    where
        F: FnOnce(usize, &T, T) -> R,
        R: Into<Self>,
    {
        let rest: T = Into::<Self>::into(f(self.index, &self.matched, self.rest)).rest;

        Self::new(self.index, self.matched, rest)
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

impl<'object, E, T, R, U> MatchStatic<'object, E, T, R> for SuccessfulMatch<U>
where
    U: MatchStatic<'object, E, T, R>,
{
    fn match_static(&'object self, pattern: T) -> Match<R> {
        self.rest.match_static(pattern)
    }
}

impl<'object, E, T, R, U> MatchStaticMultiple<'object, E, T, R> for SuccessfulMatch<U>
where
    U: MatchStaticMultiple<'object, E, T, R>,
{
    fn match_static_multiple(&'object self, pattern: T) -> Match<R> {
        self.rest.match_static_multiple(pattern)
    }
}

impl<'object, E, F, R, U> MatchWith<'object, E, F, R> for SuccessfulMatch<U>
where
    U: MatchWith<'object, E, F, R>,
{
    fn match_with(&'object self, pattern: F) -> Match<R> {
        self.rest.match_with(pattern)
    }
}

impl<'object, E, N, F, R, U> MatchWithInRange<'object, E, N, F, R> for SuccessfulMatch<U>
where
    U: MatchWithInRange<'object, E, N, F, R>,
{
    fn match_min_with(&'object self, minimum: N, pattern: F) -> Match<R> {
        self.rest.match_min_with(minimum, pattern)
    }

    fn match_max_with(&'object self, maximum: N, pattern: F) -> Match<R> {
        self.rest.match_max_with(maximum, pattern)
    }

    fn match_min_max_with(&'object self, minimum: N, maximum: N, pattern: F) -> Match<R> {
        self.rest.match_min_max_with(minimum, maximum, pattern)
    }

    fn match_exact_with(&'object self, count: N, pattern: F) -> Match<R> {
        self.rest.match_exact_with(count, pattern)
    }
}

/// Generic type that holds result of pattern matching.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[must_use]
pub struct Match<T> {
    matched: Option<SuccessfulMatch<T>>,
}

impl<T> Match<T> {
    /// Constructs a new instance.
    pub fn new(matched: Result<SuccessfulMatch<T>, MatchFailed>) -> Self {
        Self {
            matched: matched.ok(),
        }
    }

    /// Constructs a new "failed" instance.
    pub fn failed() -> Self {
        Self { matched: None }
    }

    /// Returns boolean indicating whether the pattern was matched.
    /// This returns true when the pattern didn't match.
    pub fn is_failed(&self) -> bool {
        self.matched.is_none()
    }

    /// Returns inner state.
    /// # Errors
    /// Returns `Err` when matching has failed.
    pub fn into_successful(self) -> Result<SuccessfulMatch<T>, MatchFailed> {
        if let Some(matched) = self.matched {
            Ok(matched)
        } else {
            Err(MatchFailed(()))
        }
    }

    /// Returns inner state.
    /// This is non-consuming variant of `into_successful`.
    /// # Errors
    /// Returns `Err` when matching has failed.
    pub fn to_successful(&self) -> Result<SuccessfulMatch<T>, MatchFailed>
    where
        T: Clone,
    {
        if let Some(matched) = &self.matched {
            Ok(matched.clone())
        } else {
            Err(MatchFailed(()))
        }
    }

    /// Returns `(usize, matched, rest)` wrapped in `Result`, consuming the object.
    /// # Errors
    /// Returns `Err` when matching has failed.
    pub fn take(self) -> Result<(usize, T, T), MatchFailed> {
        if let Some(matched) = self.matched {
            Ok((matched.index, matched.matched, matched.rest))
        } else {
            Err(MatchFailed(()))
        }
    }

    /// Returns inner state.
    /// This is a short-hand for `to_successful().unwrap()`.
    /// # Panics
    /// This function panics, if the `is_failed` function indicates an "failed" one.
    #[cfg_attr(not(feature = "no_track_caller"), track_caller)]
    pub fn unwrap(self) -> SuccessfulMatch<T> {
        self.matched.unwrap()
    }

    /// Returns inner state.
    /// This is a short-hand for `to_successful().expect("...")`.
    /// # Panics
    /// This function panics, if the `is_failed` function indicates an "failed" one.
    #[cfg_attr(not(feature = "no_track_caller"), track_caller)]
    pub fn expect(self, msg: &str) -> SuccessfulMatch<T> {
        self.matched.expect(msg)
    }

    /// Asserts that a certain condition is met.
    /// The "matched" and "rest" parts are passed by reference while the index is passed by value.
    pub fn assert<F>(self, f: F) -> Self
    where
        for<'context> F: FnOnce(usize, &'context T, &'context T) -> bool,
    {
        if let Some(matched) = &self.matched {
            if f(matched.index, matched.matched(), matched.rest()) {
                self
            } else {
                Self::failed()
            }
        } else {
            self
        }
    }

    /// Executes the passed function, if matching hasn't failed.
    /// The "matched" and "rest" parts are passed by reference while the index is passed by value.
    /// This method returns the match result unchanged.
    pub fn execute<F>(self, f: F) -> Self
    where
        for<'context> F: FnOnce(usize, &'context T, &'context T),
    {
        if let Some(matched) = &self.matched {
            f(matched.index, matched.matched(), matched.rest());

            self
        } else {
            self
        }
    }

    /// Analogue to the `and_then` method but retains the original match index and value while returning a new "rest" part.
    pub fn discarding<F, R>(self, f: F) -> Self
    where
        T: Clone,
        F: FnOnce(usize, T, T) -> R,
        R: Into<Self>,
    {
        if let Ok(matched) = self.into_successful() {
            matched.discarding(f)
        } else {
            Self::failed()
        }
    }

    /// Analogue to the `discarding` method but the "matched" part is passed by reference.
    pub fn discarding_ref<F, R>(self, f: F) -> Self
    where
        F: FnOnce(usize, &T, T) -> R,
        R: Into<Self>,
    {
        if let Ok(matched) = self.into_successful() {
            matched.discarding_ref(f)
        } else {
            Self::failed()
        }
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

impl<'object, E, T, R, U> MatchStatic<'object, E, T, R> for Match<U>
where
    U: MatchStatic<'object, E, T, R>,
{
    fn match_static(&'object self, pattern: T) -> Match<R> {
        if let Some(matched) = &self.matched {
            matched.rest.match_static(pattern)
        } else {
            Match::failed()
        }
    }
}

impl<'object, E, T, R, U> MatchStaticMultiple<'object, E, T, R> for Match<U>
where
    U: MatchStaticMultiple<'object, E, T, R>,
{
    fn match_static_multiple(&'object self, pattern: T) -> Match<R> {
        if let Some(matched) = &self.matched {
            matched.rest.match_static_multiple(pattern)
        } else {
            Match::failed()
        }
    }
}

impl<'object, E, F, R, U> MatchWith<'object, E, F, R> for Match<U>
where
    U: MatchWith<'object, E, F, R>,
{
    fn match_with(&'object self, pattern: F) -> Match<R> {
        if let Some(matched) = &self.matched {
            matched.rest.match_with(pattern)
        } else {
            Match::failed()
        }
    }
}

impl<'object, E, N, F, R, U> MatchWithInRange<'object, E, N, F, R> for Match<U>
where
    U: MatchWithInRange<'object, E, N, F, R>,
{
    fn match_min_with(&'object self, minimum: N, pattern: F) -> Match<R> {
        if let Some(matched) = &self.matched {
            matched.rest.match_min_with(minimum, pattern)
        } else {
            Match::failed()
        }
    }

    fn match_max_with(&'object self, maximum: N, pattern: F) -> Match<R> {
        if let Some(matched) = &self.matched {
            matched.rest.match_max_with(maximum, pattern)
        } else {
            Match::failed()
        }
    }

    fn match_min_max_with(&'object self, minimum: N, maximum: N, pattern: F) -> Match<R> {
        if let Some(matched) = &self.matched {
            matched.rest.match_min_max_with(minimum, maximum, pattern)
        } else {
            Match::failed()
        }
    }

    fn match_exact_with(&'object self, count: N, pattern: F) -> Match<R> {
        if let Some(matched) = &self.matched {
            matched.rest.match_exact_with(count, pattern)
        } else {
            Match::failed()
        }
    }
}

impl<T> From<SuccessfulMatch<T>> for Match<T> {
    fn from(matched: SuccessfulMatch<T>) -> Self {
        Self {
            matched: Some(matched),
        }
    }
}

impl<'a, T> From<Result<SuccessfulMatch<T>, MatchFailed>> for Match<T> {
    fn from(matched: Result<SuccessfulMatch<T>, MatchFailed>) -> Self {
        Self {
            matched: matched.ok(),
        }
    }
}

/// Abstracts over match results while collecting them in a `Vec`.
/// # Notes
/// This functionality is available only with the `std` feature.
#[cfg(feature = "std")]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[must_use]
pub struct CollectingMatch<T>
where
    T: Clone,
{
    matches: Vec<(usize, T)>,
    last_match: Match<T>,
}

#[cfg(feature = "std")]
impl<T> CollectingMatch<T>
where
    T: Clone,
{
    /// Constructs a new "failed" instance.
    pub fn failed() -> Self {
        Self {
            matches: Vec::new(),
            last_match: Match::failed(),
        }
    }

    /// Returns boolean indicating whether the pattern was matched.
    /// This returns true when the pattern didn't match.
    pub fn is_failed(&self) -> bool {
        self.last_match.is_failed()
    }

    /// Calls to this method indicate that the sequence is completed and the final result should be returned.
    /// # Notes
    /// If any of the matches failed, then the whole sequence is considered failed.
    /// # Errors
    /// Returns `Err` when matching has failed.
    pub fn finalize(mut self) -> Result<(Vec<(usize, T)>, T), MatchFailed> {
        match self.last_match.into_successful() {
            Ok(matched) => {
                let (index, matched, rest): (usize, T, T) = matched.take();

                self.matches.push((index, matched));

                Ok((self.matches, rest))
            }
            Err(failed) => Err(failed),
        }
    }

    /// Returns inner state.
    /// This is a short-hand for `finalize().unwrap()`.
    /// # Panics
    /// This function panics, if the `is_failed` function indicates an "failed" one.
    #[cfg_attr(not(feature = "no_track_caller"), track_caller)]
    pub fn unwrap(self) -> (Vec<(usize, T)>, T) {
        self.finalize().unwrap()
    }

    /// Returns inner state.
    /// This is a short-hand for `finalize().expect("...")`.
    /// # Panics
    /// This function panics, if the `is_failed` function indicates an "failed" one.
    #[cfg_attr(not(feature = "no_track_caller"), track_caller)]
    pub fn expect(self, msg: &str) -> (Vec<(usize, T)>, T) {
        self.finalize().expect(msg)
    }

    /// Asserts that a certain condition is met.
    /// The "matched" and "rest" parts are passed by reference while the index is passed by value.
    pub fn assert<F>(mut self, f: F) -> Self
    where
        for<'context> F: FnOnce(usize, &'context T, &'context T) -> bool,
    {
        self.last_match = self.last_match.assert(f);

        if self.is_failed() {
            Self::failed()
        } else {
            self
        }
    }

    /// Executes the passed function, if matching hasn't failed.
    /// The "matched" and "rest" parts are passed by reference while the index is passed by value.
    pub fn execute<F>(mut self, f: F) -> Self
    where
        for<'context> F: FnOnce(usize, &'context T, &'context T),
    {
        self.last_match = self.last_match.execute(f);

        self
    }

    /// Executes the matching function once
    pub fn single<F, R>(mut self, f: F) -> Self
    where
        F: FnOnce(usize, T, T) -> R,
        R: Into<Match<T>>,
    {
        if let Some(matched) = self.last_match.matched {
            self.matches.push((matched.index, matched.matched.clone()));

            self.last_match = f(matched.index, matched.matched, matched.rest).into();

            self
        } else {
            self
        }
    }

    /// Executes the matching function `count` times unless matching has failed.
    pub fn repeat<N, F, R>(mut self, mut count: N, f: F) -> Self
    where
        N: PartialEq<usize> + core::ops::SubAssign<usize>,
        F: FnMut(usize, T, T) -> R + Clone,
        R: Into<Match<T>>,
    {
        loop {
            if count == 0 {
                break self;
            }

            if self.is_failed() {
                break self;
            }

            self = self.single(f.clone());

            count -= 1;
        }
    }

    /// Analogue to the `and_then` method but retains the original match index and value while returning a new "rest" part.
    pub fn discarding<F, R>(mut self, f: F) -> Self
    where
        T: Clone,
        F: FnOnce(usize, T, T) -> R,
        R: Into<Match<T>>,
    {
        self.last_match = self.last_match.discarding(f);

        self
    }

    /// Analogue to the `discarding` method but the "matched" part is passed by reference.
    pub fn discarding_ref<F, R>(mut self, f: F) -> Self
    where
        F: FnOnce(usize, &T, T) -> R,
        R: Into<Match<T>>,
    {
        self.last_match = self.last_match.discarding_ref(f);

        self
    }
}

#[cfg(feature = "std")]
impl<T> From<SuccessfulMatch<T>> for CollectingMatch<T>
where
    T: Clone,
{
    fn from(matched: SuccessfulMatch<T>) -> Self {
        Self {
            matches: Vec::new(),
            last_match: matched.into(),
        }
    }
}

#[cfg(feature = "std")]
impl<T> From<Result<SuccessfulMatch<T>, MatchFailed>> for CollectingMatch<T>
where
    T: Clone,
{
    fn from(matched: Result<SuccessfulMatch<T>, MatchFailed>) -> Self {
        Self {
            matches: Vec::new(),
            last_match: matched.into(),
        }
    }
}

#[cfg(feature = "std")]
impl<T> From<Match<T>> for CollectingMatch<T>
where
    T: Clone,
{
    fn from(matched: Match<T>) -> Self {
        Self {
            matches: Vec::new(),
            last_match: matched,
        }
    }
}
