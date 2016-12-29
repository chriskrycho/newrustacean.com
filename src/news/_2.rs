//! Let's talk roadmap!
//!
//!   - **Date:** December 29, 2016
//!   - **Subject:** Rust's achievements in 2016 and goals for 2017
//!   - **Audio:**
//!       + [M4A](http://www.podtrac.com/pts/redirect.m4a/cdn.newrustacean.com/news_2.m4a)
//!       + [MP3](http://www.podtrac.com/pts/redirect.mp3/cdn.newrustacean.com/news_2.mp3)
//!       + [Ogg](http://www.podtrac.com/pts/redirect.ogg/cdn.newrustacean.com/news_2.ogg)
//!
//! <audio title="Rust's achievements in 2016 and goals for 2017" controls preload=metadata>
//!   <source src="http://www.podtrac.com/pts/redirect.m4a/cdn.newrustacean.com/news_2.m4a">
//!   <source src="http://www.podtrac.com/pts/redirect.mp3/cdn.newrustacean.com/news_2.mp3">
//!   <source src="http://www.podtrac.com/pts/redirect.ogg/cdn.newrustacean.com/news_2.ogg">
//! </audio>
//!
//! Links
//! -----
//!
//!   - Rust releases:
//!       + 1.10: [blog post][1.10-blog] | [release notes][1.10-rn]
//!       + 1.11: [blog post][1.11-blog] | [release notes][1.11-rn]
//!       + 1.12: [blog post][1.12-blog] | [release notes][1.12-rn]
//!       + 1.12.1: [blog post][1.12.1-blog]
//!       + 1.13: [blog post][1.13-blog] | [release notes][1.13-rn]
//!       + 1.14: [blog post][1.14-blog] | [release notes][1.14-rn]
//!   - Rust 2017 roadmap
//!       + [RFC text]
//!       + [RFC discussion]
//!   - Other official Rust blog posts:
//!       + [Shape of errors to come]
//!       + [Incremental compilation]
//!           * Milestone: [beta]
//!           * Milestone: [across crates]
//!           * Milestone: [in typechecking]
//!   - Cargo workspaces:
//!       + [Original RFC] and [discussion]
//!       + [Documentation]
//!   - Rust Language Service:
//!       + [Announcement post] on internals.rust-lang.org
//!       + [Demo]
//!   - Non-core projects mentioned on the show:
//!       + [Futures]
//!       + [Tokio]
//!       + [Rocket]
//!   - My projects
//!       + [Lightning]
//!       + RFC #1636:
//!           * [text][1636-text]
//!           * [discussion][1636-discussion]
//!           * [tracking issue][1636-tracking] (where you can contribute!)
//!
//! [1.10-blog]: https://blog.rust-lang.org/2016/07/07/Rust-1.10.html
//! [1.10-rn]: https://github.com/rust-lang/rust/blob/master/RELEASES.md#version-1100-2016-07-07
//! [1.11-blog]: https://blog.rust-lang.org/2016/08/18/Rust-1.11.html
//! [1.11-rn]: https://github.com/rust-lang/rust/blob/master/RELEASES.md#version-1110-2016-08-18
//! [1.12-blog]: https://blog.rust-lang.org/2016/09/29/Rust-1.12.html
//! [1.12-rn]: https://github.com/rust-lang/rust/blob/master/RELEASES.md#version-1120-2016-09-29
//! [1.12.1-blog]: https://blog.rust-lang.org/2016/10/20/Rust-1.12.1.html
//! [1.13-blog]: https://blog.rust-lang.org/2016/11/10/Rust-1.13.html
//! [1.13-rn]: https://github.com/rust-lang/rust/blob/stable/RELEASES.md#version-1130-2016-11-10
//! [1.14-blog]: https://blog.rust-lang.org/2016/12/22/Rust-1.14.html
//! [1.14-rn]: https://github.com/rust-lang/rust/blob/master/RELEASES.md#version-1140-2016-12-22
//! [RFC text]: https://github.com/aturon/rfcs/blob/4f40ba07f2a0730c188cb5db6b0b9c5887ae1801/text/0000-roadmap-2017.md
//! [RFC discussion]: https://github.com/rust-lang/rfcs/pull/1774
//! [Shape of errors to come]: https://blog.rust-lang.org/2016/08/10/Shape-of-errors-to-come.html
//! [Incremental compilation]: https://blog.rust-lang.org/2016/09/08/incremental.html
//! [beta]: https://github.com/rust-lang/rust/milestone/30
//! [across crates]: https://github.com/rust-lang/rust/milestone/31
//! [in typechecking]: https://github.com/rust-lang/rust/milestone/32
//! [Original RFC]: https://github.com/rust-lang/rfcs/blob/master/text/1525-cargo-workspace.md
//! [discussion]: https://github.com/rust-lang/rfcs/pull/1525
//! [Documentation]: http://doc.crates.io/manifest.html#the-workspace-section
//! [Announcement post]: https://internals.rust-lang.org/t/introducing-rust-language-server-source-release/4209
//! [Demo]: https://www.youtube.com/watch?time_continue=2405&v=pTQxHIzGqFI
//! [Futures]: https://github.com/alexcrichton/futures-rs
//! [Tokio]: https://github.com/tokio-rs/tokio
//! [Rocket]: https://rocket.rs
//! [Lightning]: https://github.com/chriskrycho/lightning-rs
//! [1636-text]: https://github.com/rust-lang/rfcs/blob/master/text/1636-document_all_features.md
//! [1636-discussion]: https://github.com/rust-lang/rfcs/pull/1636
//! [1636-tracking]: https://github.com/rust-lang/rust/issues/38643
//! 
//!
//! Sponsors
//! --------
//!
//!   - Aleksey Pirogov
//!   - Andreas Fischer
//!   - Ben Whitley
//!   - Cameron Mochrie
//!   - [Chris Palmer]
//!   - [Daniel Collin]
//!   - [Derek Morr]
//!   - [Jakub "Limeth" Hlusička]
//!   - [Jupp Müller]
//!   - Keith Gray
//!   - Lachlan Collins
//!   - Luca Schmid
//!   - Matt Rudder
//!   - Matthew Piziak
//!   - Micael Bergeron
//!   - Ovidiu Curcan
//!   - [Pascal Hertleif]
//!   - Peter Tillemans
//!   - Ralph Giles ("rillian")
//!   - Raph Levien
//!   - reddraggone9
//!   - Ryan Ollos
//!   - Steven Murawksi
//!   - Vesa Kaihlavirta
//!   - Vlad Bezden
//!   - [William Roe]
//!   - Zaki
//!
//! [Chris Palmer]: http://home.red-oxide.org/
//! [Daniel Collin]: https://twitter.com/daniel_collin
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
