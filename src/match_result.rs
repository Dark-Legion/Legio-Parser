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

    /// Analogue to `Option` and `Result`'s method `and_then`.
    pub fn and_then<F, R>(self, f: F) -> Match<T>
    where
        F: FnOnce(usize, T, T) -> R,
        R: Into<Match<T>>,
    {
        f(self.index, self.matched, self.rest).into()
    }

    /// Non-failing variant of the `and_then` method.
    pub fn and_then_non_failing<F, R>(self, f: F) -> Self
    where
        F: FnOnce(usize, T, T) -> R,
        R: Into<Self>,
    {
        f(self.index, self.matched, self.rest).into()
    }

    /// Analogue to the `and_then` method but adds a condition.
    /// When the condition is met, the progression function is executed.
    /// Otherwise, match fails.
    pub fn and_then_if<F1, F2, R>(self, condition: F1, f: F2) -> Match<T>
    where
        F1: FnOnce(usize, &T, &T) -> bool,
        F2: FnOnce(usize, T, T) -> R,
        R: Into<Match<T>>,
    {
        if condition(self.index, self.matched(), self.rest()) {
            f(self.index, self.matched, self.rest).into()
        } else {
            Match::failed()
        }
    }

    /// Analogue to the `and_then` method but adds a condition.
    /// When the condition is met, the first progression function is executed.
    /// Otherwise, the second progression function is executed.
    pub fn and_then_if_else<FC, FT, RT, FF, RF>(
        self,
        condition: FC,
        f_true: FT,
        f_false: FF,
    ) -> Match<T>
    where
        FC: FnOnce(usize, &T, &T) -> bool,
        FT: FnOnce(usize, T, T) -> RT,
        RT: Into<Match<T>>,
        FF: FnOnce(usize, T, T) -> RF,
        RF: Into<Match<T>>,
    {
        if condition(self.index, self.matched(), self.rest()) {
            f_true(self.index, self.matched, self.rest).into()
        } else {
            f_false(self.index, self.matched, self.rest).into()
        }
    }

    /// Analogue to the `and_then` method but adds a condition.
    /// When the condition is met, the first progression function is executed.
    /// Otherwise, the match on which the method was called is returned.
    pub fn and_then_if_else_self<FC, F, R>(self, condition: FC, f: F) -> Match<T>
    where
        FC: FnOnce(usize, &T, &T) -> bool,
        F: FnOnce(usize, T, T) -> R,
        R: Into<Match<T>>,
    {
        if condition(self.index, self.matched(), self.rest()) {
            f(self.index, self.matched, self.rest).into()
        } else {
            self.into()
        }
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

    /// Analogue to `Option` and `Result`'s method `and_then`.
    /// This method takes one function with two parameters.
    /// The first parameter of the function is the matched index.
    /// The second parameter of the function is the matched value.
    /// The third parameter of the function is the rest of the original value.
    /// The function has to return a match with the same argument type as the object it is
    /// called on and with the exception that the lifetime parameter can be smaller or equal.
    pub fn and_then<F, R>(self, f: F) -> Self
    where
        F: FnOnce(usize, T, T) -> R,
        R: Into<Self>,
    {
        if let Some(matched) = self.matched {
            let (index, matched, rest): (usize, T, T) = matched.take();

            f(index, matched, rest).into()
        } else {
            Self::failed()
        }
    }

    /// Analogue to the `and_then` method but adds a condition.
    /// When the condition is met, the progression function is executed.
    /// Otherwise, match fails.
    pub fn and_then_if<F1, F2, R>(self, condition: F1, f: F2) -> Self
    where
        F1: FnOnce(usize, &T, &T) -> bool,
        F2: FnOnce(usize, T, T) -> R,
        R: Into<Self>,
    {
        if let Ok(matched) = self.into_successful() {
            matched.and_then_if(condition, f)
        } else {
            Self::failed()
        }
    }

    /// Analogue to the `and_then` method but adds a condition.
    /// When the condition is met, the first progression function is executed.
    /// Otherwise, the second progression function is executed.
    pub fn and_then_if_else<FC, FT, RT, FF, RF>(
        self,
        condition: FC,
        f_true: FT,
        f_false: FF,
    ) -> Self
    where
        FC: FnOnce(usize, &T, &T) -> bool,
        FT: FnOnce(usize, T, T) -> RT,
        RT: Into<Self>,
        FF: FnOnce(usize, T, T) -> RF,
        RF: Into<Self>,
    {
        if let Ok(matched) = self.into_successful() {
            matched.and_then_if_else(condition, f_true, f_false)
        } else {
            Self::failed()
        }
    }

    /// Analogue to the `and_then` method but adds a condition.
    /// When the condition is met, the first progression function is executed.
    /// Otherwise, the match on which the method was called is returned.
    pub fn and_then_if_else_self<FC, F, R>(self, condition: FC, f: F) -> Self
    where
        FC: FnOnce(usize, &T, &T) -> bool,
        F: FnOnce(usize, T, T) -> R,
        R: Into<Self>,
    {
        if let Ok(matched) = self.into_successful() {
            matched.and_then_if_else_self(condition, f)
        } else {
            Self::failed()
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

    /// Analogue to `Option` and `Result`'s method `and_then`.
    /// This method takes one function with two parameters.
    /// The first parameter of the function is the matched index.
    /// The second parameter of the function is the matched value.
    /// The third parameter of the function is the rest of the original value.
    /// The function has to return a match with the same argument type as the object it is
    /// called on and with the exception that the lifetime parameter can be smaller or equal.
    pub fn and_then<F, R>(self, f: F) -> Self
    where
        F: FnOnce(usize, T, T) -> R,
        R: Into<Match<T>>,
    {
        if let Ok((index, matched, _)) = self.last_match.clone().take() {
            let mut matches: Vec<(usize, T)> = self.matches;

            matches.push((index, matched));

            Self {
                matches,
                last_match: self.last_match.and_then(f),
            }
        } else {
            Self {
                matches: Vec::new(),
                last_match: Match::failed(),
            }
        }
    }

    /// Analogue to `Option` and `Result`'s method `and_then`.
    /// This method takes one function with two parameters.
    /// The first parameter of the function is the matched index.
    /// The second parameter of the function is the matched value.
    /// The third parameter of the function is the rest of the original value.
    /// The function has to return a match with the same argument type as the object it is
    /// called on and with the exception that the lifetime parameter can be smaller or equal.
    pub fn and_then_repeat<N, F, R>(mut self, mut count: N, f: F) -> Self
    where
        N: PartialEq<usize> + core::ops::SubAssign<usize>,
        F: Fn(usize, T, T) -> R,
        R: Into<Match<T>>,
    {
        while !self.is_failed() && count != 0 {
            self = self.and_then(&f);

            count -= 1;
        }

        if count == 0 {
            self
        } else {
            Self::failed()
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
