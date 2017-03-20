//! Point me where I need to go
//!
//!   - **Date:** August 5, 2016
//!   - **Subject:** A deep dive on references and pointers in Rust.
//!   - **Audio:**
//!       + [M4A]
//!       + [MP3]
//!       + [Ogg]
//!
//! [M4A]: http://www.podtrac.com/pts/redirect.m4a/cdn.newrustacean.com/e017.m4a
//! [MP3]: http://www.podtrac.com/pts/redirect.mp3/cdn.newrustacean.com/e017.mp3
//! [Ogg]: http://www.podtrac.com/pts/redirect.ogg/cdn.newrustacean.com/e017.ogg
//!
//! <audio style="width: 100%" title="Point me where I need to go" controls preload=metadata>
//!   <source src="http://www.podtrac.com/pts/redirect.m4a/cdn.newrustacean.com/e017.m4a">
//!   <source src="http://www.podtrac.com/pts/redirect.mp3/cdn.newrustacean.com/e017.mp3">
//!   <source src="http://www.podtrac.com/pts/redirect.ogg/cdn.newrustacean.com/e017.ogg">
//! </audio>
//!
//!
//! Notes
//! -----
//!
//! By listener request, today we look at the syntax and semantics of
//! referencing and dereferencing and the corresponding `&` and `*` operators.
//!
//! As was the case with [e016], the code samples have little to say in their
//! documentation; *reading* the code will be necessary for seeing the ideas.
//!
//! [e016]: http://newrustacean.dev/show_notes/e016/
//!
//!
//! Links
//! -----
//!
//! - ["Inside the Fastest Font Renderer in the World"][raph]
//! - The Rust Platform:
//!     + [original blog post][blog]
//!         * [Rust internals discussion][internals]
//!         * [Reddit discussion][reddit1]
//!         * [Hacker News discussion][hn1]
//!     + [follow-up]
//!         * [Reddit discussion][reddit2]
//! - [Cargo vendoring support in nightly][vendoring]
//! - [MIR on by default in nightly][MIR]
//! - References and dereferencing:
//!     + _The Rust Programming Language_:
//!         * [References and Borrowing][book:rab]
//!         * [`Deref` coercions][book:deref]
//!     + _Rust by Example_: [Flow Control: pointers/ref][rbe]
//!     + The Rust Reference:
//!         * [Unary Operator Expressions][uoe]
//!         * [Pointer Types][pointer-types]
//!
//! [raph]: https://medium.com/@raphlinus/inside-the-fastest-font-renderer-in-the-world-75ae5270c445#.1opn7gihv
//! [blog]: http://aturon.github.io/blog/2016/07/27/rust-platform/
//! [internals]: https://internals.rust-lang.org/t/proposal-the-rust-platform/3745
//! [reddit1]: https://www.reddit.com/r/rust/comments/4uxdn8/the_rust_platform_aaron_turon/?
//! [hn1]: https://news.ycombinator.com/item?id=12177002
//! [follow-up]: https://internals.rust-lang.org/t/follow-up-the-rust-platform/3782
//! [reddit2]: https://www.reddit.com/r/rust/comments/4v9eo0/follow_up_to_the_rust_platform/?
//! [vendoring]: https://users.rust-lang.org/t/cargo-vendoring-now-on-nightly/6776
//! [MIR]: https://github.com/rust-lang/rust/pull/34096
//! [book:rab]: https://doc.rust-lang.org/book/references-and-borrowing.html
//! [book:deref]: https://doc.rust-lang.org/book/deref-coercions.html
//! [rbe]: http://rustbyexample.com/flow_control/match/destructuring/destructure_pointers.html
//! [uoe]: https://doc.rust-lang.org/reference.html#unary-operator-expressions
//! [pointer-types]: https://doc.rust-lang.org/reference.html#pointer-types
//!
//!
//! Sponsors
//! --------
//!
//!   - Aleksey Pirogov
//!   - Cameron Mochrie
//!   - Cass Costello
//!   - [Chris Palmer]
//!   - [Daniel Collin]
//!   - [Derek Morr]
//!   - Doug Reeves
//!   - Eric Fulmer
//!   - Hamza Sheikh
//!   - [Jakub "Limeth" Hlusička]
//!   - [Jared Smith]
//!   - Keith Gray
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
//!   - Steven Murawksi
//!   - Vesa Kaihlavirta
//!   - [William Roe]
//!
//! [Chris Palmer]: http://home.red-oxide.org/
//! [Daniel Collin]: twitter.com/daniel_collin
//! [Derek Morr]: https://twitter.com/derekmorr
//! [Jakub "Limeth" Hlusička]: https://github.com/Limeth
//! [Jared Smith]: http://twitter.com/jaredthecoder
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


/// A dummy container for use with references.
pub struct DataStore<'a> {
    pub contents: &'a i32
}

impl<'a> DataStore<'a> {
    pub fn new(contents: &'a i32) -> DataStore {
        DataStore { contents: contents }
    }
}


/// Give a basic example of how the reference operator works.
pub fn demonstrate_ref() {
    let twelve = 12;
    let ref_twelve = &twelve;

    // Both of these work because of the magic of automatic dereferencing, which
    // we'll talk about next week.
    println!("{:?}", twelve);
    println!("{:?}", ref_twelve);
}


/// A simple example of using the dereference operator.
pub fn demonstrate_deref() {
    // Note here we can create a reference right out of the gate, and pass it
    // without applying the `&` operator again.
    let forty_two_ref = &42;
    let some_data = DataStore::new(forty_two_ref);

    // As above! Note that the types are not the same, but dereferencing happens
    // under the covers.
    println!("{:}", some_data.contents);
    println!("{:}", *some_data.contents);
}


/// A simple demonstration of matching against a reference type.
pub fn demonstrate_match() {
    let four = 4;
    let ref_to_four = &four;

    // Here, if we try to reference the contents of `store` directly, we'll find
    // that it doesn't work: we get a type error.
    let store = DataStore::new(ref_to_four);  // type: DataStore
    let optional_store = Some(store);  // type: Option<DataStore>
    let contents = match optional_store {
        Some(store) => *store.contents,
        None => 0
    };

    println!("{:?}", contents);
}
