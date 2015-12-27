//! Just Like Something Else
//!
//!   - **Date:** 2015-12-27
//!   - **Subject:** Generics, traits, and shared behavior in Rust.
//!   - **Audio:**
//!       + [M4A](http://www.podtrac.com/pts/redirect.m4a/cdn.newrustacean.com/e008.m4a)
//!       + [MP3](http://www.podtrac.com/pts/redirect.mp3/cdn.newrustacean.com/e008.mp3)
//!       + [Ogg](http://www.podtrac.com/pts/redirect.ogg/cdn.newrustacean.com/e008.ogg)
//!
//! <audio title="Just like something else" controls preload=metadata>
//!   <source src="http://www.podtrac.com/pts/redirect.m4a/cdn.newrustacean.com/e008.m4a">
//!   <source src="http://www.podtrac.com/pts/redirect.mp3/cdn.newrustacean.com/e008.mp3">
//!   <source src="http://www.podtrac.com/pts/redirect.ogg/cdn.newrustacean.com/e008.ogg">
//! </audio>
//!
//!
//! Notes
//! -----
//! In this episode we cover---at a *very* high level---two more fundamental
//! concepts in Rust programming: generics and traits.
//!
//! Generics gives us the abilitty to write types and functions which can be
//! used with more than one type. Traits give us the ability to specify behavior
//! which can be implemented for more than one type. The combination gives us
//! powerful tools for higher-level programming constructs in Rust.
//!
//! ### Comments on source code
//!
//! Now that we have a handle on [how tests work][e007], we'll use them to
//! validate the behavior of our code going forward. This is great: we can show
//! that the tests do what we think.
//!
//! To today's point, though: we actually know even apart from whether the tests
//! *run* successfully that these generic functions and the associated traits
//! are behaving as we want. Failure with generics is a *compile*-time error,
//! not a runtime error.
//!
//! [e007]: http://www.newrustacean.com/show_notes/e007/
//!
//!
//! Links
//! -----
//!
//!   - Rust Book
//!       + [Generics][l1]
//!       + [Traits][l2] -- includes a discussion of *trait bounds* and *generic
//!         *traits*
//!   - Rust by Example
//!       + [Generics][l3]
//!       + [Traits][l4]
//!       + [Generic *traits*][l5]
//!       + [Traits *bounds*][l6]
//!   - [Generics and traits in use in Diesel][l7]
//!
//! [l1]: https://doc.rust-lang.org/book/generics.html
//! [l2]: https://doc.rust-lang.org/book/traits.html
//! [l3]: http://rustbyexample.com/generics.html
//! [l4]: http://rustbyexample.com/trait.html
//! [l5]: http://rustbyexample.com/generics/gen_trait.html
//! [l6]: http://rustbyexample.com/generics/bounds.html
//! [l7]: https://github.com/sgrif/diesel/blob/master/diesel/src/types/mod.rs
//!
//!
//! Sponsors
//! --------
//!
//!   - Chris Palmer
//!   - [Derek Morr][sponsors-2]
//!   - Luca Schmid
//!   - Micael Bergeron
//!   - Ralph Giles ("rillian")
//!   - reddraggone9
//!   - [William Roe][sponsors-7]
//!
//! [sponsors-2]: https://twitter.com/derekmorr
//! [sponsors-7]: http://willroe.me
//!
//! ### Become a sponsor
//!
//!   - [Patreon](https://www.patreon.com/newrustacean)
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


/// Demonstrate a function *generic* over any type.
///
/// This uses a cross-language convention for generics, where `T` represents the
/// generic *type*. If we have more than one generic type, it's traditional to
/// represent it with following letters, after `T`: `U`, `V`, `W`, etc. (If you
/// have four *different* generic parameters, I'm probably going to look askance
/// at your API design, though, to be honest.)
pub fn a_generic<T>(_t: T) {
    println!("The function works, but I can't actually do anything with the `_t`.");
    println!("Why? Well, because it might not have the `Display` or `Debug` traits implemented.");
    println!("What's a trait? I'm so glad you asked.");
}


/// Demonstrate a function with a *trait bound on a generic*.
pub fn a_generic_printable<T: fmt::Debug> (t: T) {
    println!("This function can actually debug-print `t`, whatever it may be.");
    println!("So: {:?}", t);
}


