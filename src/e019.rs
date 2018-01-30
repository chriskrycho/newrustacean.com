//! Let's `Clone` a `Cow`!
//!
//!   - **Date:** February 28, 2017
//!   - **Subject:** The final pieces of the story for (single-threaded) memory
//!     management in Rust.
//!   - **Audio:**
//!       + [M4A](http://www.podtrac.com/pts/redirect.m4a/cdn.newrustacean.com/e019.m4a)
//!       + [MP3](http://www.podtrac.com/pts/redirect.mp3/cdn.newrustacean.com/e019.mp3)
//!       + [Ogg](http://www.podtrac.com/pts/redirect.ogg/cdn.newrustacean.com/e019.ogg)
//!
//! <audio style="width: 100%" title="Let's `Clone` a `Cow`!" controls preload=metadata>
//!   <source src="http://www.podtrac.com/pts/redirect.m4a/cdn.newrustacean.com/e019.m4a">
//!   <source src="http://www.podtrac.com/pts/redirect.mp3/cdn.newrustacean.com/e019.mp3">
//!   <source src="http://www.podtrac.com/pts/redirect.ogg/cdn.newrustacean.com/e019.ogg">
//! </audio>
//!
//!
//! Notes
//! -----
//!
//! Sometimes, we actually *do* need to copy types. Wouldn't it be nice if Rust
//! gave us a convenient way to do that when it's convenient, or when the cost
//! is low enough that the ergonomic tradeoffs are worth it? Well, perhaps
//! unsurprisingly, it does! The `Copy` and `Clone` traits, plus the `Cow` type,
//! give us everything we need!
//!
//!
//! Links
//! -----
//!
//! - [underhanded.rs]
//!
//! - The typess
//!
//!     - [`std::marker::Copy`]
//!
//!         - ["`Copy` types" in the book][copy-book]
//!
//!         - ["Stack-Only Data: Copy" in the new book][copy-new-book]
//!
//!         - [7.2.0.2 Moved and copied types]:
//!
//!             > When a local variable is used as an rvalue, the variable will be copied if its
//!               type implements `Copy`. All others are moved.
//!
//!         - [Extended example in "Traits" section of new book][copy-traits-example-new-book]
//!
//!     - [`std::clone::Clone`]
//!
//!     - [`std::borrow::Cow`]
//!
//! - Default implementations
//!
//!     - [discussion in the current book][default-book]
//!
//!     - [discussion in the new book][default-new-book]
//!
//! - Supertraits
//!
//!     - from the discussion in the reference ([6.1.9 Traits]):
//!
//!         > Traits may inherit from other traits.... The syntax `Circle : Shape` means that types
//!           that implement `Circle` must also have an implementation for `Shape`. Multiple
//!           supertraits are separated by `+`, trait `Circle : Shape + PartialEq { }`. In an
//!           implementation of `Circle` for a given type `T`, methods can refer to `Shape` methods,
//!           since the typechecker checks that any type with an implementation of `Circle` also has
//!           an implementation of `Shape`...
//!
//!     - [discussion of trait "inheritance" in the book][trait-inheritance]
//!
//!     - [discussion of trait super- and subtyping in the new book][trait-inheritance-new-book]
//!       (note: still to-be-written at the time this episode was published)
//!
//! - Marker traits
//!
//!     - [`std::marker`]
//!
//!     - in the reference: [9 Special Traits]
//!
//! - Previous episodes on traits:
//!
//!     - [e008: Just like something else][e008]
//!
//!     - [e009: Composing a Rustic tune][e009]
//!
//!
//! [underhanded.rs]: https://underhanded.rs/
//! [`std::marker::Copy`]: https://doc.rust-lang.org/stable/std/marker/trait.Copy.html
//! [copy-book]: https://doc.rust-lang.org/book/ownership.html#copy-types
//! [copy-new-book]: http://rust-lang.github.io/book/ch04-01-what-is-ownership.html#stack-only-data-copy
//! [7.2.0.2 Moved and copied types]: https://doc.rust-lang.org/reference.html#moved-and-copied-types
//! [copy-traits-example-new-book]: http://rust-lang.github.io/book/ch10-02-traits.html#fixing-the-largest-function-with-trait-bounds
//! [`std::clone::Clone`]: https://doc.rust-lang.org/stable/std/clone/trait.Clone.html
//! [`std::borrow::Cow`]: https://doc.rust-lang.org/stable/std/borrow/enum.Cow.html
//! [default-book]: https://doc.rust-lang.org/book/traits.html#default-methods
//! [default-new-book]: http://rust-lang.github.io/book/ch10-02-traits.html#default-implementations
//! [6.1.9 Traits]: https://doc.rust-lang.org/reference.html#traits
//! [trait-inheritance]: https://doc.rust-lang.org/book/traits.html#inheritance
//! [trait-inheritance-new-book]: http://rust-lang.github.io/book/ch19-00-advanced-features.html
//! [`std::marker`]: https://doc.rust-lang.org/stable/std/marker/
//! [9 Special Traits]: https://doc.rust-lang.org/reference.html#special-traits
//! [e008]: http://www.newrustacean.com/show_notes/e008/
//! [e009]: http://www.newrustacean.com/show_notes/e009/
//!
//!
//! Sponsors
//! --------
//! 
//!
//!   - Aleksey Pirogov
//!   - Andreas Fischer
//!   - Andrew Thompson
//!   - Austin LeSure
//!   - Ben Whitley
//!   - [Charlie Egan]
//!   - [Chris Palmer]
//!   - [Christopher Giffard]
//!   - [Daniel Collin]
//!   - [Derek Morr]
//!   - [Jakub "Limeth" Hlusička]
//!   - Jordan Henderson
//!   - [Jupp Müller]
//!   - Keith Gray
//!   - Lachlan Collins
//!   - Luca Schmid
//!   - Matt Rudder
//!   - Matthew Piziak
//!   - [Max Jacobson]
//!   - Micael Bergeron
//!   - Ovidiu Curcan
//!   - [Pascal Hertleif]
//!   - Peter Tillemans
//!   - Philipp Keller
//!   - Ralph Giles ("rillian")
//!   - Raph Levien
//!   - reddraggone9
//!   - Steven Murawksi
//!   - [Stuart Hinson]
//!   - Tyler Harper
//!   - Vesa Kaihlavirta
//!   - Vlad Bezden
//!   - [William Roe]
//!   - Zaki
//!
//! [Charlie Egan]: https://charlieegan3.com
//! [Chris Palmer]: http://home.red-oxide.org/
//! [Christopher Giffard]: http://blog.cgiffard.com
//! [Daniel Collin]: https://twitter.com/daniel_collin
//! [Derek Morr]: https://twitter.com/derekmorr
//! [Jakub "Limeth" Hlusička]: https://github.com/Limeth
//! [Jupp Müller]: https://de.linkedin.com/in/juppm
//! [Max Jacobson]: https://twitter.com/maxjacobson
//! [Pascal Hertleif]: https://pascalhertleif.de/
//! [Philipp Keller]: https://twitter.com/hansapla
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


