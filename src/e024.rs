//! Traits Deep Dive, Part 2
//!
//!   - **Date:** June 19, 2018
//!   - **Subject:** Operators as sugar for traits, traits as generic
//!     constraints, monomorphization, and universal and existential types.
//!   - [**Audio**][mp3]
//!
//! [mp3]: https://www.podtrac.com/pts/redirect.mp3/cdn.newrustacean.com/file/newrustacean/e024.mp3
//!
//! <audio style="width: 100%" title="e024: Traits Deep Dive, Part 2" controls preload=metadata><source src="https://www.podtrac.com/pts/redirect.mp3/cdn.newrustacean.com/file/newrustacean/e024.mp3"></audio>
//!
//!
//! Show Notes
//! ----------
//!
//! - on monomorphization, see also [Sean Griffin's RustConf 2017 talk][sgrc2017]
//! - [zero-cost abstractions]
//!
//! [sgrc2017]: https://www.youtube.com/watch?v=wxPehGkoNOw
//! [zero-cost abstractions]: https://blog.rust-lang.org/2015/05/11/traits.html
//!
//! Sponsors
//! --------
//!
//!   - Aaron Turon
//!   - Alexander Payne
//!   - [Anthony Deschamps]
//!   - Anthony Scotti
//!   - Antonin Carette
//!   - Aleksey Pirogov
//!   - Andreas Fischer
//!   - Andrew Thompson
//!   - Austin LeSure
//!   - [Behnam Esfahbod]
//!   - Benjamin Wasty
//!   - Brent Vatne
//!   - Brian Casiello
//!   - Chap Lovejoy
//!   - [Charlie Egan]
//!   - Chris Jones
//!   - [Chris Palmer]
//!   - [Coleman McFarland]
//!   - Damien Stanton
//!   - Dan Abrams
//!   - [Daniel Collin]
//!   - Daniel Mason
//!   - [Daniel P. Clark]
//!   - [David W. Allen]
//!   - David Hewson
//!   - Derek Buckley
//!   - [Derek Morr]
//!   - Eugene Bulkin
//!   - [Hans Fjällemark]
//!   - [Henri Sivonen]
//!   - [Ian Jones]
//!   - [Jakub "Limeth" Hlusička]
//!   - James Cooper
//!   - Jerome Froelich
//!   - [John Rudnick]
//!   - Jon
//!   - Jonathan Turner
//!   - Joseph Hain
//!   - [Jupp Müller]
//!   - Justin Ossevoort
//!   - [Karl Hobley]
//!   - Keith Gray
//!   - Kilian Rault
//!   - Laurie Hedge
//!   - Luca Schmid
//!   - [Luiz Irber]
//!   - Mark LeMoine
//!   - Martin Heuschober
//!   - Masashi Fujita
//!   - Matt Rudder
//!   - Matthew Brenner
//!   - Matthias Ruszala
//!   - [Max Jacobson]
//!   - [Messense Lv]
//!   - Micael Bergeron
//!   - [Nathan Sculli]
//!   - [Nick Coish]
//!   - [Nick Stevens]
//!   - [Oluseyi Sonaiya]
//!   - Ovidiu Curcan
//!   - [Pascal Hertleif]
//!   - [Patrick O'Doherty]
//!   - [Paul Naranja]
//!   - Peter Tillemans
//!   - Ralph Giles ("rillian")
//!   - Raj Venkalil
//!   - [Ramon Buckley]
//!   - Randy MacLeod
//!   - Raph Levien
//!   - reddraggone9
//!   - [Ryan Blecher]
//!   - Ryan Osial
//!   - [Sebastián Ramírez Magrí]
//!   - Shane Utt
//!   - Simon G.
//!   - Steve Jenson
//!   - Steven Knight
//!   - Steven Murawski
//!   - [Stuart Hinson]
//!   - Tim Brooks
//!   - Timm Preetz
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
//! [Chris Palmer]: http://red-oxide.org/
//! [Coleman McFarland]: http://github.com/anxiousmodernman
//! [Daniel Collin]: https://twitter.com/daniel_collin
//! [Daniel P. Clark]: https://6ftdan.com/
//! [David W. Allen]: http://GitHub.com/DataRiot
//! [Derek Morr]: https://twitter.com/derekmorr
//! [Fjällemark]: https://fjallemark.com/
//! [Henri Sivonen]: https://hsivonen.fi/
//! [Ian Jones]: https://www.ianmjones.com/
//! [Jakub "Limeth" Hlusička]: https://github.com/Limeth
//! [John Rudnick]: http://www.cindur.com/
//! [Jupp Müller]: https://de.linkedin.com/in/juppm
//! [Karl Hobley]: https://github.com/kaedroho/
//! [Luiz Irber]: http://luizirber.org/
//! [Max Jacobson]: https://twitter.com/maxjacobson
//! [Messense Lv]: https://github.com/messense
//! [Nathan Sculli]: http://influential.co/
//! [Nick Coish]: http://github.com/ncoish
//! [Nick Stevens]: https://github.com/nastevens
//! [Oluseyi Sonaiya]: http://oluseyi.info/
//! [Pascal Hertleif]: https://pascalhertleif.de/
//! [Patrick O'Doherty]: https://twitter.com/patrickod
//! [Philipp Keller]: https://twitter.com/hansapla
//! [Ramon Buckley]: http://www.inosion.com
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

use std::fmt::Display;
use std::ops::Add;

#[doc = include_str!("../docs/e024-script.md")]
pub struct Script;

/// A data structure which will implement `std::ops::Add`.
///
/// With the implementation in place, we can do this:
///
/// ```rust
/// # use show_notes::e024::*;
/// # fn main() {
/// let a = Point {
///     x: 123,
///     y: 234,
///     z: 345,
/// };
///
/// let b = Point {
///     x: 456,
///     y: 567,
///     z: 678,
/// };
///
/// let c = a + b;
/// # }
/// ```
pub struct Point {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

/// Demonstrate a generic trait.
pub trait Eatable<T> {
    fn eat(&self) -> T;
}

impl Eatable<String> for i32 {
    fn eat(&self) -> String {
        self.to_string()
    }
}

/// Demonstrate the ability to take a trait constraint on a generic inline.
pub fn to_string_inline<T: Display>(t: T) -> String {
    format!("{}", t)
}

/// Demonstrate the ability to take a trait constraint on a generic with a
/// `where` clause.
pub fn to_string_with_where<T>(t: T) -> String
where
    T: Display,
{
    format!("{}", t)
}
