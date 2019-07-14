//! Sean Griffin on type systems and hopes for Rust's future
//!
//!   - **Date:** February 25, 2016
//!   - **Subject:** Type system strengths and weaknesses, and the weird corners
//!     of Rust (with some hopes for its future)
//!   - [**Audio**][mp3]
//!
//! [mp3]: https://www.podtrac.com/pts/redirect.mp3/cdn.newrustacean.com/file/newrustacean/interview/1/part_2.mp3
//!
//! <audio style="width: 100%" title="Interview 1::Part 1" controls preload=metadata>
//!   <source src="https://www.podtrac.com/pts/redirect.mp3/cdn.newrustacean.com/file/newrustacean/interview/1/part_2.mp3">
//! </audio>
//!
//!
//! Notes
//! -----
//!
//! Chris chats with Sean Griffin about the tradeoffs between mental overhead
//! and type safety, the expressiveness of different type systems, and some of
//! the places where Rust currently falls down.
//!
//!
//! ### Corrigenda
//!
//! Sean noted he could be wrong about `IEnumerable<T>` not having a `Sum`
//! method in C♯, and post-show research indicated that he was (it's possible it
//! was added after he had stopped doing .NET work, of course). See the
//! [documentation][c1] for details on how `IEnumerable<T>.Sum` it behaves in C♯
//! if you're curious.
//!
//! As a related note, I (Chris) have done a little bit of digging on C♯ in the
//! interval and it's fair to say that while a lot of the "ceremony" involved in
//! writing C♯ is annoying, it's much more than just a "slightly nicer Java",
//! and indeed is a much nicer language than my previous, limited exposure had
//! led me to believe. It's no Rust or F♯, but its type system is substantially
//! more capable than Java's.
//!
//! [c1]: https://msdn.microsoft.com/library/bb919210(v=vs.90).aspx
//!
//!
//! Links
//! -----
//!
//!   - fmap
//!       + [Discussion of `fmap` with `Optional` in Swift][l1]
//!       + [In Haskell][l2]
//!   - Rust:
//!       + [Trait objects][l3]
//!       + [Specialization RFC][l4]
//!           * [Implementation][l5]
//!   - [Diesel][l6]
//!
//! [l1]: https://robots.thoughtbot.com/functional-swift-for-dealing-with-optional-values
//! [l2]: http://learnyouahaskell.com/functors-applicative-functors-and-monoids
//! [l3]: https://doc.rust-lang.org/book/trait-objects.html
//! [l4]: https://github.com/rust-lang/rfcs/pull/1210
//! [l5]: https://github.com/rust-lang/rust/issues/31844
//! [l6]: https://github.com/sgrif/diesel
//!
//!
//! Sponsors
//! --------
//!
//!   - Aleksey Pirogov
//!   - Chris Palmer
//!   - [Derek Morr][s3]
//!   - Hamza Sheikh
//!   - Leif Arne Storset
//!   - Luca Schmid
//!   - Micael Bergeron
//!   - Ralph Giles ("rillian")
//!   - reddraggone9
//!   - Ryan Ollos
//!   - [William Roe][s11]
//!
//! [s3]: https://twitter.com/derekmorr
//! [s11]: http://willroe.me
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
//!     + Twitter: [@newrustacean](https://www.twitter.com/newrustacean)
//!     + Email: [hello@newrustacean.com](mailto:hello@newrustacean.com)
//!   - Chris Krycho
//!     + GitHub: [chriskrycho](https://github.com/chriskrycho)
//!     + Twitter: [@chriskrycho](https://www.twitter.com/chriskrycho)
