//! Trust Me; I Promise!
//!
//! - **Date:** November 30, 2018
//! - **Subject:** An intro to `unsafe` Rust and Rust's idea of safety.
//! - [**download mp3**][mp3]
//! - [**script**][script]
//!
//! [mp3]: https://www.podtrac.com/pts/redirect.mp3/cdn.newrustacean.com/file/newrustacean/e027.mp3
//! [script]: https://newrustacean.com/show_notes/e027/struct.script
//!
//! <audio style="width: 100%" title="News: Rust 1.29 and 1.30" controls preload=metadata src="https://www.podtrac.com/pts/redirect.mp3/cdn.newrustacean.com/file/newrustacean/e027.mp3">
//!
//! Show Notes
//! ----------
//!
//! ### Errata
//!
//! A quick correction: on the show I said that a trait needed to be unsafe when
//! it had an `unsafe fn` method. This isn't correct: safe traits can have
//! unsafe methods, and unsafe traits can exist without any methods at all (as
//! implied by my reference to `Send` and `Sync`). You can see this in practice
//! in the following example, which compiles just fine!
//!
//! ```
//! trait ASafeTrait {
//!     unsafe fn unsafe_method() {}
//! }
//!
//! unsafe trait AnUnsafeTrait {}
//! ```
//!
//! The idea of an `unsafe` trait is that it has some conditions which you must
//! uphold to safely implement it – again, just as with `Send` and `Sync`. In
//! the case of most traits, this will be because some trait method has
//! invariants it needs to hold else it would cause undefined behavior. For
//! another example of this, see the (unstable as of the time of recording)
//! trait [`std::iter::TrustedLen`].
//!
//! [`std::iter::TrustedLen`]: https://doc.rust-lang.org/nightly/std/iter/trait.TrustedLen.html
//!
//! Thanks to Rust language team member [@centril] for noting this to me after
//! listening when I was recording the show live!
//!
//! [@centril]: https://github.com/centril
//!
//! ### Links
//!
//! - [_The Rust Programming Language_, Chapter 19: Unsafe][book]
//! - [The Nomicon][nomicon]
//! - ["Rust and OpenGL from Scratch", by Nerijus Arlauskas][opengl]
//!
//! [book]: https://doc.rust-lang.org/book/2018-edition/ch19-01-unsafe-rust.html
//! [nomicon]: https://doc.rust-lang.org/nomicon/README.html
//! [opengl]: http://nercury.github.io/rust/opengl/tutorial/2018/02/08/opengl-in-rust-from-scratch-00-setup.html
//!
//!
//! ### Examples
//!
//! #### Borrow-checked code in `unsafe`
//!
//! ```
//! let mut f = String::from("foo");
//!
//! unsafe {
//!     let borrowed = &mut f;
//!
//!     // This would be unsafe and throw an error (before Rust 2018):
//!     // let borrow_again = &f;
//!
//!     println!("{}", borrowed);
//!
//!     // This would be unsafe and throw an error:
//!     // println!("{}", borrow_again);
//! }
//! ```
//!
//! [(See it in a playground)][safe `unsafe`]
//!
//! [safe `unsafe`]: https://play.rust-lang.org/?version=beta&mode=release&edition=2018&gist=38d1089cdc3a4148609e9e3bbbfd002c
//!
//! ##### Safely mutating a raw pointer
//!
//! ```
//! let f = Box::new(12);
//! let mut g = Box::into_raw(f);
//! g = &mut 10;
//! ```
//!
//! [(See it in a playground)][safe mutate]
//!
//! [safe mutate]: https://play.rust-lang.org/?version=stable&mode=debug&edition=2015&gist=3a7a9facd0f67d4a590afc3a3ecef95b
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
//! - [Brian McCallister]
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
//! - Peter Scholtens
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
//!     + Twitter: [@newrustacean](https://www.twitter.com/newrustacean)
//!     + Email: [hello@newrustacean.com](mailto:hello@newrustacean.com)
//! - Chris Krycho
//!     + GitHub: [chriskrycho](https://github.com/chriskrycho)
//!     + Twitter: [@chriskrycho](https://www.twitter.com/chriskrycho)

#[doc(include = "../docs/e027-script.md")]
pub struct Script;
