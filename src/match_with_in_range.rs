use crate::{Match, MatchWith};

mod private {
    pub trait SafeAsUsize {
        fn as_usize(self) -> usize;
    }

    macro_rules! impl_safe_as_usize {
        ($($t: ty),+ $(,)?) => {
            $(
                impl SafeAsUsize for $t {
                    fn as_usize(self) -> usize {
                        self as usize
                    }
                }
            )+
        };
    }

    impl_safe_as_usize! {
        isize,
        usize,

        i8,
        u8,
        i16,
        u16,
        i32,
        u32,
    }

    #[cfg(target_pointer_width = "64")]
    impl_safe_as_usize! {
        i64,
        u64,
    }
}

use private::SafeAsUsize;

/// Provides interface for matching single "dynamic" pattern.
/// This is a counter part of [`MatchStatic`].
///
/// [`MatchStatic`]: trait.MatchStatic.html
pub trait MatchWithInRange<E, N, F, R>: Sized {
    /// Matches a "dynamic" pattern by taking a function instead with taking into account a minimum amount.
    fn match_min_with(self, minimum: N, pattern: F) -> Match<R>;

    /// Matches a "dynamic" pattern by taking a function instead with taking into account a maximum amount.
    fn match_max_with(self, maximum: N, pattern: F) -> Match<R>;

    /// Matches a "dynamic" pattern by taking a function instead with taking into account a minimum and maximum amount.
    fn match_min_max_with(self, minimum: N, maximum: N, pattern: F) -> Match<R>;

    /// Matches a "dynamic" pattern by taking a function instead with taking into account a exact amount.
    fn match_exact_with(self, count: N, pattern: F) -> Match<R>;
}

impl<E, N, F> MatchWithInRange<E, N, F, Self> for &[E]
where
    for<'r> Self: MatchWith<E, &'r mut dyn FnMut(E) -> bool, Self>,
    N: SafeAsUsize,
    F: FnMut(E) -> bool,
{
    fn match_min_with(self, minimum: N, mut pattern: F) -> Match<Self> {
        let minimum: usize = minimum.as_usize();

        if self.len() < minimum {
            return Match::failed();
        }

        if let Ok((Some(matched), rest)) = self.match_with(&mut pattern).take() {
            if minimum <= matched.len() {
                Match::new(Some(matched), rest)
            } else {
                Match::failed()
            }
        } else {
            Match::failed()
        }
    }

    fn match_max_with(self, maximum: N, mut pattern: F) -> Match<Self> {
        let mut maximum: usize = maximum.as_usize();

        if maximum <= self.len() {
            self.match_with(&mut move |element: E| {
                if maximum == 0 {
                    false
                } else {
                    maximum -= 1;

                    pattern(element)
                }
            })
        } else {
            self.match_with(&mut pattern)
        }
    }

    fn match_min_max_with(self, minimum: N, maximum: N, pattern: F) -> Match<Self> {
        let (minimum, maximum): (usize, usize) = (minimum.as_usize(), maximum.as_usize());

        if maximum < minimum {
            return Match::failed();
        }

        if self.len() < minimum {
            return Match::failed();
        }

        if self.len() <= maximum {
            return self.match_min_with(minimum, pattern);
        }

        if let Ok((Some(matched), rest)) = self.match_max_with(minimum, pattern).take() {
            if minimum <= matched.len() {
                Match::new(Some(matched), rest)
            } else {
                Match::failed()
            }
        } else {
            Match::failed()
        }
    }

    fn match_exact_with(self, count: N, pattern: F) -> Match<Self> {
        let count: usize = count.as_usize();

        if self.len() < count {
            return Match::failed();
        }

        self.match_min_max_with(count, count, pattern)
    }
}

impl<E, N, F> MatchWithInRange<E, N, F, Self> for &str
where
    for<'r> Self: MatchWith<E, &'r mut dyn FnMut(E) -> bool, Self>,
    N: SafeAsUsize,
    F: FnMut(E) -> bool,
{
    fn match_min_with(self, minimum: N, mut pattern: F) -> Match<Self> {
        let minimum: usize = minimum.as_usize();

        if self.len() < minimum {
            return Match::failed();
        }

        if let Ok((Some(matched), rest)) = self.match_with(&mut pattern).take() {
            if minimum <= matched.len() {
                Match::new(Some(matched), rest)
            } else {
                Match::failed()
            }
        } else {
            Match::failed()
        }
    }

    fn match_max_with(self, maximum: N, mut pattern: F) -> Match<Self> {
        let mut maximum: usize = maximum.as_usize();

        if maximum <= self.len() {
            self.match_with(&mut move |element: E| {
                if maximum == 0 {
                    false
                } else {
                    maximum -= 1;

                    pattern(element)
                }
            })
        } else {
            self.match_with(&mut pattern)
        }
    }

    fn match_min_max_with(self, minimum: N, maximum: N, pattern: F) -> Match<Self> {
        let (minimum, maximum): (usize, usize) = (minimum.as_usize(), maximum.as_usize());

        if maximum < minimum {
            return Match::failed();
        }

        if self.len() < minimum {
            return Match::failed();
        }

        if self.len() <= maximum {
            return self.match_min_with(minimum, pattern);
        }

        if let Ok((Some(matched), rest)) = self.match_max_with(minimum, pattern).take() {
            if minimum <= matched.len() {
                Match::new(Some(matched), rest)
            } else {
                Match::failed()
            }
        } else {
            Match::failed()
        }
    }

    fn match_exact_with(self, count: N, pattern: F) -> Match<Self> {
        let count: usize = count.as_usize();

        if self.len() < count {
            return Match::failed();
        }

        self.match_min_max_with(count, count, pattern)
    }
}

impl<E, N, F, R, I> MatchWithInRange<E, N, F, R> for &I
where
    I: MatchWithInRange<E, N, F, R> + Clone,
{
    fn match_min_with(self, minimum: N, pattern: F) -> Match<R> {
        self.clone().match_min_with(minimum, pattern)
    }

    fn match_max_with(self, maximum: N, pattern: F) -> Match<R> {
        self.clone().match_max_with(maximum, pattern)
    }

    fn match_min_max_with(self, minimum: N, maximum: N, pattern: F) -> Match<R> {
        self.clone().match_min_max_with(minimum, maximum, pattern)
    }

    fn match_exact_with(self, count: N, pattern: F) -> Match<R> {
        self.clone().match_exact_with(count, pattern)
    }
}

impl<E, N, F, R, I> MatchWithInRange<E, N, F, R> for &mut I
where
    I: MatchWithInRange<E, N, F, R> + Clone,
{
    fn match_min_with(self, minimum: N, pattern: F) -> Match<R> {
        (&*self).match_min_with(minimum, pattern)
    }

    fn match_max_with(self, maximum: N, pattern: F) -> Match<R> {
        (&*self).match_max_with(maximum, pattern)
    }

    fn match_min_max_with(self, minimum: N, maximum: N, pattern: F) -> Match<R> {
        (&*self).match_min_max_with(minimum, maximum, pattern)
    }

    fn match_exact_with(self, count: N, pattern: F) -> Match<R> {
        (&*self).match_exact_with(count, pattern)
    }
}
