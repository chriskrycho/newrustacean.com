//! Rust 1.24
//!
//!   - **Date:** March 1, 2018
//!   - **Subject:** Performance wins, incremental compilation, and the Rust 2018 Roadmap and Epoch.
//!   - [**Audio**][mp3]
//!
//! [mp3]: https://www.podtrac.com/pts/redirect.mp3/f001.backblazeb2.com/file/newrustacean/news/rust_1_24.mp3
//!
//! <audio style="width: 100%" title="Crates You Should Know: Serde" controls preload=metadata>
//!   <source src="https://www.podtrac.com/pts/redirect.mp3/f001.backblazeb2.com/file/newrustacean/news/rust_1_24.mp3">
//! </audio>
//!
//!
//! Show Notes
//! ----------
//!
//! - [Rust 1.24.0 blog post][1.24.0]
//! - [Rust 1.24.1 blog post][1.24.1]
//! - [Rust 2018 Roadmap RFC][2018 RFC]
//! - [Rust Epochs RFC][epochs RFC]
//!
//! [1.24.0]: https://blog.rust-lang.org/2018/02/15/Rust-1.24.html
//! [1.24.1]: https://blog.rust-lang.org/2018/03/01/Rust-1.24.1.html
//! [2018 RFC]: https://github.com/rust-lang/rfcs/pull/2314
//! [epochs RFC]: https://github.com/rust-lang/rfcs/pull/2052
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
//!   - [Daniel P. Clark]
//!   - [David W. Allen]
//!   - David Hewson
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
//!   - Shane Utt
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

#[doc(include = "../docs/news/rust-1-24.md")]
pub struct Script;
