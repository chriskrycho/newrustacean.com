//! Composing a Rustic tune
//!
//!   - **Date:** 2016-01-09
//!   - **Subject:** Getting into the nitty-gritty with Rust's traits.
//!   - [**Audio**][mp3]
//!
//! [mp3]: https://www.podtrac.com/pts/redirect.mp3/f001.backblazeb2.com/file/newrustacean/e009.mp3
//!
//! <audio style="width: 100%" title="Composing a Rustic tune" controls preload=metadata>
//!   <source src="https://www.podtrac.com/pts/redirect.mp3/f001.backblazeb2.com/file/newrustacean/e009.mp3">
//! </audio>
//!
//!
//! Notes
//! -----
//! Last time, we looked at generics and traits at a high level. This time, we
//! dig deeper on traits, looking specifically at `std::iter::Iterator` as an
//! example of a powerful trait that can be composed across types, and then at
//! how we might compose multiple traits on a single type.
//!
//! We also talk about the syntax for traits, the use of marker traits, some of
//! the things you *can't* presently do with traits, and even just a smidge
//! about the *future* of traits in Rust. All that in less than 20 minutes!
//!
//! You'll find today's [source example][src] fairly interesting, I think: it's
//! just one type, but it uses almost every concept discussed on the show today!
//!
//! [src]: /src/show_notes/e009.rs.html
//!
//!
//! Links
//! -----
//!
//!   - Nick Cameron: ["Thoughts on Rust in 2016"][l1]
//!   - ["Upcoming breakage starting in Rust 1.7, from RFCs 1214 and 136"][l2]
//!       + [RFC 1214: Clarify (and improve) rules for projections and well-formedness][l3]
//!       + [RFC 136: Ban private items in public APIs][l4]
//!   - The Rust Book:
//!       + [Traits][l5]
//!       + [Trait objects][l6] (dynamic dispatch)
//!   - The Rust reference:
//!       + [`std::iter`][l7] and [`std::iter::Iterator`][l8]
//!       + [`Add`][l9]
//!       + [`Drop`][l10]
//!       + [`PartialEq`][l11] and [`Eq`][l12]
//!       + [`PartialOrd`][l13] and [`Ord`][l14]
//!       + [Special traits][l5]
//!       + [Trait objects][l16]
//!   - [RFC: impl specialization][l17]
//!       + Aaron Turon: ["Specialize to reuse"][l18]
//!
//! [l1]: http://www.ncameron.org/blog/my-thoughts-on-rust-in-2016/
//! [l2]: https://users.rust-lang.org/t/upcoming-breakage-starting-in-rust-1-7-from-rfcs-1214-and-136/4207
//! [l3]: https://github.com/rust-lang/rfcs/blob/master/text/1214-projections-lifetimes-and-wf.md
//! [l4]: https://github.com/rust-lang/rfcs/blob/master/text/0136-no-privates-in-public.md
//! [l5]: https://doc.rust-lang.org/book/traits.html
//! [l6]: https://doc.rust-lang.org/book/trait-objects.html
//! [l7]: https://doc.rust-lang.org/std/iter/index.html
//! [l8]: https://doc.rust-lang.org/std/iter/trait.Iterator.html
//! [l9]: https://doc.rust-lang.org/std/ops/trait.Add.html
//! [l10]: https://doc.rust-lang.org/std/ops/trait.Drop.html
//! [l11]: https://doc.rust-lang.org/std/cmp/trait.PartialEq.html
//! [l12]: https://doc.rust-lang.org/std/cmp/trait.Eq.html
//! [l13]: https://doc.rust-lang.org/std/cmp/trait.PartialOrd.html
//! [l14]: https://doc.rust-lang.org/std/cmp/trait.Ord.html
//! [l15]: https://doc.rust-lang.org/reference.html#special-traits
//! [l16]: https://doc.rust-lang.org/reference.html#trait-objects
//! [l17]: https://github.com/rust-lang/rfcs/pull/1210
//! [l18]: https://aturon.github.io/blog/2015/09/18/reuse/
//!
//!
//! Sponsors
//! --------
//!
//!   - Aleksey Pirogov
//!   - Chris Palmer
//!   - [Derek Morr][s3]
//!   - Hamza Sheikh
//!   - Luca Schmid
//!   - Micael Bergeron
//!   - Ralph Giles ("rillian")
//!   - reddraggone9
//!   - [William Roe][s9]
//!
//! [s3]: https://twitter.com/derekmorr
//! [s9]: http://willroe.me
//!
//! ### Become a sponsor
//!
//!   - <a href="https://www.patreon.com/newrustacean" rel="payment">Patreon</a>
//!   - [Venmo](https://venmo.com/chriskrycho)
//!   - [Dwolla](https://www.dwolla.com/hub/chriskrycho)
//!   - [Cash.me](https://cash.me/$chriskrycho)
//!
//!
//! Follow
//! ------
//!
//!   - New Rustacean:
//!       + Twitter: [@newrustacean](https://www.twitter.com/newrustacean)
//!       + App.net: [@newrustacean](https://alpha.app.net/newrustacean)
//!       + Email: [hello@newrustacean.com](mailto:hello@newrustacean.com)
//!   - Chris Krycho
//!       + Twitter: [@chriskrycho](https://www.twitter.com/chriskrycho)
//!       + App.net: [@chriskrycho](https://alpha.app.net/chriskrycho)

