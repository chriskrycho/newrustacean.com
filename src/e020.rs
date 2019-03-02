//! Putting code in its place
//!
//!   - **Date:** April 1, 2017
//!   - **Subject:** How do we organize code in Rust? Where do we break it apart
//!     into modules or crates, and why?
//!   - [**Audio**][mp3]
//!
//! [mp3]: https://www.podtrac.com/pts/redirect.mp3/f001.backblazeb2.com/file/newrustacean/e020.mp3
//!
//! <audio style="width: 100%" title="Putting code in its place" controls preload=metadata>
//!   <source src="https://www.podtrac.com/pts/redirect.mp3/f001.backblazeb2.com/file/newrustacean/e020.mp3">
//! </audio>
//!
//!
//! Notes
//! -----
//!
//! Structuring code in a language like Rust can seem a bit more ambiguous than
//! doing the same in a language with classes to attach all our functionality
//! to, but in practice, the concerns are much the same: modules are namespaces,
//! and we group by *responsibility*. In today's episode, I talk through that
//! philosophy (and give some comparisons to other languages), and then look at
//! what it looks like in practice!
//!
//!
//! Links
//! -----
//!
//!   - [Learning Rust Modules], by Jeff Walker, has a nice comparison of C#
//!     namespaces and Rust modules.
//!   - [The commit on Lightning inspired by this episode][lightning].
//!
//! [Learning Rust Modules]: http://walkercoderanger.com/blog/2015/08/learning-rust-modules/
//! [lightning]: https://github.com/chriskrycho/lightning-rs/commit/fac341d1c1b4872d62ec05253ee33f056e67d6ce
//!
//!
//! Sponsors
//! --------
//!
//!
//!   - Aleksey Pirogov
//!   - Andreas Fischer
//!   - Andrew Thompson
//!   - Austin LeSure
//!   - Ben Whitley
//!   - [Charlie Egan]
//!   - Chris Jones
//!   - [Chris Palmer]
//!   - [Christopher Giffard]
//!   - Dan Abrams
//!   - [Daniel Collin]
//!   - [Derek Morr]
//!   - Eugene Bulkin
//!   - [Jakub "Limeth" Hlusička]
//!   - Jordan Henderson
//!   - [Jupp Müller]
//!   - Justin Ossevoort
//!   - [Karl Hobley]
//!   - Keith Gray
//!   - Lachlan Collins
//!   - Luca Schmid
//!   - Matt Rudder
//!   - Matthew Piziak
//!   - [Max Jacobson]
//!   - Micael Bergeron
//!   - Ovidiu Curcan
//!   - [Pascal Hertleif]
//!   - [Patrick O'Doherty]
//!   - Peter Tillemans
//!   - Philipp Keller
//!   - Ralph Giles ("rillian")
//!   - Raph Levien
//!   - reddraggone9
//!   - Steven Murawski
//!   - [Stuart Hinson]
//!   - Tyler Harper
//!   - Vesa Kaihlavirta
//!   - Vlad Bezden
//!   - Warren Harper
//!   - [William Roe]
//!   - Zaki
//!
//! [Charlie Egan]: https://charlieegan3.com
//! [Chris Palmer]: http://red-oxide.org/
//! [Christopher Giffard]: http://blog.cgiffard.com
//! [Daniel Collin]: https://twitter.com/daniel_collin
//! [Derek Morr]: https://twitter.com/derekmorr
//! [Jakub "Limeth" Hlusička]: https://github.com/Limeth
//! [Jupp Müller]: https://de.linkedin.com/in/juppm
//! [Karl Hobley]: https://github.com/kaedroho/
//! [Max Jacobson]: https://twitter.com/maxjacobson
//! [Pascal Hertleif]: https://pascalhertleif.de/
//! [Patrick O'Doherty]: https://twitter.com/patrickod
//! [Philipp Keller]: https://twitter.com/hansapla
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
