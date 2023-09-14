use std::ops::Rem;

pub type MatchFn<T> = Box<dyn Fn(T) -> bool>;

pub struct Matcher<T> {
    matcher: MatchFn<T>,
    subs: String,
}

impl<T> Matcher<T> {
    pub fn new<F, S>(matcher: F, subs: S) -> Matcher<T>
    where
        F: Fn(T) -> bool + 'static,
        S: AsRef<str>,
    {
        Matcher {
            matcher: Box::new(matcher),
            subs: subs.as_ref().to_string(),
        }
    }
}

#[derive(Default)]
pub struct Fizzy<T>(Vec<Matcher<T>>);

impl<T> Fizzy<T>
where
    T: Copy + ToString,
{
    pub fn new() -> Self {
        Fizzy(Vec::new())
    }

    #[must_use]
    pub fn add_matcher(mut self, matcher: Matcher<T>) -> Self {
        let Fizzy(ref mut matchers) = self;
        matchers.push(matcher);
        self
    }

    pub fn apply_to(&self, item: T) -> String {
        let Fizzy(ref matchers) = self;
        let mut out = String::new();
        for matcher in matchers {
            if (matcher.matcher)(item) {
                out += &matcher.subs;
            }
        }
        if out.is_empty() {
            out = item.to_string()
        }
        out
    }

    /// convenience function: equivalent to `iter.map(move |item| self.apply_to(item))`.
    pub fn apply<I>(self, iter: I) -> impl Iterator<Item = String>
    where
        I: Iterator<Item = T>,
    {
        iter.map(move |item| self.apply_to(item))
    }
}

impl<T> From<Vec<Matcher<T>>> for Fizzy<T> {
    fn from(matchers: Vec<Matcher<T>>) -> Fizzy<T> {
        Fizzy(matchers)
    }
}

pub fn fizz_buzz<T>() -> Fizzy<T>
where
    T: Copy + Default + From<u8> + PartialEq + Rem<Output = T> + 'static,
{
    let three: T = 3.into();
    let five: T = 5.into();

    Fizzy(vec![
        Matcher::new(move |n| n % three == T::default(), "fizz"),
        Matcher::new(move |n| n % five == T::default(), "buzz"),
    ])
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn fizz_buzz_i32() {
        let expect = vec![
            "1", "2", "fizz", "4", "buzz", "fizz", "7", "8", "fizz", "buzz", "11", "fizz", "13",
            "14", "fizzbuzz", "16",
        ];
        let got = fizz_buzz().apply(1..=16).collect::<Vec<_>>();
        assert_eq!(expect, got);
    }

    #[test]
    fn fizz_buzz_u8() {
        let expect = vec![
            "1", "2", "fizz", "4", "buzz", "fizz", "7", "8", "fizz", "buzz", "11", "fizz", "13",
            "14", "fizzbuzz", "16",
        ];
        let got = fizz_buzz().apply(1_u8..=16).collect::<Vec<_>>();
        assert_eq!(expect, got);
    }

    #[test]
    fn fizz_buzz_u64() {
        let expect = vec![
            "1", "2", "fizz", "4", "buzz", "fizz", "7", "8", "fizz", "buzz", "11", "fizz", "13",
            "14", "fizzbuzz", "16",
        ];
        let got = fizz_buzz().apply(1_u64..=16).collect::<Vec<_>>();
        assert_eq!(expect, got);
    }

    #[test]
    fn fizz_buzz_nonsequential() {
        let collatz_12 = &[12, 6, 3, 10, 5, 16, 8, 4, 2, 1];
        let expect = vec![
            "fizz", "fizz", "fizz", "buzz", "buzz", "16", "8", "4", "2", "1",
        ];
        let got = fizz_buzz()
            .apply(collatz_12.iter().cloned())
            .collect::<Vec<_>>();
        assert_eq!(expect, got);
    }

    #[test]
    fn fizz_buzz_custom() {
        let expect = vec![
            "1", "2", "Fizz", "4", "Buzz", "Fizz", "Bam", "8", "Fizz", "Buzz", "11", "Fizz", "13",
            "Bam", "BuzzFizz", "16",
        ];
        let fizzer = Fizzy::new()
            .add_matcher(Matcher::new(|n| n % 5 == 0, "Buzz"))
            .add_matcher(Matcher::new(|n| n % 3 == 0, "Fizz"))
            .add_matcher(Matcher::new(|n| n % 7 == 0, "Bam"))
            .apply(1..=16);
        let got = fizzer.collect::<Vec<_>>();
        assert_eq!(expect, got);
    }

    #[test]
    fn map() {
        let expect = vec![
            "1", "2", "fizz", "4", "buzz", "fizz", "7", "8", "fizz", "buzz", "11", "fizz", "13",
            "14", "fizzbuzz", "16",
        ];
        let fb = fizz_buzz();
        let got = (1..=16)
            .map(move |item| fb.apply_to(item))
            .collect::<Vec<_>>();
        assert_eq!(expect, got);
    }

    #[test]
    fn fizz_buzz_f64() {
        let expect = vec![
            "1", "2", "fizz", "4", "buzz", "fizz", "7", "8", "fizz", "buzz", "11", "fizz", "13",
            "14", "fizzbuzz", "16",
        ];
        // a tiny bit more complicated because range isn't natively implemented on floats
        let got = fizz_buzz()
            .apply(std::iter::successors(Some(1.0), |prev| Some(prev + 1.0)))
            .take(16)
            .collect::<Vec<_>>();
        assert_eq!(expect, got);
    }
}
