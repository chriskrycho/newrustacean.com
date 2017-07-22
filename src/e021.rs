//! Keeping your types under cover
//! 
//!   - **Date:** July 17, 2017
//!   - **Subject:** Using type aliases and creating custom type wrappers for
//!     more expressive and safer code.
//!   - **Audio:**
//!       + [MP3][mp3]
//!       + [M4A][m4a]
//!       + [Ogg][ogg]
//!
//! [mp3]: http://www.podtrac.com/pts/redirect.mp3/cdn.newrustacean.com/e021.mp3
//! [m4a]: http://www.podtrac.com/pts/redirect.m4a/cdn.newrustacean.com/e021.m4a
//! [ogg]: http://www.podtrac.com/pts/redirect.ogg/cdn.newrustacean.com/e021.ogg
//!
//! <audio style="width: 100%" style="width: 100%" title="e021: Keeping your types under cover" controls preload=metadata>
//!   <source src="http://www.podtrac.com/pts/redirect.mp3/cdn.newrustacean.com/e021.mp3">
//!   <source src="http://www.podtrac.com/pts/redirect.m4a/cdn.newrustacean.com/e021.m4a">
//!   <source src="http://www.podtrac.com/pts/redirect.ogg/cdn.newrustacean.com/e021.ogg">
//! </audio>
//!
//!
//! Links and Notes
//! ---------------
//! 
//! - [`Deref`]
//! - [`Iterator`]
//! - [`std::io::Result`]
//! 
//! [`Deref`]: https://doc.rust-lang.org/stable/std/ops/trait.Deref.html
//! [`Iterator`]: https://doc.rust-lang.org/stable/std/iter/trait.Iterator.html
//! [`std::io::Result`]: https://doc.rust-lang.org/stable/std/io/type.Result.html
//! 
//! 
//! Sponsors
//! --------
//! 
//!   - [Anthony Deschamps]
//!   - Anthony Scotti
//!   - Aleksey Pirogov
//!   - Andreas Fischer
//!   - Andrew Thompson
//!   - Austin LeSure
//!   - Benham Esfabod
//!   - Benjamin Wasty
//!   - Brent Vatne
//!   - [Charlie Egan]
//!   - Chris Jones
//!   - [Chris Palmer]
//!   - Dan Abrams
//!   - [Daniel Collin]
//!   - [David W. Allen]
//!   - [Derek Morr]
//!   - Eugene Bulkin
//!   - [Henri Sivonen]
//!   - [Jakub "Limeth" Hlusička]
//!   - James Cooper
//!   - Jonathan Turner
//!   - Jordan Henderson
//!   - [Jupp Müller]
//!   - Justin Ossevoort
//!   - [Karl Hobley]
//!   - Keith Gray
//!   - Kilian Rault
//!   - Luca Schmid
//!   - Matt Rudder
//!   - Matthew Piziak
//!   - [Max Jacobson]
//!   - [Messense Lv]
//!   - Micael Bergeron
//!   - Ovidiu Curcan
//!   - [Pascal Hertleif]
//!   - [Patrick O'Doherty]
//!   - Peter Tillemans
//!   - Philipp Keller
//!   - Ralph Giles ("rillian")
//!   - Randy MacLeod
//!   - Raph Levien
//!   - reddraggone9
//!   - [Sebastián Ramírez Magrí]
//!   - Steven Murawksi
//!   - [Stuart Hinson]
//!   - Tim Brooks
//!   - Tom Prince
//!   - Ty Overby
//!   - Tyler Harper
//!   - Vesa Kaihlavirta
//!   - Warren Harper
//!   - [William Roe]
//!   - Zaki
//!
//! [Anthony Deschamps]: https://github.com/adeschamps
//! [Brent Vatne]: https://github.com/brentvatne
//! [Charlie Egan]: https://charlieegan3.com
//! [Chris Palmer]: http://home.red-oxide.org/
//! [Daniel Collin]: https://twitter.com/daniel_collin
//! [David Allen]: http://GitHub.com/DataRiot
//! [Derek Morr]: https://twitter.com/derekmorr
//! [Henri Sivonen]: https://hsivonen.fi/
//! [Jakub "Limeth" Hlusička]: https://github.com/Limeth
//! [Jupp Müller]: https://de.linkedin.com/in/juppm
//! [Karl Hobley]: https://github.com/kaedroho/
//! [Max Jacobson]: https://twitter.com/maxjacobson
//! [Messense Lv]: https://github.com/messense
//! [Pascal Hertleif]: https://pascalhertleif.de/
//! [Patrick O'Doherty]: https://twitter.com/patrickod
//! [Philipp Keller]: https://twitter.com/hansapla
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


use std::ops::Deref;


