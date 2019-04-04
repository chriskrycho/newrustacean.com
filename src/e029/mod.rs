//! I’m Out to C
//!
//! - **Date:** March 31, 2019
//! - **Subject:** Using Rust’s Foreign Function Interface (FFI) with C!
//! - [**download mp3**][mp3]
//! - [**script**][script]
//!
//! [mp3]: https://www.podtrac.com/pts/redirect.mp3/f001.backblazeb2.com/file/newrustacean/e029.mp3
//! [script]: https://newrustacean.com/show_notes/e029/struct.script
//!
//! <audio style="width: 100%" title="e029: I’m Out to C" controls preload=metadata src="https://www.podtrac.com/pts/redirect.mp3/f001.backblazeb2.com/file/newrustacean/e029.mp3">
//!
//! Show Notes
//! ----------
//!
//! The code samples here directly match the things I described in the show, so
//! you will likely want to look at [`add`] and [`ffi::add`], then [`Point`],
//! [`translate`], and [`ffi::translate`] in that order.
//!
//! [`add`]: ./fn.add.html
//! [`ffi::add`]: ./ffi/fn.add.html
//! [`Point`]: ./struct.Point.html
//! [`translate`]: ./fn.translate.html
//! [`ffi::translate`]: ./ffi/fn.translate.html
//!
//! ### Links
//!
//!
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

use std::os::raw::c_float;

/// An example of a slightly more complex data structure we can use with FFI.
///
/// Note the `#[repr(C)]`, which tells Rust to make sure this struct is laid out
/// the way that the C ABI expects. That's *not* the way that Rust's own ABI
/// (which is unstable and can change at any time) might lay it out.
///
/// Note also that `x` and `y` are `c_float`, which is [a type alias][c_float]
/// for `f32`. We use it here to make explicit the interop, and also because it
/// is *possible* that it might change on some specific operating system. If we
/// went to compile for an operating system where C's `float` type were *not* a
/// 32-point floating bit number (and the C standard does not require it to be
/// anything but "a floating point number"), the compiler would let us know.
///
/// [c_float]: https://doc.rust-lang.org/1.33.0/std/os/raw/type.c_float.html
#[repr(C)]
#[derive(Debug, PartialEq)]
pub struct Point {
    pub x: c_float,
    pub y: c_float,
}

/// The module with all the unsafe code in it! You'll want to poke at this!
///
/// (Note that this is private, and we wouldn't normally make this kind of code
/// visible in documentation at all. Instead, we would *only* make public the
/// safe abstraction around it which we're providing at the root of this module
/// (see [e027]!). I've set a `RUSTDOCFLAGS` value in the `Makefile` at the root
/// of the repo so you get these docstring notes for it!)
///
/// [e027]: https://newrustacean.com/show_notes/e027/
mod ffi {
    use super::Point;
    use std::os::raw::{c_float, c_int};

    extern "C" {
        /// A *mostly*-trivial example: addition in C instead of in Rust. (See
        /// the docs for `e029::add` for why it *isn't* totally trivial.)
        ///
        /// You can use it in an `unsafe` block like so:
        ///
        /// ```rust,ignore
        /// unsafe {
        ///     let result = add(1, 2); // 3, of course!
        /// }
        /// ```
        pub(super) fn add(a: c_int, b: c_int) -> c_int;

        /// An example of a C function we can call with an object.
        ///
        /// You can use it in an `unsafe` block like so:
        ///
        /// ```rust,ignore
        /// let mut point = Point { x: 0.0, y: 0.0 };
        /// unsafe {
        ///     translate(&mut point, 12.3, 14.4);
        /// }
        /// assert_eq!(point, Point { x: 12.3, y: 14.4 });
        /// ```
        ///
        /// [e027]: http://newrustacean.com/show_notes/e027/
        pub(super) fn translate(point: *mut Point, by_x: c_float, by_y: c_float);
    }
}

#[doc(include = "../docs/e029-script.md")]
pub struct Script;

/// A safe interface for the unsafe `ffi::add`.
///
/// Note that this particular check is as silly as calling out to C for addition
/// is, but it shows how you can provide a safe wrapper for a case where C's
/// implementation differences *might* actually matter to you.
///
/// While it might seem that something like addition is trivially safe, it turns
/// out to be *mostly* safe. The behavior of overflow for signed integers is
/// *not defined* for C. In Rust, it *is* defined, by [RFC #0560]: in modes
/// where `debug_assertions` are enabled, an overflow will cause a panic; in
/// modes where those assertions are not enabled (i.e. release mode), Rust wraps
/// them by [two's complement]. The net of that is that even something this
/// simple can have unexpected results when calling across the FFI boundary.
///
/// [RFC #0560]: https://github.com/rust-lang/rfcs/blob/master/text/0560-integer-overflow.md
/// [two's complement]: https://en.wikipedia.org/wiki/Two's_complement
///
/// ```rust
/// # use show_notes::e029::add;
/// assert_eq!(add(1, 2), Some(3));
/// ```
pub fn add(a: i32, b: i32) -> Option<i32> {
    if i32::max_value() - a >= b {
        unsafe { Some(ffi::add(a, b)) }
    } else {
        None
    }
}

/// A safe interface for the unsafe `ffi::translate` function.
///
/// In this case, there are no invariants we need to maintain other than those
/// which Rust *always* maintains, i.e. that the reference we have with
/// `&mut Point` is a valid reference (not `null`, actually points to a `Point`,
/// and so on). Since Rust guarantees this, we can simply *directly* call the
/// unsafe extern function here.
///
/// I explicitly included the cast `as *mut Point`, but an `&mut Point` will be
/// automatically converted to a `*mut Point` when needed, so it is unnecessary
/// from a compiler perspective. It may, however, be helpful for making your
/// intent clear to other users!
///
/// ```rust
/// # use show_notes::e029::{translate, Point};
/// let mut point = Point { x: 0.0, y: 0.0 };
/// translate(&mut point, 2.4, 4.8);
/// assert_eq!(point, Point { x: 2.4, y: 4.8 });
/// ```
pub fn translate(point: &mut Point, by_x: f32, by_y: f32) {
    unsafe {
        ffi::translate(point as *mut Point, by_x as c_float, by_y as c_float);
    }
}
