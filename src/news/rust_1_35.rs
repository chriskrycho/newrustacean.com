//! Rust 1.35
//!
//! - **Date:** May 23, 2019
//! - **Subject:** WASI, `Option::copied`, and the future of async/await syntax!
//! - [**download mp3**][mp3]
//! - [**script**][script]
//!
//! [mp3]: https://www.podtrac.com/pts/redirect.mp3/f001.backblazeb2.com/file/newrustacean/news/rust_1_35.mp3
//! [script]: https://newrustacean.com/show_notes/news/rust_1_35/struct.script
//!
//! <audio style="width: 100%" title="News: Rust 1.35" controls preload=metadata src="https://www.podtrac.com/pts/redirect.mp3/f001.backblazeb2.com/file/newrustacean/news/rust_1_35.mp3">
//!
//! Show Notes
//! ----------
//!
//! - Rust 1.34.1
//!     - [release notes][1.34.1 release notes]
//!     - [blog post][1.34.1 blog post]
//!     - [discussion about `Error::type_id`]
//! - Rust 1.34.2
//!     - [release notes][1.34.2 release notes]
//!     - [blog post][1.34.2 blog post]
//!     - [discussion about `Error::type_id`]
//! - Rust 1.35
//!     - [release notes]
//!     - [blog post]
//!     - [WASI]
//!         - [The Bike Shed episode]
//! - Async/await
//!     - [`std::futures` API docs]
//!     - [internal thread on syntax for `.await`]
//! 
//! [1.34.1 release notes]: https://github.com/rust-lang/rust/blob/master/RELEASES.md#version-1341-2019-04-25
//! [1.34.1 blog post]: https://blog.rust-lang.org/2019/04/25/Rust-1.34.1.html
//! [1.34.2 release notes]: https://github.com/rust-lang/rust/blob/master/RELEASES.md#version-1342-2019-05-14
//! [1.34.2 blog post]: https://blog.rust-lang.org/2019/05/14/Rust-1.34.2.html
//! [discussion about `Error::type_id`]: https://github.com/rust-lang/rust/issues/60784
//! [release notes]: https://github.com/rust-lang/rust/blob/master/RELEASES.md#version-1350-2019-05-23
//! [blog post]: https://blog.rust-lang.org/2019/05/23/Rust-1.35.0.html
//! [WASI]: https://github.com/CraneStation/wasmtime/blob/master/docs/WASI-intro.md
//! [The Bike Shed episode]: http://bikeshed.fm/195
//! [`std::futures` API docs]: https://rust-lang-nursery.github.io/futures-api-docs/0.3.0-alpha.16/futures/
//! [internal thread on syntax for `.await`]: https://internals.rust-lang.org/t/a-final-proposal-for-await-syntax/10021
//!
//! Sponsors
//! --------
//!
//! Thanks to Parity for sponsoring the show and hiring Rust developers!
//!
//! [parity]: https://www.parity.io/jobs
//!
//! ### Patreon Sponsors
//!
//! - Adam Green
//! - Aleksey Pirogov
//! - Alexander Kryvomaz
//! - Alexander Lozada
//! - Alexander Payne
//! - [Andrew Dirksen]
//! - Andrew Thompson
//! - [Anthony Deschamps]
//! - Anthony Scotti
//! - Arlen Haftevani
//! - [Arlo (Hyena)]
//! - Arun Kulshreshtha
//! - [Behnam Esfahbod]
//! - [Benjamin Manns]
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
//! - Christoffer Ceutz
//! - Cristian Paul
//! - Dan Abrams
//! - Daniel
//! - Daniel Bross
//! - [Daniel Collin]
//! - [Daniel Mason]
//! - David Carroll
//! - David Hewson
//! - [Derek Morr]
//! - Dominic Cooney
//! - Doug Reeves
//! - [Douglas Correa]
//! - Edmund Kump
//! - [Eduard Knyshov]
//! - [Embark Studios]
//! - Eugene Bulkin
//! - [Evan Stoll]
//! - [Fabio (decathorpe)]
//! - [Fabio Correa]
//! - Freeman P. Pascal
//! - [Gaveen Prabhasara]
//! - [Graham Wihlidal]
//! - [Henri Sivonen]
//! - [Ian Jones]
//! - Hoàng Đức Hiếu
//! - [Hugo Josefson]
//! - "Jake ""ferris"" Taylor"
//! - Jako Danar
//! - James Cooper
//! - James Hagans II
//! - [Jason Bowen]
//! - [Jeff May]
//! - [Jendrik Illner]
//! - Jerome Froelich
//! - JockeTF
//! - [Joar Wandborg]
//! - [Johan Andersson]
//! - [John Rudnick]
//! - Jon
//! - Jonah
//! - [Jonathan Knapp]
//! - Jonathan Turner
//! - Joseph Hain
//! - Joseph Mou
//! - Joseph Schrag
//! - [Joe Percy]
//! - Justin Ossevoort
//! - Kai Yao
//! - Kazutaka Mise
//! - Keith Gray
//! - Kilian Rault
//! - Kyle
//! - Lee Jenkins
//! - Luca Schmid
//! - [Luiz Irber]
//! - Lukas Eller
//! - [Malnormalulo]
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
//! - Oladapo Fadeyi
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
//! - Romain Chossart
//! - [Ryan Blecher]
//! - [Ryan Osial]
//! - Satoshi Yoshikawa
//! - Scott Moeller
//! - [Sebastián Ramírez Magrí]
//! - [Simon Dickson]
//! - Simon G
//! - [Soren Bramer Schmidt]
//! - Steve Jenson
//! - Steven Knight
//! - Steven Murawski
//! - [Stuart Hinson]
//! - Sven Huster
//! - Tim Brooks
//! - Tim Small
//! - Tim Süberkrüb
//! - Tom Prince
//! - Toolmaker's Guild
//! - Ty Overby
//! - Tyler Harper
//! - Victor Kruger
//! - Will Greenberg
//! - Zak van der Merwe
//! - Zachary Snyder
//! - [Zach Peters]
//! - Zaki
//!
//! [Andrew Dirksen]: https://github.com/bddap
//! [Anthony Deschamps]: https://github.com/adeschamps
//! [Arlo (Hyena)]: https://asonix.dog/@asonix
//! [Behnam Esfahbod]: https://github.com/behnam
//! [Benjamin Manns]: https://www.benmanns.com/
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
//! [Douglas Correa]: http://learnrust.io/
//! [Eduard Knyshov]: https://github.com/edvorg
//! [Embark Studios]: https://www.embark-studios.com
//! [Evan Stoll]: https://github.com/evanjs
//! [Gaveen Prabhasara]: https://twitter.com/gaveen
//! [Fabio (decathorpe)]: https://decathorpe.com/
//! [Fabio Correa]: https://linkedin.com/in/feamcor
//! [Graham Wihlidal]: https://wihlidal.com/
//! [Henri Sivonen]: https://hsivonen.fi/
//! [Ian Jones]: https://www.ianmjones.com/
//! [Hugo Josefson]: https://www.hugojosefson.com
//! [Jason Bowen]: https://twitter.com/jwbowen
//! [Jeff May]: https://gitlab.com/jeffmay
//! [Jendrik Illner]: https://www.jendrikillner.com/
//! [Joar Wandborg]: http://github.com/joar
//! [Johan Andersson]: https://www.embark-studios.com
//! [Jonathan Knapp]: https://www.coffeeandcode.com/
//! [Joe Percy]: http://joetdc.com/
//! [John Rudnick]: http://www.cindur.com/
//! [Luiz Irber]: http://luizirber.org/
//! [Malnormalulo]: https://twitter.com/Malnormalulo
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
//! [Soren Bramer Schmidt]: http://prisma.io/
//! [Stuart Hinson]: http://stuarth.github.io/
//! [Zach Peters]: https://github.com/zpeters
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

#[doc(include = "../docs/news/rust-1.35.md")]
pub struct Script;
