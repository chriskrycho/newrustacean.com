//! Rust 1.23
//!
//!   - **Date:** January 5, 2018
//!   - **Subject:** Rustdoc changes, the first `impl` period, Firefox Quantum, and more wasm!
//!   - [**Audio**][mp3]
//!
//! [mp3]: https://www.podtrac.com/pts/redirect.mp3/f001.backblazeb2.com/file/newrustacean/news/rust_1_23.mp3
//!
//! <audio style="width: 100%" title="News: Rust 1.20" controls preload=metadata src="https://www.podtrac.com/pts/redirect.mp3/f001.backblazeb2.com/file/newrustacean/news/rust_1_23.mp3" />
//!
//!
//! ## Show notes
//!
//! - Rust 1.23
//!     * [1.23 release notes]
//!     * [rustdoc tracking issue]
//!     * [rustdoc blog post]
//!     * [not copying function arguments]
//!     * [the first news episode]
//! - the `impl` period
//!     * [`impl` period announcement]
//!     * [final newsletter]
//!     * [Diesel ORM]
//! - [Firefox Quantum]
//!     * ["Fearless Concurrency in Firefox Quantum"]
//!     * [e015: Not dumb pointers.]
//! - WebAssembly
//!     * [classic asteroids game]
//!     * [a password generator]
//!     * [Yew]
//!     * [JSX]
//!     * [stdweb]
//!     * [Glimmer VM spike]
//!     * ["Rust and the Case for WebAssembly in 2018"]
//! - ["New Year’s Rust: A Call for Community Blogposts"]
//! - [my other podcast, Winning Slowly]
//!
//! [1.23 release notes]: https://github.com/rust-lang/rust/blob/50989cd98dbef60b0b6f5baa0ce4203ce778adaa/RELEASES.md#version-1230-2018-01-04
//! [rustdoc tracking issue]: https://github.com/rust-lang/rust/issues/44229
//! [rustdoc blog post]: https://blog.guillaume-gomez.fr/articles/2017-09-18+New+rustdoc+rendering+common+errors
//! [not copying function arguments]: https://github.com/rust-lang/rust/pull/45380
//! [the first news episode]: https://www.newrustacean.com/show_notes/news/_1/index.html
//! [`impl` period announcement]: https://blog.rust-lang.org/2017/09/18/impl-future-for-rust.html
//! [final newsletter]: https://internals.rust-lang.org/t/the-final-impl-period-newsletter/6408
//! [diesel orm]: http://diesel.rs/
//! [firefox quantum]: https://www.mozilla.org/en-US/firefox/quantum/
//! ["fearless concurrency in firefox quantum"]: https://blog.rust-lang.org/2017/11/14/Fearless-Concurrency-In-Firefox-Quantum.html
//! [e015: not dumb pointers.]: https://www.newrustacean.com/show_notes/e015/index.html "Not dumb pointers"
//! [classic asteroids game]: https://aochagavia.github.io/blog/rocket---a-rust-game-running-on-wasm/
//! [a password generator]: https://arkada38.github.io/2017/12/22/a-rust-password-generator-on-wasm/
//! [Yew]: https://github.com/DenisKolodin/yew
//! [JSX]: https://reactjs.org/docs/introducing-jsx.html
//! [stdweb]: https://github.com/koute/stdweb
//! [Glimmer VM spike]: https://github.com/glimmerjs/glimmer-vm/pull/752
//! ["rust and the case for webassembly in 2018"]: https://mgattozzi.com/rust-wasm
//! ["new year’s rust: a call for community blogposts"]: https://blog.rust-lang.org/2018/01/03/new-years-rust-a-call-for-community-blogposts.html
//! [my other podcast, winning slowly]: http://www.winningslowly.org "Winning Slowly"
//!
//!
//! Sponsors
//! --------
//!
//!   - Aaron Turon
//!   - Alexander Payne
//!   - [Anthony Deschamps]
//!   - Anthony Scotti
//!   - Aleksey Pirogov
//!   - Andreas Fischer
//!   - Andrew Thompson
//!   - Austin LeSure
//!   - [Behnam Esfahbod]
//!   - Benjamin Wasty
//!   - Brent Vatne
//!   - Chap Lovejoy
//!   - [Charlie Egan]
//!   - Chris Jones
//!   - [Chris Palmer]
//!   - [Coleman McFarland]
//!   - Dan Abrams
//!   - [Daniel Collin]
//!   - Daniel P. Clark
//!   - [David W. Allen]
//!   - David Hewson
//!   - [Derek Morr]
//!   - Eugene Bulkin
//!   - [Henri Sivonen]
//!   - [Ian Jones]
//!   - [Jakub "Limeth" Hlusička]
//!   - James Cooper
//!   - Jerome Froelich
//!   - [John Rudnick]
//!   - Jonathan Turner
//!   - [Jupp Müller]
//!   - Justin Ossevoort
//!   - [Karl Hobley]
//!   - Keith Gray
//!   - Kilian Rault
//!   - Luca Schmid
//!   - Masashi Fujita
//!   - Matt Rudder
//!   - Matthew Brenner
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
//!   - [Paul Naranja]
//!   - Peter Tillemans
//!   - Ralph Giles ("rillian")
//!   - Randy MacLeod
//!   - Raph Levien
//!   - reddraggone9
//!   - [Ryan Blecher]
//!   - [Sebastián Ramírez Magrí]
//!   - Simon G.
//!   - Steven Murawski
//!   - [Stuart Hinson]
//!   - Tim Brooks
//!   - Tom Prince
//!   - Ty Overby
//!   - Tyler Harper
//!   - Vesa Kaihlavirta
//!   - Victor Kruger
//!   - Will Greenberg
//!   - [William Roe]
//!   - Yaacov Finkelman
//!   - Zachary Snyder
//!   - Zaki
//!
//! [Anthony Deschamps]: https://github.com/adeschamps
//! [Behnam Esfahbod]: https://github.com/behnam
//! [Brent Vatne]: https://github.com/brentvatne
//! [Charlie Egan]: https://charlieegan3.com
//! [Chris Palmer]: http://red-oxide.org/
//! [Coleman McFarland]: http://github.com/anxiousmodernman
//! [Daniel Collin]: https://twitter.com/daniel_collin
//! [David W. Allen]: http://GitHub.com/DataRiot
//! [Derek Morr]: https://twitter.com/derekmorr
//! [Henri Sivonen]: https://hsivonen.fi/
//! [Ian Jones]: https://www.ianmjones.com/
//! [Jakub "Limeth" Hlusička]: https://github.com/Limeth
//! [John Rudnick]: http://www.cindur.com/
//! [Jupp Müller]: https://de.linkedin.com/in/juppm
//! [Karl Hobley]: https://github.com/kaedroho/
//! [Max Jacobson]: https://twitter.com/maxjacobson
//! [Messense Lv]: https://github.com/messense
//! [Nathan Sculli]: http://influential.co/
//! [Nick Stevens]: https://github.com/nastevens
//! [Oluseyi Sonaiya]: http://oluseyi.info/
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
//!   - <a href="https://www.patreon.com/newrustacean" rel="payment">Patreon</a>
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
//!     + Twitter: [@newrustacean](https://www.twitter.com/newrustacean)
//!     + Email: [hello@newrustacean.com](mailto:hello@newrustacean.com)
//!   - Chris Krycho
//!     + GitHub: [chriskrycho](https://github.com/chriskrycho)
//!     + Twitter: [@chriskrycho](https://www.twitter.com/chriskrycho)

#[rustc::nightly]
#[doc(include = "../docs/news/rust-1-23.md")]
pub struct Script;
