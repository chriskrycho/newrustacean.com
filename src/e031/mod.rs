//! FFI Deep Dive
//!
//! - **Date:** May 11, 2019
//! - **Subject:** FFI Deep Dive
//! - [**download mp3**][mp3]
//! - [**script**][script]
//!
//! [mp3]: https://www.podtrac.com/pts/redirect.mp3/f001.backblazeb2.com/file/newrustacean/e031.mp3
//! [script]: https://newrustacean.com/show_notes/e031/struct.script
//!
//! <audio style="width: 100%" title="e031: FFI Deep Dive" controls preload=metadata src="https://www.podtrac.com/pts/redirect.mp3/f001.backblazeb2.com/file/newrustacean/e031.mp3">
//!
//! Show Notes
//! ----------
//!
//! It's impossible to make the declarations below follow the order I talked
//! through them on the recording without also making them horrible to read, so
//! just use this outline instead:
//!
//! 1. [`add_in_rust`][1]
//! 2. Strings
//!     1. [`concat_strings`][2]
//!     2. [`free_rust_string`][3]
//! 3. [`Point`][4]
//!     1. [`point_translate`][5]
//! 4. [`union`][6]
//! 5. [`OpaquePoint`][7]
//!     1. [`opaque_point_new`][8]
//!     2. [`opaque_point_translate`][9]
//!     3. [`opaque_point_free`][10]
//!
//! [1]: https://newrustacean.com/target/doc/show_notes/e031/fn.add_in_rust.html
//! [2]: https://newrustacean.com/target/doc/show_notes/e031/fn.concat_strings.html
//! [3]: https://newrustacean.com/target/doc/show_notes/e031/fn.free_rust_string.html
//! [4]: https://newrustacean.com/target/doc/show_notes/e031/struct.Point.html
//! [5]: https://newrustacean.com/target/doc/show_notes/e031/fn.point_translate.html
//! [6]: https://newrustacean.com/target/doc/show_notes/e031/unions/index.html
//! [7]: https://newrustacean.com/target/doc/show_notes/e031/struct.OpaquePoint.html
//! [8]: https://newrustacean.com/target/doc/show_notes/e031/fn.opaque_point_new.html
//! [9]: https://newrustacean.com/target/doc/show_notes/e031/fn.opaque_point_translate.html
//! [10]: https://newrustacean.com/target/doc/show_notes/e031/fn.opaque_point_free.html
//!
//! ### Links
//!
//! - [Rust FFI Omnibus]
//! - [the repository for the show]
//! - [RFC #2195]
//! 
//! [Rust FFI Omnibus]: http://jakegoulding.com/rust-ffi-omnibus/
//! [the repository for the show]: https://github.com/chriskrycho/newrustacean
//! [RFC #2195]: https://github.com/rust-lang/rfcs/blob/master/text/2195-really-tagged-unions.md
//!
//! Sponsors
//! --------
//!
//! Thanks to Parity for sponsoring the show and hiring Rust developers!
//!
//! [parity]: https://www.parity.io/jobs
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
//! - <a href="https://www.patreon.com/newrustacean" rel="payment">Patreon</a>
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

use std::ffi::{CStr, CString};
use std::fmt::{Display, Error, Formatter};

use libc::{c_char, c_float, c_int};

#[doc(include = "../docs/e031-script.md")]
pub struct Script;

/// The simplest possible example of exposing Rust functions via a C FFI.
#[no_mangle]
pub extern "C" fn add_in_rust(a: c_int, b: c_int) -> c_int {
    a + b
}

/// Take two strings in and concatentate them without mutating either.
///
/// This allocates a new string, which *must* be deallocated by calling the
/// `free_rust_string` type exposed in this module.
#[no_mangle]
pub extern "C" fn concat_strings(first: *const c_char, second: *const c_char) -> *mut c_char {
    let (first, second) = unsafe {
        // Start by making sure the two strings are not null pointers (since C
        // APIs don't actually give us any help with this).
        assert!(!first.is_null());
        assert!(!second.is_null());

        // Then use `CString::from_ptr` to let Rust's own built-in smarts about
        // how to convert from a pointer to a `c_char` do the conversion
        // correctly. These are *not* the same as Rust `String`s, after all!
        (
            CStr::from_ptr(first).to_bytes(),
            CStr::from_ptr(second).to_bytes(),
        )
    };

    CStr::from_bytes_with_nul(&[&first[0..first.len()], &second[0..second.len()], b"\0"].concat())
        .expect("should be possible to construct a new `CStr` from existing `CStr`s")
        .to_owned()
        .into_raw()
}

/// Free any string allocated by Rust.
#[no_mangle]
pub extern "C" fn free_rust_string(to_free: *mut c_char) {
    // If the pointer is already `null`, we're done here. (Don't double `free`!)
    if to_free.is_null() {
        return;
    }

    // If the pointer is not already null, we take ownership of it again with
    // `from_raw` and then immediately free it by way of the inherent `Drop`.
    unsafe {
        CString::from_raw(to_free);
    }
}

