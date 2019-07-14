//! Allocate it where?
//!
//!   - **Date:** November 8, 2015
//!   - **Subject:** Returning functions from other functions, and thinking
//!     about the stack, the heap, and reference types.
//!   - [**Audio**][mp3]
//!
//! [mp3]: https://www.podtrac.com/pts/redirect.mp3/cdn.newrustacean.com/file/newrustacean/e005.mp3
//!
//! <audio style="width: 100%" title="Allocate it where?" controls preload=metadata src="https://www.podtrac.com/pts/redirect.mp3/cdn.newrustacean.com/file/newrustacean/e005.mp3" />
//!
//! # Notes
//!
//! This episode, we look at returning functions from other functions, and as
//! part of that discuss some basics about the stack and the heap---and why we
//! need to care about them for returning functions.
//!
//! The functions themselves are not especially interesting; they just show you
//! the basic form you use to return functions from other functions, and how to
//! then use them in another function. You'll want to take a detailed look
//! instead at the documentation for each (or just read the [source][notes-1]!),
//! because that's where the meat of the discussion in this week's code is.
//!
//! ## Corrigenda
//!
//!   - Steve Klabnik [pointed out][notes-2] that my description of `Vector`
//!     types as totally heap-allocated was misleading. It's better to say that
//!     the *contents* of the `Vector` -- its data -- is heap-allocated, while
//!     the memory for the smart pointer and associated metadata are allocated
//!     on the stack. I had this in mind, and sort of alluded to it earlier in
//!     the discussion, but the way I actually said it was misleading at best.
//!
//!   - Chad Sharp (@crossroads1112 on GitHub) [clarified][notes-3] that in C99,
//!     C *does* support dynamic array allocation, though it became optional for
//!     compilers to implement it as of C11. I forgot about this because I spend
//!     so much of my time dealing with Visual C++, which does not support
//!     dynamic array allocation. (Notably, Visual C does; it is Visual C++
//!     which does not: remember, C and C++ are related but distinct languages.)
//!
//! Thanks to Steve and Chad for their helpful feedback!
//!
//! [notes-1]: /src/show_notes/e005.rs.html
//! [notes-2]: https://users.rust-lang.org/t/new-rustacean-podcast-e005-allocate-it-where-2015-11-08/3153/13?u=chriskrycho
//! [notes-3]: https://github.com/chriskrycho/newrustacean.com/issues/7
//!
//! # Links
//!
//!   - [Rust 1.4 release announcement][links-1]
//!   - ["Clarify (and improve) rules for projections and well-formedness"][links-2]
//!   - [MSVC support tracking issue][links-3]
//!   - [Rust 1.4 full release notes][links-4]
//!   - ["What and where are the stack and the heap?"][links-5]
//!
//! [links-1]: http://blog.rust-lang.org/2015/10/29/Rust-1.4.html
//! [links-2]: https://github.com/rust-lang/rfcs/blob/master/text/1214-projections-lifetimes-and-wf.md
//! [links-3]: https://github.com/rust-lang/rfcs/issues/1061
//! [links-4]: https://github.com/brson/rust/blob/relnotes/RELEASES.md
//! [links-5]: http://stackoverflow.com/questions/79923/what-and-where-are-the-stack-and-heap
//!
//! # Sponsors
//!
//!   - reddraggone9
//!   - [Chris Patti][sponsors-1]
//!
//! [sponsors-1]: http://podcastinit.com
//!
//! ## Become a sponsor
//!
//!   - <a href="https://www.patreon.com/newrustacean" rel="payment">Patreon</a>
//!   - [Venmo](https://venmo.com/chriskrycho)
//!   - [Dwolla](https://www.dwolla.com/hub/chriskrycho)
//!   - [Cash.me](https://cash.me/$chriskrycho)
//!
//! # Follow
//!
//!   - New Rustacean:
//!     + Twitter: [@newrustacean](https://www.twitter.com/newrustacean)
//!       + App.net: [@newrustacean](https://alpha.app.net/newrustacean)
//!     + Email: [hello@newrustacean.com](mailto:hello@newrustacean.com)
//!   - Chris Krycho
//!     + Twitter: [@chriskrycho](https://www.twitter.com/chriskrycho)
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
    fn double(n: i32) -> i32 {
        n * 2
    };
    Box::new(double)
}

/// Creates a closure which doubles an integer.
///
/// Compare with the function above: this is basically identical to
/// `doubler_factory`, the only difference being that in this case, we return a
/// closure rather than a standalone function. That has substantial
/// ramifications for using closures and named functions, of course; refer to
/// the discussion in [e004][e004] for details
///
/// [e004]: /show_notes/e004/
pub fn doubler_closure_factory() -> Box<Fn(i32) -> i32> {
    // We could also write this as `Box::new(|n| n * 2)`, of course.
    let doubler = |n| n * 2;
    Box::new(doubler)
}

/// Uses the `doubler_factory` to get a function which doubles a number.
///
/// By contrast, here's a function which simply won't compile:
///
/// ```rust,ignore
/// fn will_not_compile() -> Fn(i32) {
///     let a_closure = |n| println!("Seriously. This won't compile. {}", n);
///     a_closure
/// }
/// ```
///
/// And another. This gets us a bit closer, because you actually have both a
/// concrete reference type and a lifetime. However, it still won't compile,
/// because after `get_normal_function_with_lifetime` ends, the function goes
/// out of scope and the reference points to junk value.
///
/// ```rust,ignore
/// fn get_normal_function_with_lifetime() -> &'static (Fn(i32) -> i32) {
///     fn multiply_by_3(n: i32) -> i32 {
///         n * 3
///     }
///
///     return &multiply_by_3;
/// }
/// ```
///
/// Even this approach doesn't work, because although the *function* lives on
/// past the end of the `get_normal_external_fn_with_lifetime`, the reference
/// created during that function call *doesn't*.
///
/// ```rust,ignore
/// fn multiply_by_4(n: i32) -> i32 { n * 4 }
///
/// fn get_normal_external_fn_with_lifetime() -> &'static (Fn(i32) -> i32) {
///     &multiply_by_4
/// }
/// ```
///
/// The solution, as we saw at the beginning, is to use `Box::new` to
/// heap-allocate the function result instead.
pub fn demonstrate_function_returns() {
    let double = doubler_factory();
    println!("{:}", double(14));
}
