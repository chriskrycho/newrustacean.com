//! Can You See Me Now?
//!
//! - **Date:** April 26, 2019
//! - **Subject:** Item visibility and `pub(<restricted>)` as API design tools.
//! - [**download mp3**][mp3]
//! - [**script**][script]
//!
//! [mp3]: https://www.podtrac.com/pts/redirect.mp3/f001.backblazeb2.com/file/newrustacean/e030.mp3
//! [script]: https://newrustacean.com/show_notes/e030/struct.script
//!
//! <audio style="width: 100%" title="e030: Can You See Me Now?" controls preload=metadata src="https://www.podtrac.com/pts/redirect.mp3/f001.backblazeb2.com/file/newrustacean/e030.mp3">
//!
//! Show Notes
//! ----------
//!
//! The easiest and most effective way to understand the example in this case
//! will simply be to look directly at [the source code][src]. You *can* read
//! the docs for each of the nested modules, but you'll be doing a *lot* of
//! navigating around for that.
//!
//! [src]: /src/show_notes/e030.rs.html
//!
//! Also, I am using Cargo’s `--document-private-items` flag, so that you can
//! see *all* the items in *all* the modules, even those which are not public,
//! but note that usually you would not see docs for those!
//!
//! ### Links
//!
//! - [the reference on visibility]
//! - [RFC #1422]
//! - [episode source code]
//! - [e020]
//!
//! [the reference on visibility]: https://doc.rust-lang.org/reference/visibility-and-privacy.html
//! [RFC #1422]: https://github.com/rust-lang/rfcs/blob/master/text/1422-pub-restricted.md
//! [episode source code]: https://newrustacean.com/src/show_notes/e030.rs.html
//! [e020]: https://newrustacean.com/show_notes/e020/
//!
//! Sponsors
//! --------
//!
//! Thanks to Manning for sponsoring the show *and* giving all of you a 40%-off
//! discount on their whole store (but especially their [<cite>WebAssembly in
//! Action</cite> MEAP][MEAP]) at [deals.manning.com/new-rustacean][manning]!
//!
//! [manning]: http://bit.ly/2OXnlEb
//! [MEAP]: https://www.manning.com/books/webassembly-in-action
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
//! - Scott Moeller
//! - [Sebastián Ramírez Magrí]
//! - [Simon Dickson]
//! - Simon G
//! - [Soren Bramer Schmidt]
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
//! - [Patreon](https://www.patreon.com/newrustacean)
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

#[doc(include = "../docs/e030-script.md")]
pub struct Script;

/// One of two internal modules (with `right`) for demonstrating visibility.
pub mod left {
    /// Visible to other items in `left`, including in sub-modules.
    fn local() {}

    /// `pub(crate) mod a` is visible everywhere within the crate that can see
    /// `left`, but hidden from all external callers.
    pub(crate) mod a {
        /// This is visible everywhere in the crate that can see `a`. Open the
        /// source to see all the other items it can interact with!
        pub fn demo() {
            // We can see other items in `left` or `a` regardless of visibility.
            super::local();
            sibling();

            // We can see other items which are marked as visible within `left`.
            crate::e030::left::b::demo();

            // We can see other items which are marked as visible within `e030`,
            // whether direct siblings to us or not.
            crate::e030::left::c::demo();
            super::c::demo();
            crate::e030::right::s::demo();

            // We can see other items which are marked as visible within the
            // crate.
            crate::e030::right::q::demo();

            // This won't compile because `crate::e030::right::r` is not visible
            // here: it is only visible within `crate::e030::right`.
            // crate::e030::right::r::demo();

            // We also cannot see items in `child` unless they are some `pub`
            // variant.
            // super::private_child::only_in_child();

            // But we can
            // super::private_child::also_only_in_child();
        }

        /// `fn sibling` is visible only to other items within `a`.
        fn sibling() {}

        /// `mod child` here is visible to `a`, but not outside `a` -- including
        /// even is siblings.
        mod private_child {
            /// `fn only_in_child` is not visible to anything outside `child`.
            fn only_in_child() {}

            /// `pub fn also_only_in_child` is also hidden to anything outside
            /// `child`.
            pub fn also_only_in_child() {}
        }
    }

    /// `pub(in crate::e030::left) mod b` is visible to
    pub(in crate::e030::left) mod b {

        /// This is visible everywhere in the crate that can see `b`. Open
        /// the source to see how all the other items it can interact with!
        pub fn demo() {}
    }

    pub(in crate::e030) mod c {
        /// This is visible everywhere in the crate that can see `b`. Open the
        /// source to see how all the other items it can interact with!
        pub fn demo() {}
    }
}

/// One of two internal modules (with `left`) for demonstrating visibility.
pub mod right {
    /// This is visible *within* the crate everywhere that can see `right`.
    pub(crate) mod q {
        /// This is visible everywhere in the crate that can see `q`. Visibility
        /// exactly mirrors `crate::e030::left::a::demo`.
        pub fn demo() {}
    }

    /// This is visible everywhere in `right`, including in `q`.
    pub(in crate::e030::right) mod r {
        /// This is visible everywhere in the crate that can see `s`. Open
        /// the source to see how all the other items it can interact with!
        pub fn demo() {}
    }

    /// This is visible everywhere in the e030 module.
    pub(in crate::e030) mod s {
        pub fn demo() {}
    }
}
