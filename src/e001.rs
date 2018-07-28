//! Document all the things!
//!
//!   - **Date:** October 3, 2015
//!   - **Subject:** Documentation in general, and `rustdoc` and `cargo doc` in
//!     particular.
//!   - [**Audio**][mp3]
//!
//! [mp3]: http://www.podtrac.com/pts/redirect.mp3/f001.backblazeb2.com/file/newrustacean/e001.mp3
//!
//! <audio style="width: 100%" title="Document all the things!" controls preload=metadata src="http://www.podtrac.com/pts/redirect.mp3/f001.backblazeb2.com/file/newrustacean/e001.mp3" />
//!
//! # Notes
//! This is a mostly-empty module, and it is intended as such. Why? Well,
//! because *almost* all the sample code exists in these comments, which serve
//! as the show notes. If you listen to the episode or take a look at the
//! [source files][src], you'll see how it works!
//!
//! [src]: /src/show_notes/e001.rs.html
//!
//! The components below are included *solely* so you can see how the
//! docstrings work with each kind of thing. Make sure to click on the names of
//! the items: there is more documentation there. Again, take a look at the
//! [source][src] to see how it looks in the context of a file module.
//!
//! Note that this module-level docstring uses `//!` rather than `///`-style
//! comments. This is because this docstring is documenting the item which
//! contains it, rather than the following item. Per [Rust RFC 505][1], the
//! preferred approach is always to use the "following" form (`///`) rather than
//! the "containing" form (`//!`), except for module-level docs like these.
//! (I will be following RFC 505 throughout.)
//!
//! [1]: https://github.com/rust-lang/rfcs/blob/master/text/0505-api-comment-conventions.md
//!
//! # Links
//!   - [Rust and MSVC tracking issue][2]
//!   - Other documentation tools:
//!       + Predecessors:
//!           * [Python's Sphinx tool]
//!           * [Doxygen]
//!           * [`JSDoc`]
//!           * [`JavaDoc`]
//!       + Other new languages with Markdown tooling
//!           * [Julia][7] has a [built-in documentation system][8]
//!           * [Elixir][9] has [`ex_doc`][10]
//!   - [Rust 1.3 release announcement][11]
//!   - Rust's package hosting: [crates.io][12]
//!       + [Crater][13] for testing for backwards compatibility
//!   - [Semantic versioning][14]
//!       + ["Stability as a Deliverable"][15]: Rust official blog post on
//!         version stability, backwards compatibility, and release channels.
//!   - [The Rust book chapter on `rustdoc`][16]
//!
//! [2]: https://github.com/rust-lang/rfcs/issues/1061
//! [Python's Sphinx tool]: http://sphinx-doc.org
//! [Doxygen]: http://www.stack.nl/~dimitri/doxygen/
//! [`JSDoc`]: http://usejsdoc.org
//! [`JavaDoc`]: http://www.oracle.com/technetwork/articles/java/index-jsp-135444.html
//! [7]: http://julialang.org
//! [8]: http://julia.readthedocs.org/en/latest/manual/documentation/
//! [9]: http://elixir-lang.org
//! [10]: https://github.com/elixir-lang/ex_doc
//! [11]: http://blog.rust-lang.org/2015/09/17/Rust-1.3.html
//! [12]: https://crates.io
//! [13]: https://github.com/brson/taskcluster-crater
//! [14]: http://semver.org
//! [15]: http://blog.rust-lang.org/2014/10/30/Stability.html
//! [16]: https://doc.rust-lang.org/book/documentation.html
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

/// This is a sample structure, to demonstrate `rustdoc`/`cargo doc`.
///
/// All of this will be attached to the structure definition when you see it
/// live on the site. If you need to document members, you can do it inline!
///
/// It's worth taking a look at the [source] for this: you can see how private
/// and public fields are handled differently in the documentation.
///
/// [source]: /src/show_notes/e001.rs.html
pub struct TVShow {
    /// Here is a string telling us what the theme song was.
    pub theme: String,
    /// Here is the year the show premiered.
    pub year: i32,
    /// Here is the director---but (s)he's rather shy and private.
    director: String,
}

/// You can also document the implementation. This is usually *not* going to be
/// something you particularly need with a `struct`, because you can just
/// attach the documentation to the `struct` itself. However, it *is* the kind
/// of thing that could be useful if you are implementing a `trait`. (More on
/// that in a later episode!)
impl TVShow {
    /// This documents a fairly ho-hum structure constructor.
    ///
    /// ```ignore
    /// let a_struct = TVShow::new("The Answer", 42, "Joss Whedon");
    /// ```
    pub fn new(theme: &str, year: i32, director: &str) -> TVShow {
        TVShow {
            theme: theme.to_string(),
            year: year,
            director: director.to_string(),
        }
    }
}

/// This documents a plain-old function.
///
/// The same basic rules apply as with functions bound to structs; the only
/// real difference is that the docs for this won't be attached to a `struct`
/// but to the module.
pub fn use_members() {
    let firefly = TVShow::new("You can't take the sky from me!", 2001, "Joss Whedon");
    println!(
        "Firefly (come on, sing along: \"{:?}\"): {:?}/{:?}",
        firefly.theme, firefly.year, firefly.director
    );
}
