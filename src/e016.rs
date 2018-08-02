//! `RefCell`s and code smells
//!
//!   - **Date:** July 23, 2016
//!   - **Subject:** Digging deeper on smart pointers and mutability with
//!     `Cell` and `RefCell`.
//!   - [**Audio**][mp3]
//!
//! [mp3]: https://www.podtrac.com/pts/redirect.mp3/f001.backblazeb2.com/file/newrustacean/e016.mp3
//!
//! <audio style="width: 100%" title="Borrow a Cell or Clone a Cow" controls preload=metadata>
//!   <source src="https://www.podtrac.com/pts/redirect.mp3/f001.backblazeb2.com/file/newrustacean/e016.mp3">
//! </audio>
//!
//!
//! Notes
//! -----
//!
//! What are the `Cell` and `RefCell` types, and when should we use them?
//!
//! Today, we follow up both the detailed discussion of smart pointers in
//! [e015] and the closely related discussion in [Interview 2] with Raph Levien,
//! and look at two types you need to have a good idea how to deal with if you
//! want to use these smart pointer types more *ergonomically*---that is, how to
//! use them without feeling like you're beating your head against a wall!
//!
//! [e015]: https://www.newrustacean.com/show_notes/e015/
//! [Interview 2]: https://www.newrustacean.com/show_notes/interview/_2/
//!
//! The descriptions of the code below are minimal; see the inline comments in
//! [the source][src] for the actual informative discussion.
//!
//! [src]: /src/show_notes/e016.rs.html
//!
//!
//! ### A comment on the code samples
//!
//! Note that in several cases below we use `&[]` to borrow a reference to a
//! slice, rather than requiring this to pass a reference to a `Vec`
//! specifically. Making the first argument be of type
//! `&Vec<RefCell<SimpleNonCopyable>>` would also work, but would be more
//! restrictive in what it could and couldn't accept. Since `Vec` implements
//! [`Deref`] to automatically convert to slices, this works just fine, *and* is
//! more general. This is how you should usually write function signatures which
//! operate on reference to vectors (and likewise for other types which can
//! dereference to slices). We'll talk about this more in a future episode!
//!
//! [`Deref`]: https://doc.rust-lang.org/1.10.0/std/ops/trait.Deref.html
//!
//!
//! Links
//! -----
//!
//!   - Rust 1.10
//!       + [blog post]
//!       + [full release notes]
//!       + cdylib
//!           * RFC: [text] | [discussion]
//!           * [implementation]
//!   - [rustup 0.3.0 release]
//!   - [Integer32] \(Carol Nichols' and Jake Goulding's new Rust consultancy)
//!       + [announcement blog post]
//!       + [reddit discussion][reddit i32]
//!   - [IntelliJ Rust]
//!       + [reddit discussion][reddit ijr]
//!   - Tango: [source][Tango] | [talk]
//!   - `Cell` and `RefCell`:
//!       + [Cell Types] in _The Rust Programming Language_
//!       + Standard library:
//!           * [`std::cell`] module docs (detailed explanation, complementary to the book)
//!           * [`Cell<T>`] docs (at `std::cell::Cell`)
//!           * [`RefCell<T>`] docs (at `stc::cell::RefCell`)
//!
//! [blog post]: https://blog.rust-lang.org/2016/07/07/Rust-1.10.html
//! [full release notes]: https://github.com/rust-lang/rust/blob/master/RELEASES.md#version-1100-2016-07-07
//! [text]: https://github.com/rust-lang/rfcs/blob/master/text/1510-rdylib.md
//! [discussion]: https://github.com/rust-lang/rfcs/pull/1510
//! [implementation]: https://github.com/rust-lang/rust/pull/33553
//! [rustup 0.3.0 release]: https://internals.rust-lang.org/t/beta-testing-rustup-rs/3316/144
//! [Integer32]: http://www.integer32.com
//! [announcement blog post]: http://www.integer32.com/2016/07/11/why-rust.html
//! [reddit i32]: https://www.reddit.com/r/rust/comments/4sdncw/why_were_starting_a_rust_consultancy/
//! [IntelliJ Rust]: https://intellij-rust.github.io
//! [reddit ijr]: https://www.reddit.com/r/rust/comments/4sbqaq/intellijrust_has_a_website_now/
//! [Tango]: https://github.com/pnkfelix/tango
//! [talk]: https://skillsmatter.com/skillscasts/8372-tango-literate-programming-in-rust
//! [Cell Types]: https://doc.rust-lang.org/book/choosing-your-guarantees.html#cell-types
//! [`std::cell`]: https://doc.rust-lang.org/std/cell/index.html
//! [`Cell<T>`]: https://doc.rust-lang.org/std/cell/struct.Cell.html
//! [`RefCell<T>`]: https://doc.rust-lang.org/std/cell/struct.RefCell.html
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
//!   - Eric Fulmer
//!   - Hamza Sheikh
//!   - [Jakub "Limeth" Hlusička]
//!   - Keith Gray
//!   - Lachlan Collins
//!   - Leif Arne Storset
//!   - Luca Schmid
//!   - Micael Bergeron
//!   - Michael Clayton
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
//! [Jakub "Limeth" Hlusička]: https://github.com/Limeth
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

