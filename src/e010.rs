//! Macros rule!
//!
//!   - **Date:** 2016-01-18
//!   - **Subject:** Using Rust's macro system, its limitations, and its future.
//!   - **Audio:**
//!       + [M4A](http://www.podtrac.com/pts/redirect.m4a/cdn.newrustacean.com/e010.m4a)
//!       + [MP3](http://www.podtrac.com/pts/redirect.mp3/cdn.newrustacean.com/e010.mp3)
//!       + [Ogg](http://www.podtrac.com/pts/redirect.ogg/cdn.newrustacean.com/e010.ogg)
//!
//! <audio style="width: 100%" title="Macros rule!" controls preload=metadata>
//!   <source src="http://www.podtrac.com/pts/redirect.m4a/cdn.newrustacean.com/e010.m4a">
//!   <source src="http://www.podtrac.com/pts/redirect.mp3/cdn.newrustacean.com/e010.mp3">
//!   <source src="http://www.podtrac.com/pts/redirect.ogg/cdn.newrustacean.com/e010.ogg">
//! </audio>
//!
//!
//! Notes
//! -----
//!
//! Because of the way macros are exported---before name resolution on crates
//! occurs---the documentation for the macros defined in the source for this
//! episode occurs in the [**Macros** section of the `show_notes` crate][n1]
//! documentation, rather than within the documentation for this module. (See
//! the Rust Book discussion of [documenting macros][n2] for details.) Even so,
//! the [source][n3] is still in this module; see the [implementations][n4] for
//! details.
//!
//! [n1]: https://www.newrustacean.com/show_notes/#macros
//! [n2]: http://doc.rust-lang.org/stable/book/documentation.html#documenting-macros
//! [n3]: https://www.newrustacean.com/src/show_notes/e009.rs.html
//! [n4]: https://www.newrustacean.com/src/show_notes/e010.rs.html#101
//!
//! Links
//! -----
//!   - Julia Evans: ["Why I (Heart) Rust"][l1]
//!   - Steve Klabnik: ["IntermezzOS"][l2] (a small teaching OS built in Rust)
//!   - [Rust book: Macros][l3]
//!   - [Rust by Example: Macros][l4]
//!   - [Rust reference: Macros][l5]
//!   - ["Macro by Example"][l6] (original paper)
//!   - Nick Cameron:
//!       + [Macros][l7]
//!       + Macros in Rust
//!           * [Part 1: `macro_rules!`][l8]
//!           * [Part 2: procedural macros][l9]
//!           * [Part 3: hygiene][l10]
//!           * [Part 4: scoping and import/export][l11]
//!           * [Part 5: current problems and possible solutions][l12]
//!           * [Part 6: more issues with `macro_rules!`][l13]
//!       + [`concat_idents` and macros in ident positions][l14]
//!       + [Macro plans, overview][l15]
//!       + [Macro hygiene in all its guises and variations][l16]
//!       + [Sets of scopes macro hygiene][l17]
//!       + [Macro plans: syntax][l18]
//!       + [Procedural macros, framework][l19]
//!
//! [l1]: https://speakerdeck.com/jvns/why-i-rust
//! [l2]: https://intermezzos.github.io/
//! [l3]: https://doc.rust-lang.org/book/macros.html
//! [l4]: http://rustbyexample.com/macros.html
//! [l5]: https://doc.rust-lang.org/reference.html#macros
//! [l6]: https://www.cs.indiana.edu/ftp/techreports/TR206.pdf
//! [l7]: http://www.ncameron.org/blog/macros/
//! [l8]: http://ncameron.org/blog/macros-in-rust-pt1/
//! [l9]: http://www.ncameron.org/blog/macros-in-rust-pt2/
//! [l10]: http://www.ncameron.org/blog/macros-in-rust-pt3/
//! [l11]: http://www.ncameron.org/blog/macros-in-rust-pt4/
//! [l12]: http://www.ncameron.org/blog/macros-in-rust-pt5/
//! [l13]: http://www.ncameron.org/blog/macros-pt6-more-issues/
//! [l14]: http://www.ncameron.org/blog/untitledconcat_idents-and-macros-in-ident-position/
//! [l15]: http://www.ncameron.org/blog/macro-plans-overview/
//! [l16]: http://www.ncameron.org/blog/macro-hygiene-in-all-its-guises-and-variations/
//! [l17]: http://www.ncameron.org/blog/sets-of-scopes-macro-hygiene/
//! [l18]: http://www.ncameron.org/blog/macro-plans-syntax/
//! [l19]: http://www.ncameron.org/blog/procedural-macros-framework/
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


