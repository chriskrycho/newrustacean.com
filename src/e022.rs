//! `Send` and `Sync`
//!
//!   - **Date:** January 31, 2018
//!   - **Subject:** The “marker traits” that Rust uses for safe concurrency.
//!   - [**Audio**][mp3]
//!
//! [mp3]: https://www.podtrac.com/pts/redirect.mp3/cdn.newrustacean.com/file/newrustacean/e022.mp3
//!
//! <audio style="width: 100%" title="e022: Send and Sync" controls preload=metadata>
//!   <source src="https://www.podtrac.com/pts/redirect.mp3/cdn.newrustacean.com/file/newrustacean/e022.mp3">
//! </audio>
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
//!   - [Coleman McFarland]
//!   - Dan Abrams
//!   - [Daniel Collin]
//!   - [Daniel P. Clark]
//!   - [David W. Allen]
//!   - David Hewson
//!   - [Derek Morr]
//!   - Eugene Bulkin
//!   - Guido Hoermann
//!   - [Hans Fjällemark]
//!   - Hendrik Sollich
//!   - [Henri Sivonen]
//!   - [Ian Jones]
//!   - [Jakub "Limeth" Hlusička]
//!   - James Cooper
//!   - Jerome Froelich
//!   - [John Rudnick]
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
//!   - [Paul Naranja]
//!   - Peter Tillemans
//!   - Ralph Giles ("rillian")
//!   - Randy MacLeod
//!   - Raph Levien
//!   - reddraggone9
//!   - [Ryan Blecher]
//!   - [Sebastián Ramírez Magrí]
//!   - Shane Utt
//!   - Simon G.
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

use std::thread;

#[doc(include = "../docs/e022-script.md")]
pub struct Script;

/// How `Send` and `Sync` work
///
/// Here we have a function which spawns a thread to run `println!()`. This is
/// silly, of course, but it works because (and only because!) `String`
/// implements the `Send` trait.
///
/// ```
/// # use std::thread;
/// pub fn demo_send() {
///     let foo = String::from("Hallo!");
///     thread::spawn(move || {
///         println!("{}", foo);
///     });
/// }
/// ```
///
/// If we had a *non-`Send` type here – e.g. an `Rc<String>` instead of just a
/// `String` – we would get a compiler error:
///
/// ```compile_fail
/// # use std::thread;
/// # use std::rc::Rc;
/// pub fn will_fail() {
///     let foo = Rc::new(String::from("Hallo!"));
///     thread::spawn(|| {
///         println!("{:?}", foo);
///     });
/// }
/// ```
///
/// Instead, you'll get a compiler error:
///
/// ```plain
/// error[E0277]: the trait bound `std::rc::Rc<std::string::String>: std::marker::Sync` is not satisfied
///    --> src/e022.rs:214:5    |
/// 214 |     thread::spawn(|| {
///     |     ^^^^^^^^^^^^^ `std::rc::Rc<std::string::String>` cannot be shared between threads safely
///     |
///     = help: the trait `std::marker::Sync` is not implemented for `std::rc::Rc<std::string::String>`
///     = note: required because of the requirements on the impl of `std::marker::Send` for `&std::rc::Rc<std::string::String>`
///     = note: required because it appears within the type `[closure@src/e022.rs:214:19: 216:6 foo:&std::rc::Rc<std::string::String>]`
///     = note: required by `std::thread::spawn`
///```
///
/// Notice that it's common to see those errors together, and notice moreover
/// that the error here includes *both* `Send` and `Sync`. The compiler tries to
/// see if `foo` can be taken via reference, but the `Rc` type is not `Sync`,
/// i.e. you cannot share references to it across threads, and that's so because
/// it is not `Send`. (Strictly speaking, all `Sync` types are `Send`, but not
/// all `Send` types must be `Sync`, though off the top of my head I can't think
/// of a scenario where a type *would* be `Send` but *not* also `Sync`.)
///
/// What's somewhat curious is that there really isn't a lot more than this to
/// demo here! To get into anything more interesting in terms of
/// *implementation* related to these traits, we'd have to be off in `unsafe`
/// land, and we haven't talked about `unsafe` at *all* yet, so we're not going
/// to do that today. However, if you want to see a good example of what makes
/// for code that *can* be safely shared across threads, take a look at the
/// implementation of [`Vec`] and the [`RawVec`] type it's built on! Reading the
/// standard library is a really good way to learn things about Rust, and it's
/// surprisingly straightforward most of the time.
///
/// [`Vec`]: https://github.com/rust-lang/rust/blob/master/src/liballoc/vec.rs
/// [`RawVec`]: https://github.com/rust-lang/rust/blob/master/src/liballoc/raw_vec.rs
pub fn demo_send_and_sync() {
    let foo = 12;
    thread::spawn(move || {
        println!("{}", foo);
    });
}
