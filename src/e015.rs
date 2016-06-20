//! Not dumb pointers.
//!
//!   - **Date:** June 17, 2016
//!   - **Subject:** `Box`, `String`, `Vec`, `Rc`, and `Arc` have this in
//!     common: they're not dumb.
//!   - **Audio:**
//!       + [M4A]
//!       + [MP3]
//!       + [Ogg]
//!
//! [M4A]: http://www.podtrac.com/pts/redirect.m4a/cdn.newrustacean.com/e015.m4a
//! [MP3]: http://www.podtrac.com/pts/redirect.mp3/cdn.newrustacean.com/e015.mp3
//! [Ogg]: http://www.podtrac.com/pts/redirect.ogg/cdn.newrustacean.com/e015.ogg
//!
//! <audio title="e014: Stringing things along" controls preload=metadata>
//!   <source src="http://www.podtrac.com/pts/redirect.m4a/cdn.newrustacean.com/e015.m4a">
//!   <source src="http://www.podtrac.com/pts/redirect.mp3/cdn.newrustacean.com/e015.mp3">
//!   <source src="http://www.podtrac.com/pts/redirect.ogg/cdn.newrustacean.com/e015.ogg">
//! </audio>
//!
//!
//! Notes
//! -----
//!
//! This episode, we take a close look at smart pointer types---from a few we've
//! already talked about, like `Box`, `Vec`, and `String`, to some new ones,
//! like `Rc` and `Arc`.
//!
//!   - What smart pointers are, and what makes them 'smart'.
//!   - Why we want or need smart pointers.
//!   - A bit about `Box`.
//!   - A lot more about `Rc` and `Arc`.
//!
//! ***Note:*** The examples below are in-progress: the `Rc` example is complete
//! but not fully documented, and there's no examples yet for `Arc`---but there
//! will be! I expect to finish them over the course of this weekend, but I
//! wanted to go ahead and get the episode out!
//!
//! ### Further reading
//!
//!   - _The Rust Programming Language_:
//!       + [The Stack and the Heap]
//!       + [Choosing Your Guarantees] -- see especially the sections on
//!         `Rc` and `Arc`.
//!   - Rust by Example: [17.1: Box, stack, and heap][rbe]
//!   - API docs:
//!       + [`std::boxed`]
//!       + [`std::rc`]
//!       + [`stc::sync::Arc`]
//!
//! Links
//! -----
//!
//!   - [RustConf]
//!   - [Rust Belt Rust Conference]
//!       + [sessions]
//!   - [Rusty Radio]
//!       + [feed]
//!   - [Rust Exercism track]
//!       + [All exercism language tracks]
//!   - [RFC 1636: Require documentation for all new features.][RFC1636] (Note:
//!     I misspoke on the episode and said this was at rust-lang.org; it's not!
//!     It's on GitHub, wtih the rest of the RFCs, of course.)
//!
//! [RustConf]: http://rustconf.com
//! [Rust Belt Rust Conference]: http://www.rust-belt-rust.com
//! [sessions]: http://www.rust-belt-rust.com/sessions/
//! [Rusty Radio]: https://soundcloud.com/posix4e/sets/rustyradio
//! [feed]: http://feeds.soundcloud.com/users/soundcloud:users:1287419/sounds.rss
//! [Rust Exercism track]: http://exercism.io/languages/rust
//! [All exercism language tracks]: http://exercism.io/languages
//! [RFC1636]: https://github.com/rust-lang/rfcs/pull/1636
//! [The Stack and the Heap]: https://doc.rust-lang.org/book/the-stack-and-the-heap.html
//! [Choosing Your Guarantees]: https://doc.rust-lang.org/book/choosing-your-guarantees.html
//! [rbe]: http://rustbyexample.com/std/box.html
//! [`std::boxed`]: https://doc.rust-lang.org/std/boxed/index.html
//! [`std::rc`]: https://doc.rust-lang.org/std/rc/index.html
//! [`stc::sync::Arc`]: https://doc.rust-lang.org/std/sync/struct.Arc.html
//!
//!
//! Sponsors
//! --------
//!
//!   - Aleksey Pirogov
//!   - [Chris Palmer]
//!   - [Daniel Collin]
//!   - [Derek Morr]
//!   - Doug Reeves
//!   - Hamza Sheikh
//!   - Lachlan Collins
//!   - Leif Arne Storset
//!   - Luca Schmid
//!   - Micael Bergeron
//!   - [Pascal Hertleif]
//!   - Ralph Giles ("rillian")
//!   - Ralph "FriarTech" Loizzo
//!   - Raph Levien
//!   - reddraggone9
//!   - Ryan Ollos
//!   - Vesa Kaihlavirta
//!   - [William Roe]
//!
//! [Chris Palmer]: http://home.red-oxide.org/
//! [Daniel Collin]: twitter.com/daniel_collin
//! [Derek Morr]: https://twitter.com/derekmorr
//! [Pascal Hertleif]: https://pascalhertleif.de/
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
//!
//!
//! Examples
//! --------
//!
//! The most basic examples of smart pointers involve the `Box` type, which
//! we've talked about before. Assume we had a type `Foo` which took a string in
//! its constructor, and that we wanted to box it up. We would just write:
//!
//! ```rust,ignore
//! let someFoo = Box::new(Foo::new("bar"));
//! ```
//!
//! It's also worth comparing the Rust code above with similar code in C++.
//! Assume we have a `class` with the same name; using a smart pointer (in this
//! case, `unique_ptr`) would give us this code:
//!
//! ```cpp
//! unique_ptr<Foo> someFoo(new Foo("bar"));
//! ```
//!
//! You can see that these are similar in length (the declaration is actually
//! the same number of characters). You're gaining *at least* some better
//! guarantees in Rust; I'd argue you're also making a substantial gain in the
//! clarity of the code (in that reading left to right as is normal in English,
//! it's much clearer how the pieces fit together without backtracking).
//!
//! I'm not including further comments on `Box` here in the docs, because we've
//! covered it before and it's fairly straightforward. The rest of these
//! materials focus entirely on `Rc` and `Arc`, as those are the most
//! interesting bits from today's episode.

