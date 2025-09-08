// the PhantomData instances in this file are just to stop compiler complaints
// about missing generics; feel free to remove them

use std::fmt::Display;

/// A Matcher is a single rule of fizzbuzz: given a function on T, should
/// a word be substituted in? If yes, which word?
pub struct Matcher<T>(fn(T) -> bool, String);

impl<T> Matcher<T>
{
    pub fn new(_matcher: fn(T) -> bool, _subs: &str) -> Matcher<T> {
        Matcher(_matcher, _subs.to_string())
    }
}

/// A Fizzy is a set of matchers, which may be applied to an iterator.
///
/// Strictly speaking, it's usually more idiomatic to use `iter.map()` than to
/// consume an iterator with an `apply` method. Given a Fizzy instance, it's
/// pretty straightforward to construct a closure which applies it to all
/// elements of the iterator. However, we're using the `apply` pattern
/// here because it's a simpler interface for students to implement.
///
/// Also, it's a good excuse to try out using impl trait.
pub struct Fizzy<T: Display + Copy>(Vec<Matcher<T>>);

impl<T: Display + Copy> Fizzy<T> {
    pub fn new() -> Self {
        Self(vec![])
    }

    // feel free to change the signature to `mut self` if you like
    #[must_use]
    pub fn add_matcher(self, _matcher: Matcher<T>) -> Self {
        let mut fizzy = Self(self.0);
        fizzy.0.push(_matcher);
        fizzy
    }

    /// map this fizzy onto every element of an iterator, returning a new iterator
    pub fn apply<I: Iterator<Item=T>>(self, _iter: I) -> impl Iterator<Item=String> {

        _iter.map(move |i| {

            let mut str = String::new();

            self.0.iter().for_each(|s| {
                if s.0(i) {
                    str.push_str(s.1.as_str())
                }
            });

            if str.is_empty() {
                str.push_str(&(i.to_string()));
            }

            str

        }).into_iter()
    }
}

/// convenience function: return a Fizzy which applies the standard fizz-buzz rules
pub fn fizz_buzz<T: Copy + Display>() -> Fizzy<T> {
    let fizz = Fizzy::new()
        .add_matcher(Matcher::new(|n:T| n.to_string().parse::<f64>().unwrap() % 3.0f64 == 0f64  , "fizz"))
        .add_matcher(Matcher::new(|n:T| n.to_string().parse::<f64>().unwrap() % 5.0f64 == 0f64, "buzz"));
    fizz
}