/// This is `Option<T>`, but using Haskell's names instead.
pub enum Maybe<T> {
    Nothing,
    Just(T),
}


/// A simple type to illustrate trait implementation.
pub struct SimpleType {
    a: String,
    b: i32
}


/// Define the display format for the SimpleType.
///
/// Note that this works because String and i32 both already have display types
/// implemented themselves!
impl fmt::Display for SimpleType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "string: {} | integer: {}", self.a, self.b)
    }
}


/// This is just a container which can hold any two types.
///
/// These types can even be the same, but they don't have to be. Moreover, they
/// can be generic types themselves; as you can see in the tests below, you can
/// use built-in or user-created generic types within this generic type.
pub struct GenericContainer<T, U> {
    pub t: T,
    pub u: U,
}


/// Show that the generics work!
#[cfg(test)]
mod tests {
    use super::*;

    /// An example struct for the tests.
    #[derive(Debug)]
    struct TestPoint {
        x: f32,
        y: i32,
    }

    /// An example enum for the tests.
    #[derive(Debug)]
    enum TestEnum {
        Nope,
        SomePointTuple(f32, i32),  /// Any tuple type
        SomeAnonStruct { x: f32, y: i32 },  /// Functionally just like `TestPoint`
        SomePointStruct(TestPoint),   // Embed an actual `TestPoint`
    }

    /// All of these tests will pass.
    #[test]
    fn test_generic_fn() {
        a_generic(1);
        a_generic(());
        a_generic((1, 2));
        a_generic(TestPoint { x: 14.0, y: 12 });
        a_generic(TestEnum::Nope);
        a_generic(TestEnum::SomePointTuple(13.0, 10));
        a_generic(TestEnum::SomeAnonStruct { x: 24.3, y: 10 });
        a_generic(TestEnum::SomePointStruct(TestPoint { x: 1.2, y: 3 }));
    }

    /// So will all of these.
    #[test]
    fn test_generic_fn_with_debug_print() {
        a_generic_printable(1);
        a_generic_printable(());
        a_generic_printable((1, 2));
        a_generic_printable(TestPoint { x: 14.0, y: 12 });
        a_generic_printable(TestEnum::Nope);
        a_generic_printable(TestEnum::SomePointTuple(13.0, 10));
        a_generic_printable(TestEnum::SomeAnonStruct { x: 24.3, y: 10 });
        a_generic_printable(TestEnum::SomePointStruct(TestPoint { x: 1.2, y: 3 }));
    }

    #[test]
    fn test_generic_enum() {
        // `_nothing` must have its type specified because it can't be inferred
        // from the context. Generics can be type-inferred (see below), but as
        // with all types in Rust, when they can't be, you specify them.
        let _nothing: Maybe<i32> = Maybe::Nothing;
        let _just_25 = Maybe::Just(25);
        let _just_str = Maybe::Just("whoa");
        let _just_slice = Maybe::Just([1, 2, 3]);
        // Here we have a generic enum, wrapping a generic container, `Vec<T>`!
        let _just_vec = Maybe::Just(vec![1, 2, 3]);
        // Things could get complicated if we need to define a `Maybe::Nothing`
        // for a `Vec`: we need to specify the `Vec` type as well:
        let _no_vec: Maybe<Vec<i32>> = Maybe::Nothing;
        // Normally that woudn't be a problem, because it would be inferred by
        // the context, with a pattern match:
        let pattern = false;
        let _maybe_vec = match pattern {
            false => Maybe::Nothing,
            true => Maybe::Just(vec![1, 2, 3])
        };
    }

    #[test]
    fn test_generic_struct() {
        // The generic container can contain "normal" (non-generic) types.
        let _container = GenericContainer { t: 14.0, u: "alpha" };
        // But it can also contain generic types, like `Vec<T>` or the `Maybe`
        // type we defined above.
        let _another = GenericContainer { t: vec![1, 2, 3], u: Maybe::Just("a string") };
    }

    #[test]
    fn test_impl_display() {
        let simple = SimpleType { a: "some string".to_string(), b: 4096 };
        println!("simple is {}", simple);
    }
}