/// A non-copyable point type
///
/// # Examples
///
/// ```rust,ignore
/// # use show_notes::e019::ANoCopyOrClonePoint;
/// let a_point = ANoCopyOrClonePoint::origin();
/// let moved_point = a_point;  // <- moves the value
/// println!("{:?}", a_point);  // <- so this is a problem!
/// println!("{:?}", moved_point)
/// ```
///
/// The output is just what we would expect from the discussion on the show:
///
/// ```plain
/// error[E0382]: use of moved value: `a_point`
/// --> <anon>:6:18
/// |
/// 5 | let moved_point = a_point;  // <- moves the value
/// |     ----------- value moved here
/// 6 | println!("{:?}", a_point);  // <- so this is a problem!
/// |                  ^^^^^^^ value used here after move
/// |
/// = note: move occurs because `a_point` has type `show_notes::e019::ANoCopyOrClonePoint`, which does not implement the `Copy` trait
/// ```
///
/// If we comment the offending line out, however, it compiles just fine.
///
/// ```rust
/// # use show_notes::e019::ANoCopyOrClonePoint;
/// let a_point = ANoCopyOrClonePoint::origin();
/// let moved_point = a_point;
/// // println!("{:?}", a_point);
/// println!("{:?}", moved_point);  // <- not a problem!
/// ```
/// 
/// [(You can confirm this in the playground.)][playground]
///
/// [playground]: https://is.gd/PZBWw0
#[derive(Debug)]
pub struct ANoCopyOrClonePoint {
    x: f64,
    y: f64,
    z: f64,
}

impl ANoCopyOrClonePoint {
    /// Generate a point at 0, 0, 0
    pub fn origin() -> ANoCopyOrClonePoint {
        ANoCopyOrClonePoint { x: 0.0, y: 0.0, z: 0.0 }
    }
}


