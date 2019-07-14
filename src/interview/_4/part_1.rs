//! Part 1: Getting to Rust
//!
//!   - **Date:** April 30, 2017
//!   - **Subject:** Background, TypeScript, coming to Rust, and how helpful the
//!     Rust community can be.
//!   - [**Audio**][mp3]
//!
//! [mp3]: https://www.podtrac.com/pts/redirect.mp3/cdn.newrustacean.com/file/newrustacean/interview/4/part_1.mp3
//!
//! <audio style="width: 100%" title="Jonathan Turner::Part 1" controls preload=metadata>
//!   <source src="https://www.podtrac.com/pts/redirect.mp3/cdn.newrustacean.com/file/newrustacean/interview/4/part_1.mp3">
//! </audio>
//!
//!
//! Show Notes
//! ----------
//!
//! On Jonathan's programming backstory:
//!
//!   - [TI-99/4A]
//!   - [Commodore 64]
//!   - [Cray]
//!       + [Chapel]
//!   - [TypeScript]
//!   - [Yehuda Katz]
//!   - [ECMAScript Language Committee]
//!   - [Data locality]
//!   - [CPPCast]
//!   - [BASIC]
//!   - [Pascal]
//!   - [Ultima]
//!
//! [TI-99/4A]: https://en.wikipedia.org/wiki/Texas_Instruments_TI-99/4A
//! [Commodore 64]: https://en.wikipedia.org/wiki/Commodore_64
//! [Cray]: http://www.cray.com
//! [Chapel]: http://chapel.cray.com
//! [TypeScript]: http://www.typescriptlang.org
//! [Yehuda Katz]: http://yehudakatz.com
//! [ECMAScript Language Committee]: https://www.ecma-international.org/memento/TC39.htm
//! [Data locality]: https://en.wikipedia.org/wiki/Locality_of_reference
//! [CPPCast]: http://cppcast.com
//! [BASIC]: https://en.wikipedia.org/wiki/BASIC
//! [Pascal]: https://en.wikipedia.org/wiki/Pascal_(programming_language)
//! [Ultima]: https://en.wikipedia.org/wiki/Ultima_%28series%29
//!
//!
//! After the transition to working on Rust full-time:
//!
//!   - Improving the error messages—
//!       + [design issue]
//!       + Jonathan's personal blog post ["Helping with the Rust Errors"]
//!       + Official Rust blog post announcing and describing the feature,
//!         ["Shape of Errors to Come"]
//!       + [Elm]
//!       + [error list issue]
//!
//! [design issue]: https://github.com/rust-lang/rust/issues/33240
//! ["Helping with the Rust Errors"]: http://www.jonathanturner.org/2016/08/helping-out-with-rust-errors.html
//! ["Shape of Errors to Come"]: https://blog.rust-lang.org/2016/08/10/Shape-of-errors-to-come.html
//! [Elm]: http://elm-lang.org
//! [error list issue]: https://github.com/rust-lang/rust/issues/35233
//!
//!
//! Sponsors
//! --------
//!
//!   - [Anthony Deschamps]
//!   - Anthony Scotti
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
//!   - [Henri Sivonen]
//!   - [Jakub "Limeth" Hlusička]
//!   - Jonathan Turner
//!   - Jordan Henderson
//!   - [Jupp Müller]
//!   - Justin Ossevoort
//!   - [Karl Hobley]
//!   - Keith Gray
//!   - Kilian Rault
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
//!   - Ty Overby
//!   - Tyler Harper
//!   - Vesa Kaihlavirta
//!   - Warren Harper
//!   - [William Roe]
//!   - Zaki
//!
//! [Anthony Deschamps]: https://github.com/adeschamps
//! [Charlie Egan]: https://charlieegan3.com
//! [Chris Palmer]: http://red-oxide.org/
//! [Christopher Giffard]: http://blog.cgiffard.com
//! [Daniel Collin]: https://twitter.com/daniel_collin
//! [Derek Morr]: https://twitter.com/derekmorr
//! [Henri Sivonen]: https://hsivonen.fi/
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
