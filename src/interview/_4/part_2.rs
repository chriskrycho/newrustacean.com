//! Part 2: Making Rust Better
//!
//!   - **Date:** May 30, 2017
//!   - **Subject:** Rust as the fusion of systems and high-level programming
//!     languages, and the RLS.
//!   - [**Audio**][mp3]
//!
//! [mp3]: http://www.podtrac.com/pts/redirect.mp3/f001.backblazeb2.com/file/newrustacean/interview/4/part_2.mp3
//!
//! <audio style="width: 100%" title="Jonathan Turner::Part 2" controls preload=metadata>
//!   <source src="http://www.podtrac.com/pts/redirect.mp3/f001.backblazeb2.com/file/newrustacean/interview/4/part_2.mp3">
//! </audio>
//!
//!
//! Show Notes
//! ----------
//!
//!   - The [survey]
//!   - Language adoption:
//!       + [Guido van Rossum] and [Python]
//!       + [Matz (Yukihiro Matsumoto)][Matz] and [Ruby]
//!       + [Dart]
//!       
//! [survey]: https://blog.rust-lang.org/2016/06/30/State-of-Rust-Survey-2016.html
//! [Guido van Rossum]: https://en.wikipedia.org/wiki/Guido_van_Rossum
//! [Python]: https://www.python.org
//! [Matz]: https://en.wikipedia.org/wiki/Yukihiro_Matsumoto
//! [Ruby]: https://www.ruby-lang.org/en/
//! [Dart]: https://www.dartlang.org
//!
//! Building the Rust Language Service:
//!
//!   - [Racer]
//!   - [rustw]
//!   - [Language Server Protocol][lsp]
//!   - [Demo at RustConf 2016]
//!   - [Anders Hejlsberg] – designer or lead developer of [Turbo Pascal],
//!     [Delphi], [C#], and [TypeScript]
//!   - [Serde]
//!   - [Roadmap GitHub Project]
//!   - Language Server Protocol plugins
//!       + [RLS reference VS Code plugin]
//!       + Kalita Alexey's [vscode-rust]
//!       + [langserver.org]
//!   - [The 2017 Rust Roadmap]
//!       + [Improved match ergonomics around references]
//!       + [const generics]
//!           * [RFC #1931]
//!
//! [Racer]: https://github.com/phildawes/racer
//! [rustw]: https://github.com/nrc/rustw
//! [lsp]: https://github.com/Microsoft/language-server-protocol
//! [Demo at RustConf 2016]: https://youtu.be/pTQxHIzGqFI?t=42m5s
//! [Anders Hejlsberg]: https://en.wikipedia.org/wiki/Anders_Hejlsberg
//! [Turbo Pascal]: https://en.wikipedia.org/wiki/Turbo_Pascal
//! [Delphi]: https://en.wikipedia.org/wiki/Delphi_(programming_language)
//! [C#]: https://docs.microsoft.com/en-us/dotnet/articles/csharp/
//! [TypeScript]: http://www.typescriptlang.org
//! [Serde]: https://serde.rs
//! [Roadmap GitHub Project]: https://github.com/rust-lang-nursery/rls/projects/2
//! [RLS reference VS Code plugin]: https://github.com/jonathandturner/rls_vscode
//! [vscode-rust]: https://github.com/editor-rs/vscode-rust
//! [langserver.org]: http://langserver.org
//! [The 2017 Rust Roadmap]: https://blog.rust-lang.org/2017/02/06/roadmap.html
//! [Improved match ergonomics around references]: https://github.com/rust-lang/rust-roadmap/issues/24
//! [const generics]: https://internals.rust-lang.org/t/lang-team-minutes-const-generics/5090
//! [RFC #1931]: https://github.com/rust-lang/rfcs/pull/1931
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
//!   - Steven Murawksi
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
