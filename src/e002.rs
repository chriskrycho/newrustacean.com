//! Something borrowed, something... moved?
//!
//!   - **Date:** October 12, 2015
//!   - **Subject:** The `struct` data type constructor, and the basics of
//!     Rust's "ownership" concept and "borrowing" and "moving".
//!   - **Audio:**
//!       + [M4A][m4a]
//!       + [MP3][mp3]
//!       + [Ogg][ogg]
//!
//! [m4a]: http://www.podtrac.com/pts/redirect.m4a/cdn.newrustacean.com/e002.m4a
//! [mp3]: http://www.podtrac.com/pts/redirect.mp3/cdn.newrustacean.com/e002.mp3
//! [ogg]: http://www.podtrac.com/pts/redirect.ogg/cdn.newrustacean.com/e002.ogg
//!
//! <audio title="Something borrowed, something... moved?" controls preload=metadata>
//!   <source src="http://www.podtrac.com/pts/redirect.m4a/cdn.newrustacean.com/e002.m4a">
//!   <source src="http://www.podtrac.com/pts/redirect.mp3/cdn.newrustacean.com/e002.mp3">
//!   <source src="http://www.podtrac.com/pts/redirect.ogg/cdn.newrustacean.com/e002.ogg">
//! </audio>
//!
//! # Notes
//!
//! Today's episode discusses, and the associated source code demonstrates,
//! a few basic behaviors of structs... including borrowing!
//!
//! After taking a short look at one of Rust's basic approaches to creating new
//! types, we dive into a *fairly* thorough overview of how borrowing works in
//! fairly run-of-the-mill Rust code. This is a basic introduction, and as such
//! I'm not getting into things like heap-allocated memory (`Box`) or dealing
//! with `move` semantics with threads or closures. (I haven't actually figured
//! those out well enough yet to write something like this for them!)
//!
//! As usual, you'll want to have the [`src`] open to see what I'm doing with
//! the components documented below.
//!
//! [`src`]: /src/show_notes/e002.rs.html
//!
//! # Links
//!
//!   - `rustfmt` -- a tool for formatting Rust code
//!       + [repo][1]
//!       + ["rustfmt-ing Rust`][2]
//!       + [Reddit discussion][3]
//!   - RFC for incremental compilation
//!       + [Text of the RFC][4]
//!       + [GitHub pull request][5]
//!
//! [1]: https://github.com/nrc/rustfmt
//! [2]: http://www.ncameron.org/blog/rustfmt-ing-rust/
//! [3]: https://www.reddit.com/r/rust/comments/3nt2vm/rustfmting_rust_please_help_me_rustfmt_the_rust/
//! [4]: https://github.com/nikomatsakis/rfcs/blob/incremental-compilation/text/0000-incremental-compilation.md
//! [5]: https://github.com/rust-lang/rfcs/pull/1298
//!
//! # Follow/Support
//!
//!   - New Rustacean:
//!       + Twitter: [@newrustacean](https://www.twitter.com/newrustacean)
//!       + App.net: [@newrustacean](https://alpha.app.net/newrustacean)
//!       + [Patreon](https://www.patreon.com/newrustacean)
//!       + Email: [hello@newrustacean.com](mailto:hello@newrustacean.com)
//!   - Chris Krycho
//!       + Twitter: [@chriskrycho](https://www.twitter.com/chriskrycho)
//!       + App.net: [@chriskrycho](https://alpha.app.net/chriskrycho)

/// This struct is simple but useful to see how borrowing and moving work.
#[derive(Debug)]
pub struct Circle {
    /// X position of the circle's origin.
    pub x: f64,
    /// Y position of the circle's origin
    pub y: f64,
    /// Radius of the circle
    pub r: f64,
}

/// Demonstrates how borrowing works with plain old functions.
pub fn demonstrate_ownership() {
    // Just create a couple circles whose origin is at the origin.
    let immutable = Circle::origin(14.0);
    let mut mutable = Circle::origin(12.0);

    // Borrow an immutable one immutably.
    borrow(&immutable);

    // You can't borrow an immutable type mutably.
    // borrow_mut(&mut immutable);

    // You can borrow a mutable type immutably, *or* mutably.
    borrow(&mutable);
    borrow_mut(&mut mutable);
    println!("Updated the radius of the mutable reference: {:}",
             mutable.r);

    // If I move an object, I can no longer access it afterward. If you
    // uncomment the following line, you'll see that the Circle instances are
    // no longer accessible. Note that moving and borrowing are orthogonal to
    // mutability. It's not accessible after this because it actually gets
    // dropped, i.e. de-allocated.
    move_circle(immutable);
    move_circle(mutable);
    // println!("Immutable: {:?}", immutable);
    // println!("Mutable: {:?}", immutable);
}


/// Demonstrates general borrowing of an immutable reference.
pub fn borrow(ref_to_circle: &Circle) {
    // I can access but not change the values.
    println!("Immutable reference-- origin: {:},{:} | Radius: {:}",
             ref_to_circle.x,
             ref_to_circle.y,
             ref_to_circle.r);

    // You can uncomment this to see the compiler error if you try to *change*
    // the contents.
    // ref_to_circle.r *= 2.0;

    // Similarly, if you try to reassign the reference itself, it will fail:
    // *ref_to_circle = Circle::new()
}


