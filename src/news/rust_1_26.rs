//! Rust 1.26
//!
//!   - **Date:** May 14, 2018
//!   - **Subject:** `impl trait`, `match` on references, `Result`s from `main`,
//!     and more. A good way to mark three years since Rust 1.0!
//!   - **Audio:**
//!       + [M4A][m4a]
//!       + [MP3][mp3]
//!       + [Ogg][ogg]
//!
//! [m4a]: http://www.podtrac.com/pts/redirect.m4a/cdn.newrustacean.com/news/rust_1_26.m4a
//! [mp3]: http://www.podtrac.com/pts/redirect.mp3/cdn.newrustacean.com/news/rust_1_26.mp3
//! [ogg]: http://www.podtrac.com/pts/redirect.ogg/cdn.newrustacean.com/news/rust_1_26.ogg
//!
//! <audio style="width: 100%" style="width: 100%" title="Crates You Should Know: Serde" controls preload=metadata>
//!   <source src="http://www.podtrac.com/pts/redirect.m4a/cdn.newrustacean.com/news/rust_1_26.m4a">
//!   <source src="http://www.podtrac.com/pts/redirect.mp3/cdn.newrustacean.com/news/rust_1_26.mp3">
//!   <source src="http://www.podtrac.com/pts/redirect.ogg/cdn.newrustacean.com/news/rust_1_26.ogg">
//! </audio>
//!
//!
//! Show Notes
//! ----------
//!
//! - Rust 1.25:
//!     - [blog post]
//!     - [release notes]
//! - [RFC #2115]
//! - [The Rusty Spike]
//! - [webassembly.studio]
//! - burntsushi's now-1.0 crates:
//!     - [regex]
//!     - [csv]
//!     - [docopt]
//! - [Search crates VS Code plugin][vsc]
//!
//! [blog post]: https://blog.rust-lang.org/2018/05/10/Rust-1.26.html
//! [release notes]: https://github.com/rust-lang/rust/blob/master/RELEASES.md#version-1260-2018-05-10
//! [RFC #2115]: https://github.com/rust-lang/rfcs/blob/master/text/2115-argument-lifetimes.md
//! [The Rusty Spike]: https://rusty-spike.blubrry.net/
//! [webassembly.studio]: https://webassembly.studio/
//! [regex]: https://crates.io/crates/regex
//! [csv]: https://crates.io/crates/csv
//! [docopt]: https://crates.io/crates/docopt
//! [vsc]: https://marketplace.visualstudio.com/items?itemName=belfz.search-crates-io
//!
//! Sponsors
//! --------
//!
//!   - Aaron Turon
//!   - adsertoris
//!   - Alexander Payne
//!   - [Anthony Deschamps]
//!   - Anthony Scotti
//!   - Antonin Carette
//!   - Aleksey Pirogov
//!   - Andreas Fischer
//!   - Andrew Thompson
//!   - Austin LeSure
//!   - [Behnam Esfahbod]
//!   - Benjamin Wasty
//!   - Brent Vatne
//!   - Brandon "Spanky" Mills
//!   - Brian Casiello
//!   - Chap Lovejoy
//!   - [Charlie Egan]
//!   - Chip
//!   - Chris Jones
//!   - [Chris Palmer]
//!   - [Coleman McFarland]
//!   - Dan Abrams
//!   - [Daniel Collin]
//!   - [Daniel P. Clark]
//!   - [David W. Allen]
//!   - David Hewson
//!   - Derek Buckley
//!   - [Derek Morr]
//!   - Eugene Bulkin
//!   - [Hans Fjällemark]
//!   - [Henri Sivonen]
//!   - [Ian Jones]
//!   - [Jakub "Limeth" Hlusička]
//!   - James Cooper
//!   - Jerome Froelich
//!   - [John Rudnick]
//!   - Jonathan Turner
//!   - Joseph Hain
//!   - [Jupp Müller]
//!   - Justin Ossevoort
//!   - [Karl Hobley]
//!   - Keith Gray
//!   - Kilian Rault
//!   - Laurie Hedge
//!   - Luca Schmid
//!   - [Luiz Irber]
//!   - Mark LeMoine
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
//!   - Raj Venkalil
//!   - [Ramon Buckley]
//!   - Randy MacLeod
//!   - Raph Levien
//!   - reddraggone9
//!   - [Ryan Blecher]
//!   - [Sebastián Ramírez Magrí]
//!   - Shane Utt
//!   - Simon G.
//!   - Steve Jenson
//!   - Steven Knight
//!   - Steven Murawksi
//!   - [Stuart Hinson]
//!   - Tim Brooks
//!   - Timm Preetz
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
//! [Chris Palmer]: http://home.red-oxide.org/
//! [Coleman McFarland]: http://github.com/anxiousmodernman
//! [Daniel Collin]: https://twitter.com/daniel_collin
//! [Daniel P. Clark]: https://6ftdan.com/
//! [David W. Allen]: http://GitHub.com/DataRiot
//! [Derek Morr]: https://twitter.com/derekmorr
//! [Fjällemark]: https://fjallemark.com/
//! [Henri Sivonen]: https://hsivonen.fi/
//! [Ian Jones]: https://www.ianmjones.com/
//! [Jakub "Limeth" Hlusička]: https://github.com/Limeth
//! [John Rudnick]: http://www.cindur.com/
//! [Jupp Müller]: https://de.linkedin.com/in/juppm
//! [Karl Hobley]: https://github.com/kaedroho/
//! [Luiz Irber]: http://luizirber.org/
//! [Max Jacobson]: https://twitter.com/maxjacobson
//! [Messense Lv]: https://github.com/messense
//! [Nathan Sculli]: http://influential.co/
//! [Nick Stevens]: https://github.com/nastevens
//! [Oluseyi Sonaiya]: http://oluseyi.info/
//! [Pascal Hertleif]: https://pascalhertleif.de/
//! [Patrick O'Doherty]: https://twitter.com/patrickod
//! [Philipp Keller]: https://twitter.com/hansapla
//! [Ramon Buckley]: http://www.inosion.com
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

#[doc(include = "../docs/news/rust-1-26.md")]
pub struct Script;
