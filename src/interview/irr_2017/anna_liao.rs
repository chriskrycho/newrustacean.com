//! Anna Liao
//!
//!   - **Date:** December 20, 2017
//!   - **Subject:** Anna's experience learning Rust while porting a Raspberry Pi
//!     Python project as part of the Increasing Rust’s Reach 2017 program.
//!   - **Audio:**
//!       + [M4A][m4a]
//!       + [MP3][mp3]
//!       + [Ogg][ogg]
//!
//! [m4a]: http://www.podtrac.com/pts/redirect.m4a/cdn.newrustacean.com/anna_liao.m4a
//! [mp3]: http://www.podtrac.com/pts/redirect.mp3/cdn.newrustacean.com/anna_liao.mp3
//! [ogg]: http://www.podtrac.com/pts/redirect.ogg/cdn.newrustacean.com/anna_liao.ogg
//!
//! <audio style="width: 100%" style="width: 100%" title="Jonathan Turner::Part 3" controls preload=metadata>
//!   <source src="http://www.podtrac.com/pts/redirect.m4a/cdn.newrustacean.com/anna_liao.m4a">
//!   <source src="http://www.podtrac.com/pts/redirect.mp3/cdn.newrustacean.com/anna_liao.mp3">
//!   <source src="http://www.podtrac.com/pts/redirect.ogg/cdn.newrustacean.com/anna_liao.ogg">
//! </audio>
//!
//!
//! Show Notes
//! ----------
//! 
//! - [Increasing Rust's Reach]
//! - [Go]
//! - [This Week In Rust]
//! - [PyCon]
//! - [Raspberry Pi]
//! - [Sense HAT]
//! - [PyCascades]
//! - [Andrew Gallant (`burntsushi`)]
//!     + [ripgrep]
//! - [RustBridge]
//! - [exercism.io]
//! - [the other Sense HAT Rust conversion]
//! 
//! [Increasing Rust's Reach]: https://blog.rust-lang.org/2017/06/27/Increasing-Rusts-Reach.html
//! [Go]: https://golang.org
//! [This Week In Rust]: https://this-week-in-rust.org
//! [PyCon]: http://www.pycon.org
//! [Raspberry Pi]: https://www.raspberrypi.org
//! [Sense HAT]: https://www.raspberrypi.org/products/sense-hat/
//! [PyCascades]: https://www.pycascades.com
//! [Andrew Gallant (`burntsushi`)]: https://github.com/BurntSushi
//! [ripgrep]: https://github.com/BurntSushi/ripgrep
//! [RustBridge]: https://github.com/rust-community/rustbridge
//! [exercism.io]: https://exercism.io
//! [the other Sense HAT Rust conversion]: https://github.com/thejpster/pi-workshop-rs
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
//!   - Coleman McFarland
//!   - Dan Abrams
//!   - [Daniel Collin]
//!   - Daniel P. Clark
//!   - [David W. Allen]
//!   - David Hewson
//!   - [Derek Morr]
//!   - Eugene Bulkin
//!   - [Henri Sivonen]
//!   - [Ian Jones]
//!   - [Jakub "Limeth" Hlusička]
//!   - James Cooper
//!   - Jerome Froelich
//!   - John Rudnick
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
//!   - Peter Tillemans
//!   - Ralph Giles ("rillian")
//!   - Randy MacLeod
//!   - Raph Levien
//!   - reddraggone9
//!   - [Ryan Blecher]
//!   - [Sebastián Ramírez Magrí]
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
//! [Daniel Collin]: https://twitter.com/daniel_collin
//! [David W. Allen]: http://GitHub.com/DataRiot
//! [Derek Morr]: https://twitter.com/derekmorr
//! [Henri Sivonen]: https://hsivonen.fi/
//! [Ian Jones]: https://www.ianmjones.com/
//! [Jakub "Limeth" Hlusička]: https://github.com/Limeth
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

/// Transcript coming soon!
pub struct Transcript;