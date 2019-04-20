//! Rust 1.29 and 1.30
//!
//! - **Date:** November 17, 2018
//! - **Subject:** Procedural macros, better paths, and the Rust 2018 beta!
//! - [**download mp3**][mp3]
//! - [**script**][script]
//!
//! [mp3]: https://www.podtrac.com/pts/redirect.mp3/f001.backblazeb2.com/file/newrustacean/news/rust_1_29_1_30.mp3
//! [script]: https://newrustacean.com/show_notes/news/rust_1_29_1_30/struct.script
//!
//! <audio style="width: 100%" title="News: Rust 1.29 and 1.30" controls preload=metadata src="https://www.podtrac.com/pts/redirect.mp3/f001.backblazeb2.com/file/newrustacean/news/rust_1_29_1_30.mp3">
//!
//! Show Notes
//! ----------
//!
//! Rust 1.29:
//!
//! - [blog post][1.29-blog]
//! - [release notes][1.29-release]
//!
//! [1.29-blog]: https://blog.rust-lang.org/2018/09/13/Rust-1.29.html
//! [1.29-release]: https://github.com/rust-lang/rust/blob/master/RELEASES.md#version-1290-2018-09-13
//!
//! Rust 1.30:
//!
//! - [blog post][1.30-blog]
//! - [release notes][1.30-release]
//! - [RFC: path clarity][rfc]
//!
//! [1.30-blog]: https://blog.rust-lang.org/2018/10/25/Rust-1.30.0.html
//! [1.30-release]: https://github.com/rust-lang/rust/blob/master/RELEASES.md#version-1300-2018-10-25
//! [rfc]: https://github.com/rust-lang/rfcs/blob/master/text/2126-path-clarity.md
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
//! - Alexander Payne
//! - Alexander Kryvomaz
//! - Andrew Thompson
//! - [Anthony Deschamps]
//! - Anthony Scotti
//! - [Behnam Esfahbod]
//! - Benjamin Wasty
//! - Brandon 'Spanky' Mills
//! - Brian Casiello
//! - Brian Manning
//! - [Bryan Stitt]
//! - Bryce Johnston
//! - Caryn Finkelman
//! - Cass Costello
//! - Chap Lovejoy
//! - [Charlie Egan]
//! - Chip
//! - [Chris Palmer]
//! - Daniel
//! - Dan Abrams
//! - Daniel Bross
//! - [Daniel Collin]
//! - [Daniel Mason]
//! - David Hewson
//! - [Derek Morr]
//! - Doug Reeves
//! - Eugene Bulkin
//! - Fábio Botelho
//! - [Gaveen Prabhasara]
//! - [Graham Wihlidal]
//! - [Henri Sivonen]
//! - [Ian Jones]
//! - "Jake ""ferris"" Taylor"
//! - Jako Danar
//! - James Cooper
//! - James Hagans II
//! - Jerome Froelich
//! - [Joar Wandborg]: Joar Wandborg
//! - [John Rudnick]
//! - Jon
//! - [Jonathan Knapp]
//! - Jonathan Turner
//! - Joseph Hain
//! - Joseph Marhee
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
//! - Peter Tillemans
//! - Ralph Giles
//! - [Ramon Buckland]
//! - Randy MacLeod
//! - Raph Levien
//! - Richard Dallaway
//! - Rob Tsuk
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
//! [Anthony Deschamps]: https://github.com/adeschamps
//! [Behnam Esfahbod]: https://github.com/behnam
//! [Bryan Stitt]: http://www.stitthappens.com/
//! [Charlie Egan]: https://charlieegan3.com
//! [Chris Palmer]: http://red-oxide.org/
//! [Damien Stanton]: https://github.com/damienstanton
//! [Daniel Collin]: https://twitter.com/daniel_collin
//! [Daniel Mason]: https://github.com/gisleburt
//! [Daniel P. Clark]: https://6ftdan.com/
//! [David W. Allen]: http://GitHub.com/DataRiot
//! [Derek Morr]: https://twitter.com/derekmorr
//! [Gaveen Prabhasara]: https://twitter.com/gaveen
//! [Graham Wihlidal]: https://wihlidal.com/
//! [Henri Sivonen]: https://hsivonen.fi/
//! [Ian Jones]: https://www.ianmjones.com/
//! [Joar Wandborg]: http://github.com/joar
//! [Jonathan Knapp]: https://www.coffeeandcode.com/
//! [John Rudnick]: http://www.cindur.com/
//! [Luiz Irber]: http://luizirber.org/
//! [Martin Heuschober]: https://github.com/epsilonhalbe
//! [Max Jacobson]: https://twitter.com/maxjacobson
//! [Messense Lv]: https://github.com/messense
//! [Michael Mc Donnell]: https://www.linkedin.com/in/michaelmcdonnell/
//! [Nathan Sculli]: http://influential.co/
//! [Nick Coish]: http://github.com/ncoish
//! [Nick Stevens]: https://github.com/nastevens
//! [Nicolas Pochet]: https://github.com/n-pochet
//! [Oluseyi Sonaiya]: http://oluseyi.info/
//! [Pascal]: https://pascalhertleif.de/
//! [Patrick O'Doherty]: https://twitter.com/patrickod
//! [Philipp Keller]: https://twitter.com/hansapla
//! [Ramon Buckland]: http://www.inosion.com
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
//!       + Twitter: [@newrustacean](https://www.twitter.com/newrustacean)
//!       + Email: [hello@newrustacean.com](mailto:hello@newrustacean.com)
//! - Chris Krycho
//!       + GitHub: [chriskrycho](https://github.com/chriskrycho)
//!       + Twitter: [@chriskrycho](https://www.twitter.com/chriskrycho)

#[doc(include = "../docs/news/rust_1_29_1_30.md")]
pub struct Script;