/// A type alias for a string that is appropriate to use as an email address.
pub type Email = String;

/// A "newtype" built by wrapping a `String` in a tuple struct.
/// 
/// Declare it and destructure it:
/// 
/// ```
/// # use show_notes::e021::*;
/// let email = EmailStruct("hello@newrustacean.com".into());
/// let EmailStruct(the_underlying_string) = email;
/// send(the_underlying_string);
/// ```
/// 
/// Note that we can implement `map` on it:
/// 
/// ```
/// # use show_notes::e021::*;
/// let email = EmailStruct("hello@newrustacean.com".into());
/// let transformed = email.map(|value| value.replace("newrustacean", "massaffection"));
/// println!("contact email for my other show is {:?}", transformed);
/// ```
/// 
/// And we can implement a (very badly behaving) `Iterator` on it, too. (And
/// I do mean *badly* behaved: do *not* attempted to use this iterator without
/// setting a limit on it, as it is infinite.)
/// 
/// ```
/// # use show_notes::e021::*;
/// let mut email = EmailStruct("hello@newrustacean.com".into());
/// let next = email.next();
/// let and_the_next = email.next();
/// assert_eq!(next, and_the_next, "It always returns the same thing 😱");
/// ```
/// 
/// (If we wanted to implement a better-behaved iterator, we'd need to do
/// substantially more work; for a good example of that on a wrapper type like
/// this, see [Result] or [Option].)
/// 
/// [Result]: https://doc.rust-lang.org/stable/std/result/enum.Result.html
/// [Option]: https://doc.rust-lang.org/stable/std/option/enum.Option.html
#[derive(Debug, PartialEq, Eq)]
pub struct EmailStruct(pub String);

impl EmailStruct {
    pub fn map<F: FnOnce(String) -> String>(self, f: F) -> EmailStruct {
        let EmailStruct(the_string) = self;
        EmailStruct(f(the_string))
    }
}

impl Iterator for EmailStruct {
    type Item = EmailStruct;

    fn next(&mut self) -> Option<EmailStruct> {
        let &mut EmailStruct(ref the_address) = self;
        Some(EmailStruct(the_address.clone()))
    }
}

impl Deref for EmailStruct {
    type Target = String;

    fn deref(&self) -> &String {
        // Return a reference to the underlying `String`, which itself
        // implements `Deref` for `&str`. That plus the `AsRef` impls
        // for each of those layers means you'll be able to write
        // `&EmailStruct` and simply pass it in to things expecting
        // `&str`.
        &self.0
    }
    
}

/// A "newtype" built by wrapping a `String` in a single-variant enum.
/// 
/// It's simple to use to create a wrapped variant, and then because it is
/// a single-variant enum, we can also destructure it:
/// 
/// ```
/// # use show_notes::e021::*;
/// let email = EmailEnum::Address("hello@newrustacean.com".into());
/// let EmailEnum::Address(just_the_string) = email;
/// send(just_the_string);
/// ```
/// 
/// Note, however, that it is *much* more verbose.
pub enum EmailEnum {
    Address(String),
}

impl Deref for EmailEnum {
    type Target = String;
    
    fn deref(&self) -> &String {
        match *self {
            EmailEnum::Address(ref string) => &string
        }
    }
}

/// A simple thing to demonstrate destructuring
/// 
/// ```
/// # use show_notes::e021::*;
/// let thing = ThingToDestructure {
///     a_field: "Neat!".into(),
///     another: 42,
/// };
/// 
/// let ThingToDestructure { a_field, another: can_rename } = thing;
/// println!("`a_field` is {} and another (`can_rename`) is {}", a_field, can_rename);
/// ```
pub struct ThingToDestructure {
    /// Just a field we can destructure.
    pub a_field: String,
    /// And another field we can destructure.
    pub another: i32,
}

/// A simple function showing you can use a `String` where an `Email` is required.
/// 
/// ```
/// # use show_notes::e021::*;
/// send(String::from("hello@newrustacean.com"));
/// ```
pub fn send(_to_address: Email) {}


/// A function which takes a string, to use with `Deref` and `EmailStruct`.
/// 
/// E.g. you can do this:
/// 
/// ```
/// # use show_notes::e021::*;
/// let email_struct = EmailStruct("hello@newrustacean.com".into());
/// takes_a_str(&email_struct);
/// ```
/// 
/// And likewise, with the enum variant:
/// 
/// ```
/// # use show_notes::e021::*;
/// let email_enum = EmailEnum::Address("hello@newrustacean.com".into());
/// takes_a_str(&email_enum)
/// ```
/// 
/// Note, however, that the `Deref` implementation is *much* more complicated
/// for the enum variant, as everything is in general.
pub fn takes_a_str(_some_str: &str) {}
