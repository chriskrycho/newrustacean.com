//! Traits Deep Dive, Part 3
//!
//!   - **Date:** July 4, 2018
//!   - **Subject:** Closure traits, `impl trait`, `dyn trait`, and object safety!
//!   - [**Script**][script]
//!   - [**Audio**][mp3]
//!
//! [script]: https://newrustacean.com/show_notes/e025/struct.script
//! [mp3]: https://www.podtrac.com/pts/redirect.mp3/f001.backblazeb2.com/file/newrustacean/e025.mp3
//!
//! <audio style="width: 100%" title="" controls preload=metadata>
//!   <source src="https://www.podtrac.com/pts/redirect.mp3/f001.backblazeb2.com/file/newrustacean/e025.mp3">
//! </audio>
//!
//!
//! Show Notes
//! ----------
//!
//! Sponsored by [Parity Technologies][parity]! Parity is hiring Rust developers
//! so if you're interested, you should check out their [job listings][jobs]!
//!
//! [parity]: https://paritytech.io
//! [jobs]: https://paritytech.io/jobs/
//!
//! ### Links
//!
//! - [RFC #1733: Trait Aliases][1733]
//! - [RFC #255: Object Safety][255]
//! - [Ch. 17 in the Second Edition of *The Rust Programming Language*][trpl]
//! - [Huon Wilson's post][huon]
//!
//! [1733]: https://github.com/rust-lang/rfcs/blob/master/text/1733-trait-alias.md
//! [255]: https://github.com/rust-lang/rfcs/blob/master/text/0255-object-safety.md
//! [trpl]: https://doc.rust-lang.org/book/second-edition/ch17-02-trait-objects.html
//! [huon]: https://huonw.github.io/blog/2015/05/where-self-meets-sized-revisiting-object-safety/ "Where Self Meets Sized: Revisiting Object Safety"
//!
//! ### Example
//!
//! You can see all of the pieces of the final example described in the show
//! here (and the module has the required definitions for `Point`).
//!
//! ```rust
//! # use show_notes::e025::*;
//! # fn main() {
//! let points = vec![
//!     Point { x: 1.0, y: 2.0 },
//!     Point { x: 12.0, y: 4.3 },
//!     Point { x: -5.4, y: 18.7 },
//! ];
//!
//! let origin = Point::default();
//!
//! // This is the version we start with. It works fine, but it's not elegant.
//! let distances_inline: Vec<f32> = points
//!     .iter()
//!     .map(|point| {
//!         let change = point - &origin;
//!         (change.x.powi(2) + change.y.powi(2)).sqrt()
//!     })
//!     .collect();
//!
//! // This version is *much* cleaner!
//! let distances_impl: Vec<f32> = points.iter().map(distance_from_impl(&origin)).collect();
//! # }
//! ```
//!
//! Sponsors
//! --------
//!
//!   - Aaron Turon
//!   - Alexander Kryvomaz
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
//!   - [Ramon Buckland]
//!   - Randy MacLeod
//!   - Raph Levien
//!   - reddraggone9
//!   - [Robert Chrzanowski]
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
//! [Ramon Buckland]: http://www.inosion.com
//! [Robert Chrzanowski]: https://github.com/zirman
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

use std::{f32, ops::Sub};

#[derive(Default)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

impl<'p> Sub for &'p Point {
    type Output = Point;

    fn sub(self, other: &Point) -> Point {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

pub fn distance_from_boxed<'a, 'b: 'a>(offset: &'b Point) -> Box<FnMut(&'a Point) -> f32 + 'a> {
    Box::new(move |point| {
        let change = point - offset;
        (change.x.powi(2) + change.y.powi(2)).sqrt()
    })
}

pub type DistanceFrom<'a> = Box<FnMut(&'a Point) -> f32 + 'a>;

pub fn distance_from_alias<'a, 'b: 'a>(offset: &'b Point) -> DistanceFrom<'a> {
    Box::new(move |point| {
        let change = point - offset;
        (change.x.powi(2) + change.y.powi(2)).sqrt()
    })
}

pub fn distance_from_impl<'a, 'b: 'a>(offset: &'b Point) -> impl FnMut(&'a Point) -> f32 {
    move |point| {
        let change = point - offset;
        (change.x.powi(2) + change.y.powi(2)).sqrt()
    }
}

#[rustc::nightly]
#[doc(include = "../docs/e025-script.md")]
pub struct Script;