/// Demonstrates general borrowing of a mutable reference.
pub fn borrow_mut(mutable_ref_to_circle: &mut Circle) {
    // I can still access these values, of course.
    println!("Mutable reference-- origin: {:},{:} | Radius: {:}",
             mutable_ref_to_circle.x,
             mutable_ref_to_circle.y,
             mutable_ref_to_circle.r);

    // But I can also update the values.
    mutable_ref_to_circle.r *= 2.0;
}


/// Demonstrates general moving of an instance.
pub fn move_circle(moved_circle: Circle) {
    println!("Moved a circle: {:?}", moved_circle);

    // Note that this is an immutable type: this won't compile.
    // moved_circle.r *= 2.0;

    // We own the object, but it's still immutable, so we can't reassign the
    // name as we like.
    // moved_circle = Circle::new(4.5, 9.0, 18.0);
}


/// Implement some methods on the `Circle`.
///
/// This lets use demonstrate both how methods work in general and specifically
/// how they interact with the idea of ownership.
impl Circle {
    /// Creates a `Circle` instance centered on the "origin" (x = 0, y = 0).
    fn origin(r: f64) -> Circle {
        Circle {
            x: 0.0,
            y: 0.0,
            r: r,
        }
    }

    /// Creates a `Circle` instance centered on specified x, y values.
    pub fn new(x: f64, y: f64, r: f64) -> Circle {
        Circle { x: x, y: y, r: r }
    }

    /// Returns the value of `Circle.x`, borrowing an immutable reference to
    /// the circle to do it.
    ///
    /// Because the reference is immutable, if you tried to do this---
    ///
    /// ```ignore
    /// self.x = 10;
    /// ```
    ///
    /// ---the compiler would not allow it.
    pub fn x_by_ref(&self) -> f64 {
        println!("Taking a reference.");
        // The reference is immutable.
        self.x
    }

    // Returns the value of `Circle.x`, borrowing a mutable reference to the
    // circle and changing the value (demonstrating a situation in which you
    // would want to use a mutable rather than immutable reference).
    pub fn x_by_mut_ref(&mut self) -> f64 {
        println!("Taking a mutable reference.");
        self.x = self.x + 1.0;
        self.x
    }

    /// Returns the value of `Circle.x`, taking ownership of the circle. As a
    /// result of the change in ownership, the circle goes out of scope after
    /// the method returns, so the circle instance will be inaccessible after
    /// that.
    ///
    /// Note that the item is taken as immutable, so attempting to change the
    /// internals will still fail. **Ownership is orthogonal to immutability.**
    pub fn by_take(self) -> f64 {
        println!("Taking ownership, not just borrowing a reference. INTENSE.");
        self.x
    }

    /// Returns the value of `Circle.x`, taking ownership of a mutable circle.
    pub fn by_take_mut(mut self) -> f64 {
        println!("Taking ownership *and* mutating all the things.");
        self.x += 14.0;
        self.x
    }
}


/// Demonstrates how the same concepts apply when dealing with methods.
pub fn demonstrate_method_ownership() {
    // There are several ways to construct struct types. The first is a plain
    // struct constructor.
    let basic = Circle {
        x: 1.0,
        y: 2.0,
        r: 5.0,
    };
    // The next is using a constructor.
    let mut mutable = Circle::new(14.0, 12.0, 10.0);
    // Constructors can have default behavior, like this one.
    let immutable = Circle::origin(12.0);
    println!("{:?}", basic);
    println!("{:?}", immutable);
    println!("{:?}", mutable);

    // Now we can see how different method calls work. When we borrow a
    // reference, there is no consequence: during the life of the method, the
    // immutable object is "borrowed" by the `x_by_ref` method. Ownership returns
    // to the `main` function when it returns.
    let by_ref_x = immutable.x_by_ref();
    println!("By reference: {}", by_ref_x);

    // The same is true of a method that borrows a mutable reference. Note,
    // however, that since the reference is mutable, it can change the contents
    // of the struct.
    print!("Original x_by_mut_ref x value: {}... ", mutable.x);
    let by_mut_ref_x = mutable.x_by_mut_ref();
    println!("After calling `x_by_mut_ref`: {}", by_mut_ref_x);

    // Note that both of these struct instances are still available for `main`
    // to use once they return from the method call:
    println!("immutable's y: {} | mutable's r: {}",
             immutable.y,
             mutable.r);

    // A method that takes ownership can still return a value. However, it has
    // a very different behavior in terms of the struct instance, in a way that
    // seems surprising compared to languages without the concept of "borrowing"
    // vs. "taking" (C, Java, Python, etc.).
    //
    // Run a method that takes ownership rather than just borrowing a reference.
    // The value returned is as expected. However, the item has been *moved*,
    // not just borrowed. That is, `main` no longer owns it---`by_take` took
    // over the ownership. Once `by_take` ended, `immutable` actually gets
    // deconstructed ("dropped") because it went out of scope. As a result, the
    // following line won't compile (you can uncomment it to see for yourself):
    let by_take_x = immutable.by_take();
    println!("By take: {}", by_take_x);
    // println!("Does the circle still exist? {:?}", immutable);

    // Likewise, if we use the mutable circle with the method which takes
    // ownership and mutates it, we can get the value back as before, but the
    // circle will have been dropped when the `by_take_mut` call ended.
    let by_mut_take_x = mutable.by_take_mut();
    println!("By mutable take: {}", by_mut_take_x);
    // println!("Does the mutable circle still exist? {:?}", mutable);
}
