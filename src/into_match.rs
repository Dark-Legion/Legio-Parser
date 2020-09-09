use crate::Match;

pub trait IntoMatch: Sized {
    fn into_match(self) -> Match<Self> {
        Match::new(None, self)
    }
}

impl<T> IntoMatch for T {}
