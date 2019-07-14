//! Rust 1.28
//!
//!   - **Date:** August 16, 2018
//!   - **Subject:** Stable `#[global_allocator]`, more Rust 2018 Edition
//!     schedule news, video learning resources, and a static site generator!
//!   - [**Audio**][mp3]
//!   - [**Script**][script]
//!
//! [mp3]: https://www.podtrac.com/pts/redirect.mp3/cdn.newrustacean.com/file/newrustacean/news/rust_1_28.mp3
//! [script]: https://newrustacean.com/show_notes/news/rust_1_28/struct.script
//!
//! <audio style="width: 100%" title="News – Rust 1.28" controls preload=metadata>
//!   <source src="https://www.podtrac.com/pts/redirect.mp3/cdn.newrustacean.com/file/newrustacean/news/rust_1_28.mp3">
//! </audio>
//!
//!
//! Show Notes
//! ----------
//!
//! - Rust 1.28:
//!     - [blog post]
//!     - [release notes]
//! - `wee_alloc`:
//!     - [repository]
//!     - [blog post][wee bp]
//! - [rustfmt RC]
//! - Community
//!     - [Nick Cameron’s LinuxConfAu 2018 tutorial]
//!     - [Gutenberg]
//!
//! [blog post]: https://blog.rust-lang.org/2018/08/02/Rust-1.28.html
//! [release notes]: https://github.com/rust-lang/rust/blob/1.28.0/RELEASES.md
//! [repository]: https://github.com/rustwasm/wee_alloc
//! [wee bp]: http://fitzgeraldnick.com/2018/02/09/wee-alloc.html
//! [rustfmt RC]: https://www.ncameron.org/blog/rustfmt-1-rc/
//! [Nick Cameron’s LinuxConfAu 2018 tutorial]: https://www.youtube.com/watch?v=vqavdUGKeb4
//! [Gutenberg]: https://getgutenberg.io
//!
//! Sponsors
//! --------
//!
//!   - Aaron Turon
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
//!   - Brian Casiello
//!   - Chap Lovejoy
//!   - [Charlie Egan]
//!   - Chris Jones
//!   - [Chris Palmer]
//!   - [Coleman McFarland]
//!   - [Damien Stanton]
//!   - Dan Abrams
//!   - [Daniel Collin]
//!   - [Daniel Mason]
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
//!   - Joar Wandborg
//!   - [John Rudnick]
//!   - Jon
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
//!   - [Martin Heuschober]:
//!   - Masashi Fujita
//!   - Matt Rudder
//!   - Matthew Brenner
//!   - Matthias Ruszala
//!   - [Max Jacobson]
//!   - [Messense Lv]
//!   - Micael Bergeron
//!   - [Nathan Sculli]
//!   - [Nick Coish]
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
//!   - Robert Chrzanowski
//!   - [Ryan Blecher]
//!   - [Ryan Osial]
//!   - Sascha Grunert
//!   - [Sebastián Ramírez Magrí]
//!   - Shane Utt
//!   - [Simon Dickson]
//!   - Simon G.
//!   - Steve Jenson
//!   - Steven Knight
//!   - Steven Murawski
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
//! [Chris Palmer]: http://red-oxide.org/
//! [Coleman McFarland]: http://github.com/anxiousmodernman
//! [Damien Stanton]: https://github.com/damienstanton
//! [Daniel Collin]: https://twitter.com/daniel_collin
//! [Daniel Mason]: https://github.com/gisleburt
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
//! [Martin Heuschober]: https://github.com/epsilonhalbe
//! [Max Jacobson]: https://twitter.com/maxjacobson
//! [Messense Lv]: https://github.com/messense
//! [Nathan Sculli]: http://influential.co/
//! [Nick Coish]: http://github.com/ncoish
//! [Nick Stevens]: https://github.com/nastevens
//! [Oluseyi Sonaiya]: http://oluseyi.info/
//! [Pascal Hertleif]: https://pascalhertleif.de/
//! [Patrick O'Doherty]: https://twitter.com/patrickod
//! [Philipp Keller]: https://twitter.com/hansapla
//! [Ramon Buckley]: http://www.inosion.com
//! [Ryan Blecher]: http://notryanb.github.io/
//! [Ryan Osial]: https://github.com/osialr
//! [Sebastián Ramírez Magrí]: https://www.twitter.com/sebasmagri
//! [Simon Dickson]: https://www.simonhdickson.com/
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

#[doc(include = "../docs/news/rust-1-28.md")]
pub struct Script;
