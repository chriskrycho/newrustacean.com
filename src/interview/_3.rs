//! Carol (Nichols || Goulding)
//!
//!   - **Date:** September 30, 2016
//!   - **Subject:** Learning Rust, teaching Rust, and building community
//!   - [**Audio**][mp3]
//!
//! [mp3]: https://www.podtrac.com/pts/redirect.mp3/f001.backblazeb2.com/file/newrustacean/interview/3.mp3
//!
//! <audio style="width: 100%" title="Interview 3 – Carol (Nichols || Goulding)" controls preload=metadata>
//!   <source src="https://www.podtrac.com/pts/redirect.mp3/f001.backblazeb2.com/file/newrustacean/interview/3.mp3">
//! </audio>
//!
//!
//! Notes
//! -----
//!
//! Chris talks with Carol (Nichols || Goulding), a Rust community team member, co-author of
//! the first major revision of _The Rust Programming Language_, and co-founder
//! of the first Rust consultancy.
//!
//! Links
//! -----
//!
//! - [<abbr>XSLT</abbr> (XML Style Sheet Transformations)][XSLT]
//! - [Rspec]
//! - [Rails]
//! - [Think Through Math] — remedial math tutoring app built with Rails
//! - [Rust for Rubyists]
//! - [_The Rust Programming Language_][TRPL]
//! - [Julia Evans]
//!     + [RustConf 2016 keynote][keynote]
//! - [_Rust by Example_][RBE]
//! - [Rustlings]
//! - Sass
//!     + [language][sass-lang]
//!     + [Carol's in-progress implementation][sass-impl]
//! - [#rust-community][IRC] — open meetings at 4pm UTC every Wednesday, with
//!   [minutes available online][minutes]
//! - [first Rust community survey][survey]
//! - [Rust community on GitHub][community]
//! - [new version of the book on GitHub][TRPL update] — *you* can help, and
//!   especially if you're new, because Steve and Carol both need input to deal
//!   with the "familiarity"/["curse of knowledge"] problem
//!     - ownership and borrowing chapters
//! - [RustConf]
//! - [RustFest]
//! - [Rust Belt Rust] — October 27–28, 2016. _Don't forget to use code
//!   **newrustacean** to get 20% off of your registration cost!_
//! - [Integer32]
//!     - [Panoptix] – nickel-jwt-session: [crate] | [docs] | [source]
//! - [Jake Goulding on Stack Overflow][so]
//! - [Friends of the Tree]
//! - [Friends of the Forest]
//! - [_Working Effectively With Legacy Code_][WEWLC]
//! - Tilde
//! - [_The C Programming Language_][TCPL] -- the book Carol compared _The Rust
//!   Programming Language_ to in terms of its responsibilities, and also one of
//!   the books from which Chris learned C.
//!
//! [XSLT]: https://developer.mozilla.org/en-US/docs/Web/XSLT
//! [Rspec]: http://rspec.info
//! [Rails]: http://rubyonrails.org
//! [Think Through Math]: https://www.thinkthroughmath.com
//! [Rust for Rubyists]: https://github.com/steveklabnik/rust_for_rubyists
//! [TRPL]: https://doc.rust-lang.org/book/
//! [Julia Evans]: http://jvns.ca
//! [keynote]: http://jvns.ca/blog/2016/09/11/rustconf-keynote/
//! [RBE]: http://rustbyexample.com
//! [Rustlings]: https://github.com/carols10cents/rustlings
//! [sass-lang]: http://sass-lang.com
//! [sass-impl]: https://github.com/carols10cents/sassers
//! [IRC]: https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-community
//! [minutes]: https://github.com/rust-community/team/tree/master/meeting-minutes
//! [survey]: https://blog.rust-lang.org/2016/06/30/State-of-Rust-Survey-2016.html
//! [community]: https://github.com/rust-community
//! [TRPL update]: http://rust-lang.github.io/book/
//! ["curse of knowledge"]: https://en.wikipedia.org/wiki/Curse_of_knowledge
//! [RustConf]: http://rustconf.com
//! [RustFest]: http://www.rustfest.eu
//! [Rust Belt Rust]: http://www.rust-belt-rust.com
//! [Integer32]: http://www.integer32.com
//! [Panoptix]: http://www.panoptix.co.za/
//! [crate]: https://crates.io/crates/nickel-jwt-session
//! [docs]: https://docs.rs/nickel-jwt-session/
//! [source]: https://github.com/kaj/nickel-jwt-session
//! [so]: http://stackoverflow.com/cv/jakegoulding
//! [Friends of the Tree]: https://github.com/rust-lang/rust-wiki-backup/blob/master/Doc-friends-of-the-tree.md
//! [Friends of the Forest]: https://github.com/rust-community/team/issues/51
//! [WEWLC]: http://www.alibris.com/booksearch?keyword=working%20effectively%20with%20legacy%20code
//! [TCPL]: http://www.alibris.com/The-C-Programming-Language-Brian-Kernighan/book/875968
//!
//! Sponsors
//! --------
//!
//!   - Aleksey Pirogov
//!   - Cameron Mochrie
//!   - [Chris Palmer]
//!   - [Daniel Collin]
//!   - [Derek Morr]
//!   - Doug Reeves
//!   - Hamza Sheikh
//!   - [Jakub "Limeth" Hlusička]
//!   - [Jupp Müller]
//!   - Keith Gray
//!   - Lachlan Collins
//!   - Leif Arne Storset
//!   - Luca Schmid
//!   - Matthew Piziak
//!   - Micael Bergeron
//!   - Nils Tekampe
//!   - Ovidiu Curcan
//!   - [Pascal Hertleif]
//!   - Ralph Giles ("rillian")
//!   - Ralph "FriarTech" Loizzo
//!   - Raph Levien
//!   - reddraggone9
//!   - Ryan Ollos
//!   - Sean Jensen-Gray
//!   - Steven Murawski
//!   - Vesa Kaihlavirta
//!   - [William Roe]
//!
//! [Chris Palmer]: http://red-oxide.org/
//! [Daniel Collin]: twitter.com/daniel_collin
//! [Derek Morr]: https://twitter.com/derekmorr
//! [Jakub "Limeth" Hlusička]: https://github.com/Limeth
//! [Jupp Müller]: https://de.linkedin.com/in/juppm
//! [Pascal Hertleif]: https://pascalhertleif.de/
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
