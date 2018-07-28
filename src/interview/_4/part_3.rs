//! Part 3: Smoothing the Rust dev story
//!
//!   - **Date:** June 17, 2017
//!   - **Subject:** Future work on the RLS, in Rust itself, and in Servo.
//!   - [**Audio**][mp3]
//!
//! [mp3]: http://www.podtrac.com/pts/redirect.mp3/f001.backblazeb2.com/file/newrustacean/interview/4/part_3.mp3
//!
//! <audio style="width: 100%" title="Jonathan Turner::Part 3" controls preload=metadata>
//!   <source src="http://www.podtrac.com/pts/redirect.mp3/f001.backblazeb2.com/file/newrustacean/interview/4/part_3.mp3">
//! </audio>
//!
//!
//! Show Notes
//! ----------
//!
//! Building the Rust Language Service:
//!
//!   - Language Server Protocol plugins
//!       + [RLS reference VS Code plugin]
//!       + Kalita Alexey's [vscode-rust]
//!       + [langserver.org]
//!   - [The 2017 Rust Roadmap]
//!       + [Improved match ergonomics around references]
//!       + [const generics]
//!           * [RFC #1931]
//!
//! [RLS reference VS Code plugin]: https://github.com/jonathandturner/rls_vscode
//! [vscode-rust]: https://github.com/editor-rs/vscode-rust
//! [langserver.org]: http://langserver.org
//! [The 2017 Rust Roadmap]: https://blog.rust-lang.org/2017/02/06/roadmap.html
//! [Improved match ergonomics around references]: https://github.com/rust-lang/rust-roadmap/issues/24
//! [const generics]: https://internals.rust-lang.org/t/lang-team-minutes-const-generics/5090
//! [RFC #1931]: https://github.com/rust-lang/rfcs/pull/1931
//!
//!
//! Working on Servo:
//!
//!   - [Servo]
//!       + [Windows nightlies]
//!   - [LLVM]
//!       + Apple's use on their graphics pipeline:
//!           - [OpenGL]
//!           - [Metal]
//!       + [clang]
//!       + [Swift]
//!   - [Project Quantum]
//!   - [WebKit]
//!       + [KHTML]
//!       + [Safari]
//!   
//! [Servo]: https://servo.org
//! [Windows nightlies]: https://blog.servo.org/2017/04/13/windows/
//! [llvm]: http://llvm.org
//! [OpenGL]: http://lists.llvm.org/pipermail/llvm-dev/2006-August/006497.html
//! [Metal]: https://developer.apple.com/metal/metal-shading-language-specification.pdf
//! [clang]: http://clang.llvm.org
//! [swift]: https://swift.org
//! [Project Quantum]: https://medium.com/mozilla-tech/a-quantum-leap-for-the-web-a3b7174b3c12
//! [WebKit]: https://webkit.org
//! [KHTML]: https://en.wikipedia.org/wiki/KHTML
//! [Safari]: https://www.apple.com/safari/
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
//!   - [Behnam Esfahbod]
//!   - Ben Whitley
//!   - Benjamin Wasty
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
//!   - Randy MacLeod
//!   - Raph Levien
//!   - reddraggone9
//!   - Steven Murawksi
//!   - [Stuart Hinson]
//!   - Tim Brooks
//!   - Tom Prince
//!   - Ty Overby
//!   - Tyler Harper
//!   - Vesa Kaihlavirta
//!   - Warren Harper
//!   - [William Roe]
//!   - Zaki
//!
//! [Anthony Deschamps]: https://github.com/adeschamps
//! [Behnam Esfahbod]: https://github.com/behnam
//! [Charlie Egan]: https://charlieegan3.com
//! [Chris Palmer]: http://home.red-oxide.org/
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