use std::fmt;
use std::ops;

/// Define a simple struct on which to implement `Iterator`... and more!
///
/// Pull in the `Clone` and `Copy` traits so that we can create copies cheaply
/// with `into_iter`. Derive `Debug` so error printing works. Derive `PartialEq`
/// so items can be compared!
///
/// Look at all those default implementations we get for free!
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DoubleAndOne {
    /// A value to double and add one to on every `next()` call.
    pub value: i64,
}

impl DoubleAndOne {
    /// Start with 0.
    pub fn default() -> DoubleAndOne {
        DoubleAndOne { value: 0 }
    }
}

/// Define a simple trait so we can see how it works.
///
/// Note that its `a_default_print()` method becomes available automatically for
/// `DoubleAndOne` when we `impl ASimpleTrait for DoubleAndOne` below.
pub trait ASimpleTrait {
    /// Have the item return an integer.
    fn get_some_integer(&self) -> i64;

    /// Have the item print and then return some string.
    fn a_default_print(&self) -> &str {
        let msg = "This is implemented already!";
        println!("{}", msg);
        msg
    }
}

impl ASimpleTrait for DoubleAndOne {
    fn get_some_integer(&self) -> i64 {
        self.value
    }
}

impl fmt::Display for DoubleAndOne {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "DoubleAndOne.value = {}", self.value)
    }
}

impl Iterator for DoubleAndOne {
    /// Define the required `Item` as being the the struct itself.
    type Item = DoubleAndOne;

    fn next(&mut self) -> Option<DoubleAndOne> {
        self.value = self.value * 2 + 1;
        Some(DoubleAndOne { value: self.value })
    }
}

impl ops::Add for DoubleAndOne {
    type Output = DoubleAndOne;

    fn add(self, rhs: DoubleAndOne) -> DoubleAndOne {
        DoubleAndOne {
            value: self.value + rhs.value,
        }
    }
}

/// Demonstrate using a for loop over an (infinite!) iterator.
#[cfg_attr(feature = "clippy", allow(explicit_counter_loop))]
pub fn demonstrate_for() {
    let mut printed = 0;
    let max_to_print = 10;
    let d = DoubleAndOne::default();
    for item in d {
        println!("{}", item);
        printed += 1;
        if printed > max_to_print {
            break;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_a_simple_trait() {
        let d = DoubleAndOne { value: 24 };
        // We have the trait function we implemented.
        assert_eq!(d.get_some_integer(), 24);
        // We also have the trait function we *didn't* implement.
        assert_eq!(d.a_default_print(), "This is implemented already!");
    }

    #[test]
    fn test_iterator() {
        let mut d = DoubleAndOne::default();
        assert_eq!(d.next(), Some(DoubleAndOne { value: 1 }));
        assert_eq!(d.next(), Some(DoubleAndOne { value: 3 }));
        assert_eq!(d.next(), Some(DoubleAndOne { value: 7 }));
        assert_eq!(d.next(), Some(DoubleAndOne { value: 15 }));
        assert_eq!(d.next(), Some(DoubleAndOne { value: 31 }));
        assert_eq!(d.next(), Some(DoubleAndOne { value: 63 }));
    }

    #[test]
    fn test_add() {
        let d = DoubleAndOne { value: 10 };
        let e = DoubleAndOne { value: 12 };
        assert_eq!(d + e, DoubleAndOne { value: 22 });
    }

    #[test]
    fn test_together() {
        let d = DoubleAndOne { value: 42 };
        let e = d.into_iter().next().unwrap();
        assert_eq!(d + e, DoubleAndOne { value: 127 });
    }
}
