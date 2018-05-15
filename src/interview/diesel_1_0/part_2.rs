//! Part 2: The future of Diesel
//!
//!   - **Date:** January 21, 2018
//!   - **Subject:** Getting Diesel to 1.0, writing docs and exposing problems
//!     with the API, improving Diesel in the future, and thinking about API
//!     design for open source libraries in general.
//!   - **Audio:**
//!       + [M4A][m4a]
//!       + [MP3][mp3]
//!       + [Ogg][ogg]
//!
//! [m4a]: http://www.podtrac.com/pts/redirect.m4a/cdn.newrustacean.com/interview/diesel_1_0/part_2.m4a
//! [mp3]: http://www.podtrac.com/pts/redirect.mp3/cdn.newrustacean.com/interview/diesel_1_0/part_2.mp3
//! [ogg]: http://www.podtrac.com/pts/redirect.ogg/cdn.newrustacean.com/interview/diesel_1_0/part_2.ogg
//!
//! <audio style="width: 100%" style="width: 100%" title="Interview: Diesel 1.0, with Sean Griffin, Part 2" controls preload=metadata>
//!   <source src="http://www.podtrac.com/pts/redirect.m4a/cdn.newrustacean.com/interview/diesel_1_0/part_2.m4a">
//!   <source src="http://www.podtrac.com/pts/redirect.mp3/cdn.newrustacean.com/interview/diesel_1_0/part_2.mp3">
//!   <source src="http://www.podtrac.com/pts/redirect.ogg/cdn.newrustacean.com/interview/diesel_1_0/part_2.ogg">
//! </audio>
//!
//!
//! ## Show notes
//!
//! - [Macros 2.0]
//! - The Bike Shed episodes on Diesel 0.99 and 1.0
//!     - [126: Speaking of Compilers...] - where Sean talked about some of the same
//!       changes mentioned on the show here.
//!     - [135: A Series of Unfortunate Examples] - where Sean talks a bit more
//!       about his adventures writing docs for Diesel 1.0 (and the
//!       corresponding feature freeze).
//! - [Generic associated types], which you may have heard of under the name
//!   "associated type constructors"
//! - [Multitenancy]
//! - [Library patterns: multiple levels of abstraction][lib] – Tomas Petricek
//!   on designing library abstractions with a 80%/14%/5%/1% structure.
//! - [blanket implementations], [specialization], [coherence], and [the
//!   lattice rule]
//!
//!
//! [Macros 2.0]: https://github.com/nrc/rfcs/blob/7dcb7374aee3281c261510ca5af53399a3df60f5/text/0000-macros.md
//! [126: Speaking of Compilers...]: http://bikeshed.fm/126
//! [135: A Series of Unfortunate Examples]: http://bikeshed.fm/135
//! [Generic associated types]: https://github.com/rust-lang/rfcs/blob/master/text/1598-generic_associated_types.md
//! [multitenancy]: https://en.wikipedia.org/wiki/Multitenancy
//! [lib]: http://tomasp.net/blog/2015/library-layers/index.html
//! [blanket implementations]: https://doc.rust-lang.org/book/second-edition/ch10-02-traits.html#using-trait-bounds-to-conditionally-implement-methods
//! [specialization]: https://github.com/rust-lang/rfcs/blob/master/text/1210-impl-specialization.md
//! [coherence]: https://aturon.github.io/blog/2017/02/06/specialization-and-coherence/
//! [lattice rule]: https://rust-lang.github.io/rfcs/1210-impl-specialization.html#alternative-specialization-designs
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
//!   - [Daniel P. Clark]
//!   - [David W. Allen]
//!   - David Hewson
//!   - [Derek Morr]
//!   - Eugene Bulkin
//!   - Guido Hoermann
//!   - [Hans Fjällemark]
//!   - Hendrik Sollich
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
//!   - Steven Murawksi
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

/// Transcript – coming at some point!
pub struct Script;
