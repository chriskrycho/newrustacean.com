//! Rust 1.32
//!
//! - **Date:** January 22, 2019
//! - **Subject:** `dbg!`, unified paths, more places you can use `Self`, and a
//!   *bunch* of `const fn` stabilizations—plus some neat community highlights!
//! - [**download mp3**][mp3]
//! - [**script**][script]
//!
//! [mp3]: https://www.podtrac.com/pts/redirect.mp3/cdn.newrustacean.com/file/newrustacean/news/rust_1_32.mp3
//! [script]: https://newrustacean.com/show_notes/news/rust_1_32/struct.script
//!
//! <audio style="width: 100%" title="News: Rust 1.31, Part II" controls preload=metadata src="https://www.podtrac.com/pts/redirect.mp3/cdn.newrustacean.com/file/newrustacean/news/rust_1_32.mp3"></audio>
//!
//! Show Notes
//! ----------
//!
//! - Rust 1.32
//!     - [blog post]
//!     - [release notes]
//!     - [smaller builds on Twitter][Twitter]
//! - [Amethyst]
//!     - [examples]
//!     - [docs]
//!     - [RustConf 2018 keynote]
//! - [insta]
//!
//! [blog post]: https://blog.rust-lang.org/2019/01/17/Rust-1.32.0.html
//! [release notes]: https://github.com/rust-lang/rust/blob/master/RELEASES.md#version-1320-2019-01-17
//! [Twitter]: https://twitter.com/softprops/status/1086723200095059972
//! [Amethyst]: https://www.amethyst.rs/
//! [examples]: https://github.com/amethyst/amethyst/tree/v0.10.0/examples
//! [docs]: https://www.amethyst.rs/book/latest/
//! [RustConf 2018 keynote]: https://m.youtube.com/watch?v=aKLntZcp27M
//! [insta]: https://crates.io/crates/insta
//!
//! Sponsors
//! --------
//!
//! Thanks to Parity for sponsoring the show again. Go check out their [***Rust
//! jobs***][parity]!
//!
//! [parity]: https://paritytech.io/jobs/
//!
//! ### Patreon Sponsors
//!
//! - Adam Green
//! - Aleksey Pirogov
//! - Alexander Kryvomaz
//! - Alexander Payne
//! - [Andrew Dirksen]
//! - Andrew Thompson
//! - [Anthony Deschamps]
//! - Anthony Scotti
//! - Arlen Haftevani
//! - [beaorn]
//! - [Behnam Esfahbod]
//! - Benjamin Wasty
//! - Brandon 'Spanky' Mills
//! - Brian Casiello
//! - Brian Manning
//! - [Brian McCallister]
//! - [Bryan Stitt]
//! - Caryn Finkelman
//! - Cass Costello
//! - Cat Dad
//! - Chap Lovejoy
//! - [Charlie Egan]
//! - Chip
//! - [Chris Palmer]
//! - Dan Abrams
//! - Daniel
//! - Daniel Bross
//! - [Daniel Collin]
//! - [Daniel Mason]
//! - David Hewson
//! - [Derek Morr]
//! - Doug Reeves
//! - Douglas Correa
//! - [Eduard Knyshov]
//! - [Embark Studios]
//! - Eugene Bulkin
//! - [Fabio (decathorpe)]
//! - [Gaveen Prabhasara]
//! - [Graham Wihlidal]
//! - [Henri Sivonen]
//! - [Ian Jones]
//! - Hoàng Đức Hiếu
//! - "Jake ""ferris"" Taylor"
//! - Jako Danar
//! - James Cooper
//! - James Hagans II
//! - Jerome Froelich
//! - [Joar Wandborg]
//! - [Johan Andersson]
//! - [John Rudnick]
//! - Jon
//! - Jonah
//! - [Jonathan Knapp]
//! - Jonathan Turner
//! - Joseph Hain
//! - [Joe Percy]
//! - Justin Ossevoort
//! - Kai Yao
//! - Keith Gray
//! - Kilian Rault
//! - Lee Jenkins
//! - Luca Schmid
//! - [Luiz Irber]
//! - Lukas Eller
//! - [Martin Heuschober]
//! - Masashi Fujita
//! - Matt Rudder
//! - Matthew Brenner
//! - Matthias Ruszala
//! - [Max Jacobson]
//! - Max R.R. Collada
//! - [Messense Lv]
//! - Micael Bergeron
//! - [Michael Mc Donnell]
//! - [Michael Melanson]
//! - Michael Sanders
//! - [Nathan Sculli]
//! - [Nick Coish]
//! - Nick Gideo
//! - [Nick Stevens]
//! - [Nicolas Pochet]
//! - Olaf Leidinger
//! - Oliver Uvman
//! - [Oluseyi Sonaiya]
//! - Ovidiu Curcan
//! - [Pascal]
//! - [Patrick O'Doherty]
//! - Paul Naranja
//! - Paul Osborne
//! - Peter Scholtens
//! - Peter Tillemans
//! - Pierre-Antoine Champin
//! - Ralph Giles
//! - [Ramon Buckland]
//! - Randy MacLeod
//! - Raph Levien
//! - Richard Dallaway
//! - Rob Tsuk
//! - [Robbie Clarken]
//! - Robert Chrzanowski
//! - [Ryan Blecher]
//! - [Ryan Osial]
//! - Scott Moeller
//! - [Sebastián Ramírez Magrí]
//! - [Simon Dickson]
//! - Simon G
//! - [Steffen Loen Sunde]
//! - Steve Jenson
//! - Steven Knight
//! - Steven Murawski
//! - [Stuart Hinson]
//! - Tim Brooks
//! - Tim Süberkrüb
//! - Tom Prince
//! - Toolmaker's Guild
//! - Ty Overby
//! - Tyler Harper
//! - Victor Kruger
//! - Will Greenberg
//! - [William Roe]
//! - Zak van der Merwe
//! - Zachary Snyder
//! - Zaki
//!
//! [Andrew Dirksen]: https://github.com/bddap
//! [Anthony Deschamps]: https://github.com/adeschamps
//! [beaorn]: https://github.com/beaorn
//! [Behnam Esfahbod]: https://github.com/behnam
//! [Brian McCallister]: https://skife.org/
//! [Bryan Stitt]: http://www.stitthappens.com/
//! [Charlie Egan]: https://charlieegan3.com
//! [Chris Palmer]: http://red-oxide.org/
//! [Damien Stanton]: https://github.com/damienstanton
//! [Daniel Collin]: https://twitter.com/daniel_collin
//! [Daniel Mason]: https://github.com/gisleburt
//! [Daniel P. Clark]: https://6ftdan.com/
//! [David W. Allen]: http://GitHub.com/DataRiot
//! [Derek Morr]: https://twitter.com/derekmorr
//! [Eduard Knyshov]: https://github.com/edvorg
//! [Embark Studios]: https://www.embark-studios.com
//! [Gaveen Prabhasara]: https://twitter.com/gaveen
//! [Fabio (decathorpe)]: https://decathorpe.com/
//! [Graham Wihlidal]: https://wihlidal.com/
//! [Henri Sivonen]: https://hsivonen.fi/
//! [Ian Jones]: https://www.ianmjones.com/
//! [Joar Wandborg]: http://github.com/joar
//! [Johan Andersson]: https://www.embark-studios.com
//! [Jonathan Knapp]: https://www.coffeeandcode.com/
//! [Joe Percy]: http://joetdc.com/
//! [John Rudnick]: http://www.cindur.com/
//! [Luiz Irber]: http://luizirber.org/
//! [Martin Heuschober]: https://github.com/epsilonhalbe
//! [Max Jacobson]: https://twitter.com/maxjacobson
//! [Messense Lv]: https://github.com/messense
//! [Michael Mc Donnell]: https://www.linkedin.com/in/michaelmcdonnell/
//! [Michael Melanson]: https://www.michaelmelanson.net
//! [Nathan Sculli]: http://influential.co/
//! [Nick Coish]: http://github.com/ncoish
//! [Nick Stevens]: https://github.com/nastevens
//! [Nicolas Pochet]: https://github.com/n-pochet
//! [Oluseyi Sonaiya]: http://oluseyi.info/
//! [Pascal]: https://pascalhertleif.de/
//! [Patrick O'Doherty]: https://twitter.com/patrickod
//! [Philipp Keller]: https://twitter.com/hansapla
//! [Ramon Buckland]: http://www.inosion.com
//! [Robbie Clarken]: https://github.com/RobbieClarken/
//! [Ryan Blecher]: http://notryanb.github.io/
//! [Ryan Osial]: https://github.com/osialr
//! [Sebastián Ramírez Magrí]: https://www.twitter.com/sebasmagri
//! [Simon Dickson]: https://www.simonhdickson.com/
//! [Steffen Loen Sunde]: https://www.ntnu.edu/employees/steffen.sunde
//! [Stuart Hinson]: http://stuarth.github.io/
//! [William Roe]: http://willroe.me
//!
//! (Thanks to the couple people donating who opted out of the reward tier, as
//! well. You know who you are!)
//!
//! ### Become a sponsor
//!
//! - <a href="https://www.patreon.com/newrustacean" rel="payment">Patreon</a>
//! - [Venmo](https://venmo.com/chriskrycho)
//! - [Dwolla](https://www.dwolla.com/hub/chriskrycho)
//! - [Cash.me](https://cash.me/$chriskrycho)
//! - [Flattr](https://flattr.com/profile/chriskrycho)
//! - [PayPal.me](https://paypal.me/chriskrycho)
//!
//!
//! Contact
//! -------
//!
//! - New Rustacean:
//!     + Twitter: [@newrustacean](https://www.twitter.com/newrustacean)
//!     + Email: [hello@newrustacean.com](mailto:hello@newrustacean.com)
//! - Chris Krycho
//!     + GitHub: [chriskrycho](https://github.com/chriskrycho)
//!     + Twitter: [@chriskrycho](https://www.twitter.com/chriskrycho)

#[doc = include_str!("../../docs/news/rust-1-32.md")]
pub struct Script;
