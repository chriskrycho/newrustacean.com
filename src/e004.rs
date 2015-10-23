//! TODO: title
//!
//!   - **Date:** October 12, 2015
//!   - **Subject:** The `struct` data type constructor, and the basics of
//!     Rust's "ownership" concept and "borrowing" and "moving".
//!   - **Audio:**
//!       + [M4A](http://www.podtrac.com/pts/redirect.m4a/cdn.newrustacean.com/e004.m4a)
//!       + [MP3](http://www.podtrac.com/pts/redirect.mp3/cdn.newrustacean.com/e004.mp3)
//!       + [Ogg](http://www.podtrac.com/pts/redirect.ogg/cdn.newrustacean.com/e004.ogg)
//!
//! # Notes
//!
//! # Links
//!
//!  - [Exercism][link-1] (hat tip: [Lechindanier on GitHub][link-2])
//!  - [Rust Learning][link-3]
//!
//! [link-1]: http://exercism.io/languages/rust
//! [link-2]: https://github.com/Lechindianer
//! [link-3]: https://github.com/ctjhoa/rust-learning
//!
//! # Follow/Support
//!
//!   - New Rustacean:
//!       + Twitter: [@newrustacean](https://www.twitter.com/newrustacean)
//!       + App.net: [@newrustacean](https://alpha.app.net/newrustacean)
//!       + [Patreon](https://www.patreon.com/newrustacean)
//!       + Email: [hello@newrustacean.com](mailto:hello@newrustacean.com)
//!   - Chris Krycho
//!       + Twitter: [@chriskrycho](https://www.twitter.com/chriskrycho)
//!       + App.net: [@chriskrycho](https://alpha.app.net/chriskrycho)


/// Creates a function which doubles an integer.
///
/// Note that it doesn't have `Fn(i32) -> i32` as its return type, but rather
/// `Box<Fn(i32) -> i32`. At present, functions have to be explicitly
/// heap-allocated. If we tried to return just a `Fn` type, we'd end up in a
/// tangled mess of lifetime and ownership (whether with a regular function, as
/// here, or a closure as in `doubler_closure_factory`).
pub fn doubler_factory() -> Box<Fn(i32) -> i32> {
    /// The doubler function we will return.
    pub fn double(n: i32) -> i32 { n * 2 };
    Box::new(double)
}



pub fn demonstrate_function_returns() {
    let double = doubler_factory();
    println!("{:}", double(14));
}


/// Shows how methods work.
pub struct MethodDemonstrator {
    // Public data.
    pub an_int: i64;
    pub a_string: String;

    // Private data.
    /// A tuple holding a floating point value and a string slice. (We'll
    /// discuss string slices in a future episode.)
    a_tuple: (f64, &str);
}


impl MethodDemonstrator {

    /// A standard constructor pattern.
    ///
    /// Note that Rust doesn't have constructors in the same sense as C++ or
    /// Java: you can construct a `MethodDemonstrator` just as this function
    /// does somewhere else. Using `new` is a convenient convention, so you can
    /// just call `MethodDemonstrator::new()` to get an instance, rather than
    /// needing to worry about the details of the struct.
    ///
    /// This is particularly important because not all types are necessarily
    /// public.
    pub fn new() -> MethodDemonstrator {
        { an_int: 0, a_string: "Nothin' into nothing, divide the nothin'...".to_string() }
    }
}