/// A struct which implements `Clone` but not `Copy`.
///
/// # Examples
///
/// If you don't do anything, the item will be moved on assignment:
///
/// ```rust,ignore
/// # use show_notes::e019::BJustClonePoint;
/// let a_point = BJustClonePoint::origin();
/// let cloned_point = a_point;  // <- moves the value
/// println!("{:?}", a_point);  // <- compile error!
/// println!("{:?}", cloned_point);
/// ```
///
/// The output is just what we would expect from the discussion on the show:
///
/// ```plain
/// error[E0382]: use of moved value: `a_point`
/// --> <anon>:6:18
/// |
/// 5 | let cloned_point = a_point;  // <- moves the value
/// |     ------------ value moved here
/// 6 | println!("{:?}", a_point);  // <- compile error!
/// |                  ^^^^^^^ value used here after move
/// |
/// = note: move occurs because `a_point` has type `show_notes::e019::BJustClonePoint`, which does not implement the `Copy` trait
/// ```
///
///
/// But you can manually call the `clone` method, and it will work:
///
/// ```rust
/// # use show_notes::e019::BJustClonePoint;
/// let a_point = BJustClonePoint::origin();
/// let cloned_point = a_point.clone();
/// println!("{:?}", a_point);  // <- not a problem
/// println!("{:?}", cloned_point);
/// ```
#[derive(Clone,Debug)]
pub struct BJustClonePoint {
    x: f64,
    y: f64,
    z: f64,
}

impl BJustClonePoint {
    pub fn origin() -> BJustClonePoint {
        BJustClonePoint { x: 0.0, y: 0.0, z: 0.0 }
    }
}


/// A struct with identical behavior to `ANoCopyOrClonePoint`, except with `Copy`.
///
/// Note that we have `Clone` as well as `Copy` here---we have to, since
/// `Clone` is a supertrait for `Copy`.
///
/// # Examples
///
/// Note that this is just like the non-compiling example in [`ANoCopyOrClonePoint`], but because
/// `CCopyPoint` implements `Copy`, the line which previously caused a compile
/// error now works without any issue.
///
/// ```rust
/// # use show_notes::e019::CCopyPoint;
/// let a_point = CCopyPoint::origin();
/// let copied_point = a_point;
/// println!("{:?}", a_point);  // <- not a problem
/// println!("{:?}", copied_point)
/// ```
///
/// [`ANoCopyOrClonePoint`]: /show_notes/e019/struct.ANoCopyOrClonePoint.html
#[derive(Clone,Copy,Debug)]
pub struct CCopyPoint {
    x: f64,
    y: f64,
    z: f64,
}

impl CCopyPoint {
    /// Generate a point at 0, 0, 0
    pub fn origin() -> CCopyPoint {
        CCopyPoint { x: 0.0, y: 0.0, z: 0.0 }
    }
}

/// The `Cow` type can wrap around other types and make them "reusable".
/// 
/// Note that the body of this function is identical with that of the body of
/// the example below.
/// 
/// # Examples
/// 
/// We'll reuse the `BJustClonePoint` since `Cow::Owned` requires that the
/// underlying type implement `Clone`.
/// 
/// ```rust
/// # use std::borrow::Cow;
/// # use show_notes::e019::{BJustClonePoint,demonstrate_cow};
/// let a_point = Cow::Owned(BJustClonePoint::origin());
/// demonstrate_cow(&a_point);
/// ```
/// 
/// Note that even though `demonstrate_cow` takes a reference to
/// `BJustClonePoint`, we can pass it the `Cow` instance; this is where the
/// `Deref` implementation on `Cow` comes in handy.
pub fn demonstrate_cow(_point: &BJustClonePoint) {}


/// What if we need a mutable reference to the wrapped type?
/// 
/// # Examples
/// 
/// We can get a mutable reference to the wrapped item, even if the wrapped item
/// isn't itself mutable, as long as it's `Clone`-able. In this case, we're
/// making a copy---this is explicit in the `to_mut()` call. If the underlying
/// item isn't mutably accessible, we'll just get a mutable copy.
/// 
/// ```rust
/// # use std::borrow::Cow;
/// # use show_notes::e019::{BJustClonePoint,demonstrate_mut_cow};
/// let mut a_point: Cow<BJustClonePoint> = Cow::Owned(BJustClonePoint::origin());
/// demonstrate_mut_cow(a_point.to_mut());
/// ```
pub fn demonstrate_mut_cow(_point: &mut BJustClonePoint) {}