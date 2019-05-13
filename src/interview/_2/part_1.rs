//! Raph Levien on using Rust to build the Xi editor
//!
//!   - **Date:** June 24, 2016
//!   - **Subject:** The Xi Editor project as a window into complex data
//!     structures, memory management, and the Unix philosophy.
//!   - [**Audio**][mp3]
//!
//! [mp3]: https://www.podtrac.com/pts/redirect.mp3/f001.backblazeb2.com/file/newrustacean/interview/2/part_1.mp3
//!
//! <audio style="width: 100%" title="Interview 2::Part 1 – Raph Levien" controls preload=metadata>
//!   <source src="https://www.podtrac.com/pts/redirect.mp3/f001.backblazeb2.com/file/newrustacean/interview/2/part_1.mp3">
//! </audio>
//!
//!
//! Notes
//! -----
//!
//! Chris chats with Raph Levien about his background in software development,
//! what attracted him to Rust, and how he's using Rust to build the Xi Editor,
//! a project which aims to be the fastest text editor out there, with native
//! user interfaces and a Rust text engine.
//!
//!
//! Links
//! -----
//!
//!   - Past work:
//!
//!       + [GIMP]
//!       + [GTK]
//!       + [GhostScript]
//!       + [Google Web Fonts]
//!       + [font-rs]
//!
//!   - Current projects:
//!
//!       + [pulldown-cmark]
//!       + [Xi Editor]
//!
//!   - [fuzz testing]
//!
//!   - [sanitizers]
//!
//!   - [FreeType]
//!
//!   - [HarfBuzz]
//!
//!   - [ICU]
//!
//!   - Ropes
//!
//!       + [Wikipedia summary]
//!       + [original paper]
//!   - ["log n operation"], or *O(log n)*
//!
//!       + [Big O notation]
//!   - Rust:
//!
//!       + [`Arc`]
//!       + [`RefCell`]
//!       + [`Borrow`]
//!       + [`AsRef`]
//!   - Rust libraries
//!
//!       + Crossbeam: [source][Crossbeam source] | [docs][Crossbeam docs]
//!       + Rayon: [source][Rayon source] | [docs][Rayon docs]
//!
//!   - [Unix philosophy]
//!
//!     > The Unix philosophy emphasizes building simple, short, clear,
//!     > modular, and extensible code that can be easily maintained and
//!     > repurposed by developers other than its creators. The Unix philosophy
//!     > favors composability as opposed to monolithic design.
//!
//! [GIMP]: http://www.gimp.org
//! [GTK]: http://gtk.org
//! [GhostScript]: http://www.ghostscript.com
//! [Google Web Fonts]: https://fonts.google.com
//! [font-rs]: https://github.com/google/font-rs
//! [pulldown-cmark]: https://github.com/google/pulldown-cmark
//! [Xi Editor]: https://github.com/google/xi-editor
//! [fuzz testing]: https://en.wikipedia.org/wiki/Fuzz_testing
//! [sanitizers]: http://developers.redhat.com/blog/2014/12/02/address-and-thread-sanitizers-gcc/
//! [FreeType]: https://www.freetype.org
//! [HarfBuzz]: https://www.freedesktop.org/wiki/Software/HarfBuzz/
//! [ICU]: http://site.icu-project.org
//! [Wikipedia summary]: https://en.wikipedia.org/wiki/Rope_%28data_structure%29
//! [original paper]: http://citeseer.ist.psu.edu/viewdoc/download?doi=10.1.1.14.9450&rep=rep1&type=pdf
//! ["log n operation"]: http://stackoverflow.com/questions/2307283/what-does-olog-n-mean-exactly
//! [Big O notation]: https://en.wikipedia.org/wiki/Big_O_notation
//! [`Arc`]: http://doc.rust-lang.org/std/sync/struct.Arc.html
//! [`RefCell`]: http://doc.rust-lang.org/std/cell/struct.RefCell.html
//! [`Borrow`]: http://doc.rust-lang.org/std/borrow/trait.Borrow.html
//! [`AsRef`]: http://doc.rust-lang.org/std/convert/trait.AsRef.html
//! [Crossbeam source]: https://github.com/aturon/crossbeam
//! [Crossbeam docs]: http://aturon.github.io/crossbeam-doc/crossbeam/
//! [Rayon source]: https://github.com/nikomatsakis/rayon
//! [Rayon docs]: http://nikomatsakis.github.io/rayon/rayon/
//! [Unix philosophy]: https://en.wikipedia.org/wiki/Unix_philosophy
//!
//!
//! Sponsors
//! --------
//!
//!   - Aleksey Pirogov
//!   - [Chris Palmer]
//!   - [Daniel Collin]
//!   - [Derek Morr]
//!   - Doug Reeves
//!   - Hamza Sheikh
//!   - Lachlan Collins
//!   - Leif Arne Storset
//!   - Luca Schmid
//!   - Micael Bergeron
//!   - [Pascal Hertleif]
//!   - Ralph Giles ("rillian")
//!   - Ralph "FriarTech" Loizzo
//!   - Raph Levien
//!   - reddraggone9
//!   - Ryan Ollos
//!   - Vesa Kaihlavirta
//!   - [William Roe]
//!
//! [Chris Palmer]: http://red-oxide.org/
//! [Daniel Collin]: twitter.com/daniel_collin
//! [Derek Morr]: https://twitter.com/derekmorr
//! [Pascal Hertleif]: https://pascalhertleif.de/
//! [William Roe]: http://willroe.me
//!
//! (Thanks to the couple people donating who opted out of the reward tier, as
//! well. You know who you are!)
//!
//! ### Become a sponsor
//!
//!   - <a href="https://www.patreon.com/newrustacean" rel="payment">Patreon</a>
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
//!     + Twitter: [@newrustacean](https://www.twitter.com/newrustacean)
//!     + Email: [hello@newrustacean.com](mailto:hello@newrustacean.com)
//!   - Chris Krycho
//!     + GitHub: [chriskrycho](https://github.com/chriskrycho)
//!     + Twitter: [@chriskrycho](https://www.twitter.com/chriskrycho)