/// Define a macro like `try!` but which works in the context of `main()`.
///
/// [`try!`][try] takes a `Result<T, E>` and, if it is `Ok<T>`, supplies the `T`
/// value, or if it is `Err<E>` returns the error from the function. However,
/// since `main` has a void tuple `()` return type, you cannot use `try!` in
/// `main()`. This `main_try!` macro instead debug-prints the error and returns.
///
/// [try]: https://doc.rust-lang.org/std/macro.try!.html
///
/// ```rust
/// # #[macro_use] extern crate show_notes;
/// # fn main() {
/// // Alias `Result` for brevity.
/// type LocalResult = Result<i32, &'static str>;
///
/// let an_ok: LocalResult = Ok(10);
/// let ok_val = main_try!(an_ok);
/// assert_eq!(ok_val, 10);
///
/// // We try to assign to another val, but it's an error, so we return.
/// let an_err: LocalResult = Err("Alas, this is a failure.");
/// let err_val = main_try!(an_err);  // Prints `Alas, this is a failure.`
/// // We'll never get here. If we did, the doctest would fail.
/// assert!(false);
/// # }
/// ```
#[macro_export]
macro_rules! main_try {
    ($e:expr) => (match $e {
        std::result::Result::Ok(val) => val,
        std::result::Result::Err(err) => {
            println!("{:?}", err);
            return;
        }
    })
}


/// Define an ident macro to show how they can capture different *syntax*.
///
/// Whereas `main_try!` captured *expressions*, `print_ident_name!` captures
/// *identifiers*. If you try to pass in an expression, it simply won't work.
/// So, if you tried to do any of these, it won't compile:
///
/// ```rust,ignore
/// # #[macro_use] extern crate show_notes;
/// # fn main() {
/// print_ident_name!(());
/// print_ident_name!(42);
/// print_ident_name!(if true { println!("Neato!"); } else { println!("Sads!"); });
/// # }
/// ```
///
/// Instead, it will complain that it expected an identifier, and you handed it
/// something else.
///
/// On the other hand, this works just fine:
///
/// ```rust
/// # #[macro_use] extern crate show_notes;
/// # fn main() {
/// let _x = 0;
/// print_ident_name!(_x);  // "The ident's name was: _x"
/// # }
/// ```
///
/// In this case, the implementation uses two other macros: `format!` and
/// `stringify!`, to accomplish its work. This highlights the reality that
/// macros can use any other language machinery, including other macros.
#[macro_export]
macro_rules! print_ident_name {
    ($id:ident) => ( format!("The ident's name was: {}", stringify!($id)); );
}


/// Trivial alias for Result for convenience.
pub type TryResult = Result<i32, &'static str>;

/// Demonstrate how `try!` works in practice.
pub fn demonstrate_try(tr: TryResult) -> TryResult {
    // If the caller passes in an error, this returns that error.
    let val = try!(tr);
    // Thus, if `tr` was `Err`, we'll never get here. If it's an `Ok`, `val` has
    // the value, so we can return it slightly modified to show that that's what
    // actually happened.
    Ok(val + 1)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_demonstrate_try() {
        assert_eq!(demonstrate_try(Ok(14)), Ok(15));
        assert_eq!(demonstrate_try(Err("WHAT")), Err("WHAT"));
    }

    #[test]
    fn test_print_ident_name() {
        // We can operate on any identifier. Variables:
        let _x = 42;
        assert_eq!(print_ident_name!(_x), format!("The ident's name was: _x"));
        // Functions:
        assert_eq!(print_ident_name!(demonstrate_try),
                   format!("The ident's name was: demonstrate_try"));
        // Types:
        assert_eq!(print_ident_name!(TryResult), format!("The ident's name was: TryResult"));
        // Even macros!
        assert_eq!(print_ident_name!(print_ident_name),
                   format!("The ident's name was: print_ident_name"));
    }
}