use std::rc::{Rc,Weak};
// use std::sync::Arc;  // TODO


/// A trivial (and frankly rather silly) example for use with `Rc`.
pub struct FileData {
    contents: String,
}

impl FileData {
    pub fn new(contents: &str) -> FileData {
        FileData { contents: contents.to_string() }
    }
}

pub struct ASendableType {}


/// Note that this function is *generic*: it will work for any type.
pub fn print_rc_count<T>(t: &Rc<T>) {
    println!("Reference count: {:}", Rc::strong_count(&t));
}

/// Note that this function is not generic because it assumes `FileData`.
fn print_rc_body(fd: &Rc<FileData>) {
    println!("The contents are: {:}", fd.contents);
}


/// Demonstrate the basics of reference-counted types. (Read the source, Luke!)
pub fn demonstrate_rc() {
    // Note that we have valid data here.
    let a_ref = get_wrapped_file_data();
    print_rc_body(&a_ref);
    print_rc_count(&a_ref);  // Just 1
    let added_another_ref = a_ref.clone();
    print_rc_count(&a_ref);  // 2

    // Create a block to show that we can get another copy.
    {
        let yet_another_ref = a_ref.clone();
        print_rc_count(&yet_another_ref);  // 3
        print_rc_body(&yet_another_ref);  // we can print the contents here.
    }  // we've gone out of scope; `yet_another_ref` is deallocated here.

    print_rc_count(&a_ref);  // 2 again
    drop(a_ref);  // Remember, it doesn't matter which we drop!
    print_rc_count(&added_another_ref);  // 1
    print_rc_body(&added_another_ref);  // valid

    // Most explicit form:
    let a_weak_ref: Weak<FileData> = Rc::downgrade(&added_another_ref);
    // clone the weak ref.
    let _another_weak_ref = a_weak_ref.clone();
    print_rc_count(&added_another_ref);  // still 1.

    // Now we *move* the reference into the other function.
    let empty_weak = get_empty_weak(added_another_ref);

    match empty_weak.upgrade() {
        Some(fd) => println!("{:}", fd.contents),
        None => println!("Nothing to see here. We're done."),
    }
}


/// Note that this takes ownership of the data.
pub fn get_empty_weak(fd: Rc<FileData>) -> Weak<FileData> {
    Rc::downgrade(&fd)
}


pub fn get_wrapped_file_data() -> Rc<FileData> {
    let plain_data = FileData::new("This would really read from a file. And not be terrible.");
    // Both of these now have "strong" references to the type. Neither "trumps"
    // the other; whichever goes out of scope first will be deallocated, but
    // *without* affecting the other.
    let wrapped = Rc::new(plain_data);
    print_rc_count(&wrapped);
    let a_reference_to_it = wrapped.clone();
    print_rc_count(&a_reference_to_it);

    a_reference_to_it
}
