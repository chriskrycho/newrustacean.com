//! I’m Out to C
//!
//! - **Date:** March 31, 2019
//! - **Subject:** Using Rust’s Foreign Function Interface (FFI) with C!
//! - [**download mp3**][mp3]
//! - [**script**][script]
//!
//! [mp3]: https://www.podtrac.com/pts/redirect.mp3/f001.backblazeb2.com/file/newrustacean/e029.mp3
//! [script]: https://newrustacean.com/show_notes/e027/struct.script
//!
//! <audio style="width: 100%" title="e029: I’m Out to C" controls preload=metadata src="https://www.podtrac.com/pts/redirect.mp3/f001.backblazeb2.com/file/newrustacean/e029.mp3">
//!
//! Show Notes
//! ----------
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
//! TODO: FILL THIS IN.
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
//!       + Twitter: [@newrustacean](https://www.twitter.com/newrustacean)
//!       + Email: [hello@newrustacean.com](mailto:hello@newrustacean.com)
//! - Chris Krycho
//!       + GitHub: [chriskrycho](https://github.com/chriskrycho)
//!       + Twitter: [@chriskrycho](https://www.twitter.com/chriskrycho)

/// An example of a slightly more complex data structure we can
#[repr(C)]
#[derive(Debug, PartialEq)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

extern "C" {
    /// An example of a C function we can call with an object.
    ///
    /// We wouldn't normally make this kind of code `pub`: instead, we'd provide
    /// a safe abstraction around it (see [e027]!). I've made it public here so
    /// you get docstring notes for it!
    ///
    /// ```rust
    /// # use show_notes::e029::*;
    /// let mut point = Point { x: 0.0, y: 0.0 };
    /// unsafe {
    ///     translate(&mut point, 12.3, 14.4);
    /// }
    /// assert_eq!(point, Point { x: 12.3, y: 14.4 });
    /// ```
    ///
    /// [e027]: http://newrustacean.com/show_notes/e027/
    pub fn translate(point: *mut Point, by_x: f32, by_y: f32);
}

#[doc(include = "../docs/e029-script.md")]
pub struct Script;