use std::cell::{Cell, RefCell};

/// A container showing a type where `Cell<T>` works with `Vec<T>`.
#[derive(Copy, Clone)]
pub struct SimpleContainer {
    contents: i32,
}

/// Demonstrate how you need `Cell<T>` even just with a `Vec<T>`;
pub fn demonstrate_need_for_cell() -> Vec<Cell<SimpleContainer>> {
    let a_simple_container = SimpleContainer { contents: 42 };
    let another_one = SimpleContainer { contents: 84 };
    let _a_vec = vec![a_simple_container, another_one];
    // So far so good, but say we want to change the contents of one of the
    // struct types here. Uncomment the following line; it will fail to compile.
    //
    // let retrieved = _a_vec.get_mut(1);
    //
    // If, however, we use a `Cell`, we can get at the mutable data. See the
    // test below!
    vec![Cell::new(a_simple_container), Cell::new(another_one)]
}

/// Operate mutably on the contenets of an immutable reference to a `Vec`.
pub fn double_cell(containers: &[Cell<SimpleContainer>]) {
    for container_cell in containers {
        let old = container_cell.get().contents;
        let new = old * 2;
        container_cell.set(SimpleContainer { contents: new });
    }
}

/// A container showing where `Cell<T>` doesn't work and `RefCell<T>` does.
pub struct SimpleNonCopyable {
    contents: String,
}

/// Demonstrate interior mutability with `Rc` and `RefCell`.
pub fn add_to_each_string(list_contained_strings: &[RefCell<SimpleNonCopyable>], to_push: &str) {
    for contained in list_contained_strings {
        let mut original = contained.borrow_mut();
        original.contents.push_str(to_push);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn need_for_cell() {
        // Here we have an immutable `Vec<Cell<SimpleContainer>>`
        let all_wrapped_up = demonstrate_need_for_cell();
        assert_eq!(all_wrapped_up.get(0).unwrap().get().contents, 42);
        // And yet its contents can be changed! The `Cell::set()` method lets us
        // update the *interior* of the item.
        all_wrapped_up
            .get(0)
            .unwrap()
            .set(SimpleContainer { contents: 12 });
        assert_eq!(all_wrapped_up.get(0).unwrap().get().contents, 12);
    }

    #[test]
    fn double_cell_works() {
        use std::cell::Cell;

        let some_containers = vec![
            Cell::new(SimpleContainer { contents: 3 }),
            Cell::new(SimpleContainer { contents: 5 }),
        ];

        double_cell(&some_containers);
        assert_eq!(some_containers.get(0).unwrap().get().contents, 6);
        assert_eq!(some_containers.get(1).unwrap().get().contents, 10);
    }

    #[test]
    fn add_to_each_string_works() {
        use std::cell::RefCell;

        let contained_strings = vec![
            RefCell::new(SimpleNonCopyable {
                contents: "Boom".to_string(),
            }),
            RefCell::new(SimpleNonCopyable {
                contents: "Bang".to_string(),
            }),
        ];

        add_to_each_string(&contained_strings, " razzle");

        assert_eq!(
            contained_strings.get(0).unwrap().borrow().contents,
            String::from("Boom razzle")
        );
        assert_eq!(
            contained_strings.get(1).unwrap().borrow().contents,
            String::from("Bang razzle")
        );
    }
}
