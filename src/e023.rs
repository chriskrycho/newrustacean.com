//! Traits Deep Dive, Part 1
//!
//!   - **Date:** April 28, 2018
//!   - **Subject:** Defining and using your own traits, using other crates' traits, and the orphan rule.
//!   - [**Audio**][mp3]
//!
//! [mp3]: https://www.podtrac.com/pts/redirect.mp3/cdn.newrustacean.com/file/newrustacean/e023.mp3
//!
//! <audio style="width: 100%" title="e023: Traits Deep Dive, Part 1" controls preload=metadata><source src="https://www.podtrac.com/pts/redirect.mp3/cdn.newrustacean.com/file/newrustacean/e023.mp3"></audio>
//!
//!
//! Show Notes
//! ----------
//!
//! Traits—
//!
//! - [in the Rust book][book]
//! - [in Rust by Example][rbe]
//!
//! [book]: https://doc.rust-lang.org/book/second-edition/ch10-02-traits.html
//! [rbe]: https://rustbyexample.com/trait.html
//!
//! Also of interest: *specialization*:
//!
//! - [RFC #1210][1210]
//! - recent blog posts:
//!     + ["Maximally minimal specialization: always applicable impls"][mm]
//!     + ["Sound and ergonomic specialization for Rust"][sound]
//!
//! [1210]: https://rust-lang.github.io/rfcs/1210-impl-specialization.html
//! [PR]: https://github.com/rust-lang/rfcs/pull/1210
//! [mm]: http://smallcultfollowing.com/babysteps/blog/2018/02/09/maximally-minimal-specialization-always-applicable-impls/
//! [sound]: http://aturon.github.io/2018/04/05/sound-specialization/
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
//!   - Dan Abrams
//!   - [Daniel Collin]
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
//!   - [Paul Naranja]
//!   - Peter Tillemans
//!   - Ralph Giles ("rillian")
//!   - Raj Venkalil
//!   - [Ramon Buckley]
//!   - Randy MacLeod
//!   - Raph Levien
//!   - reddraggone9
//!   - [Ryan Blecher]
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

use regex::Regex;

#[doc = include_str!("../docs/e023-script.md")]
pub struct Script;

/// A trait, to be implemented by types like `Breakfast`.
pub trait Eatable {
    /// The stuff to do *before* you eat things. (No implementation!)
    fn prepare(&self) -> String;

    /// How to eat things. (Has a default implementation.)
    fn eat(&self) -> String {
        self.prepare();
        String::from("Open mouth, insert food!")
    }
}

/// Just another way to eat things – exists to show conflicting trait methods.
pub trait Nomable {
    fn eat(&self) -> String {
        String::from("nom nom nom!")
    }
}

/// Breakfast is eatable, so `Breakfast` is `Eatable`!
#[allow(dead_code)]
pub enum Breakfast {
    Waffles,
    Cereal,
    Pancakes,
}

impl Eatable for Breakfast {
    fn prepare(&self) -> String {
        String::from(match *self {
            Breakfast::Cereal => "Just add milk",
            Breakfast::Pancakes | Breakfast::Waffles => "Butter plus syrup for the win",
        })
    }
}

/// Moar Breakfast is even better, so `MoarBreakfast` is also `Eatable` and `Nomable`!
#[allow(dead_code)]
pub enum MoarBreakfast {
    Waffles,
    Cereal,
    Pancakes,
    FrenchToast,
    Bagels,
}

impl Nomable for MoarBreakfast {}

impl Eatable for MoarBreakfast {
    fn prepare(&self) -> String {
        String::from(match *self {
            MoarBreakfast::Waffles | MoarBreakfast::Pancakes => "Pour syrup",
            MoarBreakfast::Cereal => "Add milk",
            MoarBreakfast::FrenchToast => "Sprinkle on powdered sugar, pour syrup",
            MoarBreakfast::Bagels => "Lightly toast, and spread cream cheese",
        })
    }

    fn eat(&self) -> String {
        let preparation = self.prepare();
        let consumption = match *self {
            MoarBreakfast::Waffles | MoarBreakfast::Pancakes | MoarBreakfast::FrenchToast => {
                "shovel into mouth with fork"
            }
            MoarBreakfast::Cereal => "enjoy the crunch",
            MoarBreakfast::Bagels => "use your fingers, of course",
        };

        format!("{}, then {}", preparation, consumption)
    }
}

impl Eatable for Regex {
    fn prepare(&self) -> String {
        String::from("This is truly absurd.")
    }

    fn eat(&self) -> String {
        format!("{} But we can do it anyway!", self.prepare())
    }
}

/// Shows how you can use traits with your own and others' types.
pub fn demo_eatable() {
    let basic = Breakfast::Cereal;
    basic.eat();

    let moar = MoarBreakfast::FrenchToast;
    Nomable::eat(&moar);

    let re = Regex::new("hallo").expect("'hallo' is a valid regex");
    re.eat();
}
