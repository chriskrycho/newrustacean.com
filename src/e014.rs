//! Stringing things along
//!
//!   - **Date:** April 24, 2016
//!   - **Subject:** `Strings` `&str`s and `Vec`s and slices (and Unicode) --
//!     oh, my!
//!   - [**Audio**][mp3]
//!
//! [mp3]: http://www.podtrac.com/pts/redirect.mp3/f001.backblazeb2.com/file/newrustacean/e014.mp3
//!
//! <audio style="width: 100%" title="e014: Stringing things along" controls preload=metadata>
//!   <source src="http://www.podtrac.com/pts/redirect.mp3/f001.backblazeb2.com/file/newrustacean/e014.mp3">
//! </audio>
//!
//!
//! Notes
//! -----
//!
//! This episode, I take a deep dive on strings in Rust, looking at the
//! differences between `String` and `&str`, discussing Unicode a bit, and then
//! expanding the discussion to think about how these types relate to the types
//! they're built on (like `Vec`).
//!
//! ### Corrigenda
//!
//! Listener Nev pointed out to me that I got it wrong when describing how
//! `&str` data is stored. It is *not* stack-allocated, but rather goes in the
//! [data segment]. I should have said *statically*-allocated, not
//! *stack*-allocated. Thanks to Nev for the correction!
//!
//! [data segment]: https://en.wikipedia.org/wiki/Data_segment
//!
//!
//! Links
//! -----
//!
//!   - Strings:
//!       + [The Rust Book]
//!       + [Rust by Example]
//!       + `str` docs:
//!           * [module][strmod]
//!           * [primitive type]
//!       + `String`
//!           * [module][stringmod]
//!           * [type definition]
//!   - Dereferencing
//!       + [coercions]
//!       + [`std::ops::Deref`]
//!
//! [The Rust Book]: https://doc.rust-lang.org/book/strings.html
//! [Rust by Example]: http://rustbyexample.com/std/str.html
//! [strmod]: http://doc.rust-lang.org/std/str/
//! [primitive type]: http://doc.rust-lang.org/std/primitive.str.html
//! [stringmod]: http://doc.rust-lang.org/std/string/index.html
//! [type definition]: http://doc.rust-lang.org/std/string/struct.String.html
//! [coercions]: http://doc.rust-lang.org/book/deref-coercions.html
//! [`std::ops::Deref`]: http://doc.rust-lang.org/std/ops/trait.Deref.html
//!
//! Sponsors
//! --------
//!
//!   - Aleksey Pirogov
//!   - [Chris Palmer]
//!   - [Derek Morr]
//!   - Hamza Sheikh
//!   - Lachlan Collins
//!   - Leif Arne Storset
//!   - Luca Schmid
//!   - Micael Bergeron
//!   - [Pascal Hertleif]
//!   - Ralph Giles ("rillian")
//!   - Ralph "FriarTech" Loizzo
//!   - reddraggone9
//!   - Ryan Ollos
//!   - Vesa Kaihlavirta
//!   - [William Roe]
//!
//! [Chris Palmer]: http://home.red-oxide.org/
//! [Derek Morr]: https://twitter.com/derekmorr
//! [Pascal Hertleif]: https://pascalhertleif.de/
//! [William Roe]: http://willroe.me
//!
//! (Thanks to the couple people donating who opted out of the reward tier, as
//! well. You know who you are!)
//!
//! ### Become a sponsor
//!
//!   - [Patreon](https://www.patreon.com/newrustacean)
//!   - [Venmo](https://venmo.com/chriskrycho)
//!   - [Dwolla](https://www.dwolla.com/hub/chriskrycho)
//!   - [Cash.me](https://cash.me/$chriskrycho)
//!   - [Flattr](https://flattr.com/profile/chriskrycho)
//!
//!
//! Contact
//! -------
//!
//!   - New Rustacean:
//!       + Twitter: [@newrustacean](https://www.twitter.com/newrustacean)
//!       + Email: [hello@newrustacean.com](mailto:hello@newrustacean.com)
//!   - Chris Krycho
//!       + GitHub: [chriskrycho](https://github.com/chriskrycho)
//!       + Twitter: [@chriskrycho](https://www.twitter.com/chriskrycho)

/// Get a string *slice*. Note the required lifetime specifier on the type!
///
/// String slices are pointers to a given chunk of data.
pub fn get_a_slice() -> &'static str {
    "this is a statically allocated slice"
}

/// Get a `String` instance. Note there's no lifetime.
pub fn get_a_string() -> String {
    let mut a_string = String::new();
    a_string = a_string + "this is a heap-allocated String";
    a_string
}

/// It's easy enough to get a `String` from a `str`.
pub fn show_from_behavior() -> String {
    String::from("any old slice will do")
}

/// Print a 🚀, just because we can.
pub fn demonstrate_unicode() {
    println!("{}", "🚀");
}

pub fn get_back_some_unicode(desc: &str) -> String {
    match desc {
        "rocket" => "🚀".to_string(),
        "hearts" => "💕".to_string(),
        _ => " ".to_string(),
    }
}

/// Get a `String` with a specified capacity.
///
/// Strings are heap-allocated, so we can simply build them to hold a certain
/// number of characters by default if we know how big they are, allowing them
/// to expand later *if necessary*.
pub fn get_a_string_with_capacity(capacity: usize) -> String {
    let mut string = String::with_capacity(capacity);
    string = string + "few";
    string
}

/// Demonstrate dereferencing. (You'll want to read this example carefully.)
///
/// Note that here we have two types which are empty of values, which makes the
/// dereferencing operation quite straightforward. If the types had contents,
/// this would be a bit more involved!
///
/// Note as well that the dereference in this case recurses. This is a
/// consequence of having the empty types---so it's not exactly recommended to
/// do this in that case! (In fact, it's *usually*, though not always, pointless
/// to do that.)
pub mod demo_deref {
    use std::ops::Deref;

    /// A no-content struct to serve as the type to dereference from.
    pub struct Origin;

    /// A no-content struct to serve as the target to dereference to.
    pub struct DerefTarget;

    impl Deref for Origin {
        type Target = DerefTarget;

        #[cfg_attr(feature = "clippy", allow(unconditional_recursion))]
        fn deref(&self) -> &DerefTarget {
            self
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn show_string_capacity() {
        let capacity: usize = 4;
        let mut the_str = get_a_string_with_capacity(capacity);
        assert_eq!(the_str.capacity(), capacity);
        the_str = the_str + "this is more than 4";
        assert!(the_str.capacity() > capacity);
    }
}
