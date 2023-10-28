//! Meet My Associates
//!
//! - **Date:** February 21, 2019
//! - **Subject:** Associated items: functions and methods, constants, types,
//!   and (very briefly!) GATs.
//! - [**download mp3**][mp3]
//! - [**script**][script]
//!
//! [mp3]: https://www.podtrac.com/pts/redirect.mp3/cdn.newrustacean.com/file/newrustacean/e028.mp3
//! [script]: https://newrustacean.com/show_notes/e027/struct.script
//!
//! <audio style="width: 100%" title="e028: Meet My Associates" controls preload=metadata src="https://www.podtrac.com/pts/redirect.mp3/cdn.newrustacean.com/file/newrustacean/e028.mp3">
//!
//! Show Notes
//! ----------
//!
//! - [`Iterator` source][the source]
//! - [the first Traits deep dive episode]
//! - [RFC #1598]
//! - [RFC #195]
//! - [The Reference on associated items][reference]
//! - [Rust 1.20]
//! - [deals.manning.com/new-rustacean]
//!
//! [the source]: https://doc.rust-lang.org/1.32.0/src/core/iter/iterator.rs.html&version=1.32.0
//! [the first Traits deep dive episode]: https://newrustacean.com/show_notes/e023/
//! [RFC #1598]: https://github.com/rust-lang/rfcs/blob/master/text/1598-generic_associated_types.md
//! [RFC #195]: https://github.com/rust-lang/rfcs/blob/master/text/0195-associated-items.md
//! [reference]: https://doc.rust-lang.org/reference/glossary.html#associated-item
//! [Rust 1.20]: https://newrustacean.com/show_notes/news/rust_1_20/
//! [deals.manning.com/new-rustacean]: http://bit.ly/2OXnlEb
//!
//! Sponsors
//! --------
//!
//! Thanks to Manning for sponsoring the show *and* giving all of you a 40%-off
//! discount on their whole store (but especially Carol Nichols' and Jake
//! Goulding's _Rust in Motion_ video content and the _Rust in Action_ MEAP!) at
//! [deals.manning.com/new-rustacean][manning]
//!
//! [manning]: http://bit.ly/2OXnlEb
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
//! - [beaorn]
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
//! - Dan Abrams
//! - Daniel
//! - Daniel Bross
//! - [Daniel Collin]
//! - [Daniel Mason]
//! - David Carroll
//! - David Hewson
//! - [Derek Morr]
//! - Doug Reeves
//! - [Douglas Correa]
//! - [Eduard Knyshov]
//! - [Embark Studios]
//! - Eugene Bulkin
//! - Evan Stoll
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
//! - [Jason Bowen]
//! - [Jendrik Illner]
//! - Jerome Froelich
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
//! - [Soren Bramer Schmidt]
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
//! [Arlo (Hyena)]: https://asonix.dog/@asonix
//! [beaorn]: https://github.com/beaorn
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
//! [Graham Wihlidal]: https://wihlidal.com/
//! [Henri Sivonen]: https://hsivonen.fi/
//! [Ian Jones]: https://www.ianmjones.com/
//! [Jason Bowen]: https://twitter.com/jwbowen
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

#[doc = include_str!("../docs/e028-script.md")]
pub struct Script;
