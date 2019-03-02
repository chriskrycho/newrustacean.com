//! Rust 1.21 and 1.22
//!
//!   - **Date:** November 24, 2017
//!   - **Subject:** Quality of life improvements, `Failure`, wasm, and rustdoc fun.
//!   - [**Audio**][mp3]
//!
//! [mp3]: https://www.podtrac.com/pts/redirect.mp3/f001.backblazeb2.com/file/newrustacean/news/rust_1_21_1_22.mp3
//!
//! <audio style="width: 100%" title="News: Rust 1.20" controls preload=metadata>
//!   <source src="https://www.podtrac.com/pts/redirect.mp3/f001.backblazeb2.com/file/newrustacean/news/rust_1_21_1_22.mp3">
//! </audio>
//!
//!
//! Links and Notes
//! ---------------
//!
//! - Rust 1.21
//!     + [blog post][1.21]
//!     + ["When Will the RLS be Released?"][rls]
//! - [Rust 1.22][1.22]
//! - [Rusty Spike]
//! - [Rust Fest]
//! - [Rust Belt Rust]
//!     + [YouTube channel]
//!     + my talk: [script] and [slides]
//! - [the `Failure` crate]
//!     + prior art: [error-chain]
//! - [wasm support in Rust itself][wasm]
//! - external Markdown files for docs
//!     + [the RFC]
//!     + [the implementation]
//!
//! [1.21]: https://blog.rust-lang.org/2017/10/12/Rust-1.21.html
//! [rls]: https://www.ncameron.org/blog/when-will-the-rls-be-released/
//! [1.22]: https://blog.rust-lang.org/2017/11/22/Rust-1.22.html
//! [Rusty Spike]: https://rusty-spike.blubrry.net
//! [Rust Fest]: http://www.rustfest.eu
//! [Rust Belt Rust]: http://rust-belt-rust.com
//! [YouTube Channel]: https://www.youtube.com/channel/UCptxtVyJkQAJZcFwBbIDZcg
//! [script]: http://www.chriskrycho.com/2017/becoming-a-contributor.html
//! [slides]: http://www.chriskrycho.com/talks/rust-belt-rust/
//! [the `Failure` crate]: https://github.com/withoutboats/failure
//! [error-chain]: https://github.com/rust-lang-nursery/error-chain
//! [wasm]: https://github.com/rust-lang/rust/pull/45905
//! [the RFC]: https://github.com/rust-lang/rfcs/pull/1990
//! [the implementation]: https://github.com/rust-lang/rust/pull/44781
//!
//! Sponsors
//! --------
//!
//!   - Aaron Turon
//!   - Alexander Payne
//!   - [Anthony Deschamps]
//!   - Anthony Scotti
//!   - Anton Van Moere
//!   - Aleksey Pirogov
//!   - Andreas Fischer
//!   - Andrew Thompson
//!   - Austin LeSure
//!   - [Behnam Esfahbod]
//!   - Benjamin Wasty
//!   - Brent Vatne
//!   - Caryn Finkelman
//!   - [Charlie Egan]
//!   - Chris Jones
//!   - Christian Schwarz
//!   - [Chris Palmer]
//!   - Dan Abrams
//!   - [Daniel Collin]
//!   - [David W. Allen]
//!   - David Hewson
//!   - [Derek Morr]
//!   - Eugene Bulkin
//!   - [Henri Sivonen]
//!   - [Ian Jones]
//!   - [Jakub "Limeth" Hlusička]
//!   - James Cooper
//!   - Jerome Froelich
//!   - John Chandler
//!   - Jonathan Turner
//!   - [Jupp Müller]
//!   - Justin Ossevoort
//!   - [Karl Hobley]
//!   - Keith Gray
//!   - Kilian Rault
//!   - Luca Schmid
//!   - Masashi Fujita
//!   - Matt Rudder
//!   - Matthew Bettcher
//!   - Matthew Brenner
//!   - Matthew Piziak
//!   - Matthias Ruszala
//!   - [Max Jacobson]
//!   - [Messense Lv]
//!   - Micael Bergeron
//!   - [Nathan Sculli]
//!   - [Nick Stevens]
//!   - [Oluseyi Sonaiya]
//!   - Ovidiu Curcan
//!   - [Pascal Hertleif]
//!   - [Patrick O'Doherty]
//!   - Peter Tillemans
//!   - Peter Zuidhoek
//!   - Ralph Giles ("rillian")
//!   - Randy MacLeod
//!   - Raph Levien
//!   - reddraggone9
//!   - [Ryan Blecher]
//!   - Sam Whited
//!   - [Sebastián Ramírez Magrí]
//!   - Simon G.
//!   - Steven Murawski
//!   - [Stuart Hinson]
//!   - Tim Brooks
//!   - Tom Prince
//!   - Ty Overby
//!   - Tyler Harper
//!   - Vesa Kaihlavirta
//!   - [William Roe]
//!   - Zachary Snyder
//!   - Zaki
//!
//! [Anthony Deschamps]: https://github.com/adeschamps
//! [Behnam Esfahbod]: https://github.com/behnam
//! [Brent Vatne]: https://github.com/brentvatne
//! [Charlie Egan]: https://charlieegan3.com
//! [Chris Palmer]: http://red-oxide.org/
//! [Daniel Collin]: https://twitter.com/daniel_collin
//! [David W. Allen]: http://GitHub.com/DataRiot
//! [Derek Morr]: https://twitter.com/derekmorr
//! [Henri Sivonen]: https://hsivonen.fi/
//! [Ian Jones]: https://www.ianmjones.com/
//! [Jakub "Limeth" Hlusička]: https://github.com/Limeth
//! [Jupp Müller]: https://de.linkedin.com/in/juppm
//! [Karl Hobley]: https://github.com/kaedroho/
//! [Max Jacobson]: https://twitter.com/maxjacobson
//! [Messense Lv]: https://github.com/messense
//! [Nathan Sculli]: http://influential.co/
//! [Nick Stevens]: https://github.com/nastevens
//! [Oluseyi Sonaiya]: http://oluseyi.info
//! [Pascal Hertleif]: https://pascalhertleif.de/
//! [Patrick O'Doherty]: https://twitter.com/patrickod
//! [Philipp Keller]: https://twitter.com/hansapla
//! [Ryan Blecher]: http://notryanb.github.io/
//! [Sebastián Ramírez Magrí]: https://www.twitter.com/sebasmagri
//! [Stuart Hinson]: http://stuarth.github.io/
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
//!   - [PayPal.me](https://paypal.me/chriskrycho)
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

#[doc(include = "../docs/news/rust-1-21-1-22.md")]
pub struct Script;