/// A simple struct which we can expose to a C API. Note that it is `#[repr(C)]`!
#[repr(C)]
pub struct Point {
    /// x position -- made `pub` to indicate that we're exposing it to C!
    pub x: f32,
    /// y position -- made `pub` to indicate that we're exposing it to C!
    pub y: f32,
}

impl Point {
    fn translate(&mut self, by_x: f32, by_y: f32) {
        self.x += by_x;
        self.y += by_y;
    }
}

/// Expose an interface for C API callers to call the `Point` impl.
#[no_mangle]
pub extern "C" fn point_translate(point: *mut Point, by_x: c_float, by_y: c_float) {
    let point = unsafe {
        assert!(!point.is_null());
        &mut *point
    };

    // Note that if this wasn't safe, because for some reason `c_float` did not
    // match `f32`, the compiler would tell us.
    point.translate(by_x, by_y);
}

/// A struct identical to `Point`, but which is *not* `#[repr(C)]`!
///
/// The layout here is intentionally left in Rust's own representation, and we
/// do *not* expose the internals in `e031.h`.
pub struct OpaquePoint {
    x: f32,
    y: f32,
}

impl OpaquePoint {
    fn translate(&mut self, by_x: f32, by_y: f32) {
        self.x += by_x;
        self.y += by_y;
    }
}

impl Display for OpaquePoint {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "`{}, {}`", self.x, self.y)
    }
}

/// Expose an interface for C API callers to call the `OpaquePoint` impl.
///
/// This implementation is *identical* to the implementation of the `Point`
/// above. The only difference is that the C side doesn't get access to the
/// internal structure of the type… which is we want.
#[no_mangle]
pub extern "C" fn opaque_point_translate(point: *mut OpaquePoint, by_x: c_float, by_y: c_float) {
    let point = unsafe {
        assert!(!point.is_null());
        &mut *point
    };

    // Note that if this wasn't safe, because for some reason `c_float` did not
    // match `f32`, the compiler would tell us.
    point.translate(by_x, by_y);
}

#[no_mangle]
pub extern "C" fn opaque_point_new(x: c_float, y: c_float) -> *mut OpaquePoint {
    Box::into_raw(Box::new(OpaquePoint { x, y }))
}

#[no_mangle]
pub extern "C" fn opaque_point_describe(point: *mut OpaquePoint) -> *mut c_char {
    let point = unsafe {
        assert!(!point.is_null());
        &mut *point
    };

    CString::new(format!("{}", point))
        .expect("always safe to get `CString` from `String`")
        .into_raw()
}

/// Safely drops the `OpaquePoint` instance.
#[no_mangle]
pub extern "C" fn opaque_point_free(point: *mut OpaquePoint) {
    if point.is_null() {
        return;
    }

    unsafe { Box::from_raw(point) };
}

/// Demonstrate unions! Combines an `enum` and a `union` into a `struct` that
/// acts mostly like a regular Rust `enum`.
pub mod unions {
    /// Builds an instance of `Either`, a manually-managed "tagged union" type.
    ///
    /// If you read the body, you'll notice that we're not *helped* in any way
    /// by Rust like we are with normal `enum` types.
    pub fn demo_union() {
        // Here, we construct the type correctly.
        let either = Either::<i32, Wrapped<u32>> {
            tag: Tag::Left,
            value: EitherValue { left: 42 },
        };

        // But notice that the compiler doesn't help us! Comment out the
        // following lines and see that it still *compiles* just fine... but is
        // very much *not* correct semantically: we have a `Left` tag with a
        // `right` value!

        // let bad_either = Either::<i32, Wrapped<u32>> {
        //     tag: Tag::Left,
        //     value: EitherValue { right: Wrapped(42) },
        // };

        unsafe {
            match either {
                Either {
                    tag: Tag::Left,
                    value: EitherValue { left },
                } => {
                    dbg!(left);
                }
                Either {
                    tag: Tag::Right,
                    value: EitherValue { right },
                } => {
                    dbg!(right);
                }
            }
        }
    }

    /// For tagging the type in `Either`. See the body of `demo_union`.
    #[derive(Clone, Copy)]
    pub enum Tag {
        Left,
        Right,
    }

    /// A simple type designed to demo unions. See the body of `demo_union`.
    #[derive(Debug, Copy, Clone)]
    pub struct Wrapped<T: Copy + Clone>(T);

    /// A union, to be used as the inner value for `Either`.
    pub union EitherValue<L: Copy, R: Copy> {
        left: L,
        right: R,
    }

    /// Uses an `enum` and a `union` to get close to a regular Rust enum.
    ///
    /// Roughly, because the compiler won't check you for exhaustiveness, or
    /// even make sure you're using the tag and value pair the way you should!
    pub struct Either<L: Copy, R: Copy> {
        pub tag: Tag,
        pub value: EitherValue<L, R>,
    }
}
