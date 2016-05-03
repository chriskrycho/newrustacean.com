//! No. more. nulls.
//!
//!   - **Date:** October 21, 2015
//!   - **Subject:** Enumerated (`enum`) types, pattern matching, and meaningful
//!     return values.
//!   - **Audio:**
//!       + [M4A][m4a]
//!       + [MP3][mp3]
//!       + [Ogg][ogg]
//!
//! [m4a]: http://www.podtrac.com/pts/redirect.m4a/cdn.newrustacean.com/e003.m4a
//! [mp3]: http://www.podtrac.com/pts/redirect.mp3/cdn.newrustacean.com/e003.mp3
//! [ogg]: http://www.podtrac.com/pts/redirect.ogg/cdn.newrustacean.com/e003.ogg
//!
//! <audio title="No. more. nulls." controls preload=metadata>
//!   <source src="http://www.podtrac.com/pts/redirect.m4a/cdn.newrustacean.com/e003.m4a">
//!   <source src="http://www.podtrac.com/pts/redirect.mp3/cdn.newrustacean.com/e003.mp3">
//!   <source src="http://www.podtrac.com/pts/redirect.ogg/cdn.newrustacean.com/e003.ogg">
//! </audio>
//!
//! # Notes
//!
//! Today's episode discusses, in order:
//!
//!   - Enumerated types, with an eye to the difference between structs and
//!     enums, and to the differences between `enum`s in C and in Rust.
//!   - Pattern matching, with a focus on using them with enumerated types and
//!     some discussion about how they differ from `switch` blocks in C-like
//!     languages.
//!   - Using the `Option` and `Result` enumerated types with pattern matching
//!     to provide meaningful returns from functions safely.
//!
//! ## Order
//! There is a specific order to the examples below, and it is *not* the
//! automatically-alphabetized order rendered by `rustdoc`. Instead, you should
//! work through in the sequence they appear in the [source][note-0]:
//!
//!  1. [RelatedishThings][note-1]
//!  2. [demonstrate_basic_enumeration][note-2]
//!  3. [demonstrate_match][note-3]
//!  4. [get_an_option][note-4]
//!  5. [demonstrate_option][note-5]
//!  6. [get_a_result][note-6]
//!  7. [demonstrate_result][note-7]
//!
//! [note-0]: /src/show_notes/src/e003.rs.html
//! [note-1]: /show_notes/e003/enum.RelatedishThings.html
//! [note-2]: /show_notes/e003/fn.demonstrate_basic_enumeration.html
//! [note-3]: /show_notes/e003/fn.demonstrate_match.html
//! [note-4]: /show_notes/e003/fn.get_an_option.html
//! [note-5]: /show_notes/e003/fn.demonstrate_option.html
//! [note-6]: /show_notes/e003/fn.get_a_result.html
//! [note-7]: /show_notes/e003/fn.demonstrate_result.html
//!
//! ## Links
//!
//!   - New Rustacean [Pull Request #1][link-1]
//!   - Work on IDE support!
//!       + [Landing page][link-2]
//!       + My chosen tool: [JetBrains/IntelliJ][link-3]
//!   - [Rustlings][link-4]
//!   - [Rust FFI Omnibus][link-5]
//!
//! [link-1]: https://github.com/chriskrycho/newrustacean.com/pull/1
//! [link-2]: https://www.rust-lang.org/ides.html
//! [link-3]: https://github.com/alexeykudinkin/intellij-rust
//! [link-4]: https://github.com/carols10cents/rustlings
//! [link-5]: http://jakegoulding.com/rust-ffi-omnibus/basics/
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


/// Just exists to be used as an element in `RelatedishThings`.
#[derive(Debug)]
pub struct PreexistingStruct {
    pub some_int: i32,
    pub some_string: String,
}


/// An enumeration can *hold* a variety of types. This one shows you a few.
///
/// Note: using an `enum` this way is actually crazy. The types should usually
/// have some relationship to each other. Here, they don't. The *only* reason I
/// have them together like this is to show you that these aren't just integers.
/// Enums in Rust can have members of any other type.
///
/// As [Rust by Example][1] [puts it][2]:
///
/// > Any variant which is valid as a `struct` is also valid as an `enum`.
///
/// What's the difference between an `enum` and `struct`? An `enum` is only ever
/// *one* of the options which comprise it, whereas a `struct` is always *all*
/// the elements which comprise it.
///
/// One enormous benefit of `enum` types is that, when they are the return value
/// of a function (as in the examples below), they *must* be handled.
///
/// [1]:http://rustbyexample.com/
/// [2]: http://rustbyexample.com/custom_types/enum.html
#[derive(Debug)]
pub enum RelatedishThings {
    /// This doesn't have a value other than being RelatedishThings::Unit.
    Unit,
    /// It could be a tuple struct, with basically any value type embedded.
    SomeName(String),
    SomeValue(i32),
    /// It can be a full-on struct-type construct.
    ComplexData {
        description: String,
        number: String,
    },
    /// And it can use other complex data types within those, of course.
    ReusedStructure(PreexistingStruct),
}


/// Shows how returning a RelatedishThings::Unit instance works.
fn get_unit() -> RelatedishThings {
    RelatedishThings::Unit
}


/// Shows how returning a RelatedishThings::SomeName instance works.
fn get_name() -> RelatedishThings {
    RelatedishThings::SomeName("New Rustacean".to_string())
}


/// Shows how returning a RelatedishThings::SomeValue instance works.
fn get_value() -> RelatedishThings {
    RelatedishThings::SomeValue(42)
}


/// Shows how returning a RelatedishThings::ComplexData instance works.
fn get_complex_data() -> RelatedishThings {
    RelatedishThings::ComplexData {
        description: "A fun podcast!".to_string(),
        number: "e003".to_string(),
    }
}


/// Shows how returning a RelatedishThings::ReusedStructure instance works.
fn get_reused_structure() -> RelatedishThings {
    // First create a struct from our (silly) PreexistingStruct type.
    let s = PreexistingStruct {
        some_int: 42,
        some_string: "Hey".to_string(),
    };

    // Then bundle that value into the returned enum type.
    RelatedishThings::ReusedStructure(s)
}


/// Shows how the result of an enum comes back as increasingly complex data.
///
/// Note that while we *could* have a numeric value associated with each
/// enumerated value (as in a C/C++ `enum`), we don't *have* to. The value of
/// an enumeration in Rust is *which enumerated value it is*, and nothing more
/// unless you specify something more.
pub fn demonstrate_basic_enumeration() {
    print!("{:?}\n{:?}\n{:?}\n{:?}\n{:?}\n",
           get_unit(),
           get_name(),
           get_value(),
           get_complex_data(),
           get_reused_structure());
}


/// Shows in a bit more detail how `match` works.
///
/// Note that the pattern matches include matching against numbers, tuple types,
/// and more. Big takeaways:
///
///   - You can match against all sorts of types.
///   - You have to match exhaustively. Handle "everything else" with `_` as the
///     sole component of the match arm.
///   - You can destructure complex types into their components. You can ignore
///     components of complex types with `_` as well.
pub fn demonstrate_match() {
    // You can match on numbers...
    let answer = 42;
    let question = match answer {
        // That includes individual numbers or ranges.
        0 => "What do you get when you divide by this? PROBLEMS.",
        1...41 => "This is all pretty normal, right?",
        42 => "Life, the universe, and everything, eh? (Canadian version)",
        // What about catching *everything else*? Use `_`.
        _ => "I've got nothing useful to say here.",
    };

    println!("I asked: '{:}'; the answer was: {:}", question, answer);

    // or letters...
    let character = 'C';
    match character {
        'A'...'B' => println!("Nope, not those letters."),
        'C' => println!("Why, yes, my name *does* start with a 'C'"),
        'D'...'z' => println!("None of those either."),
        _ => println!("That's not even a letter!"),
    }

    // or more complex types...
    let some_tuple = (3.141, 2.718);
    match some_tuple {
        // You can match on the full tuple with specific values.
        (3.141, 2.718) => println!("Got both!"),
        // You can match on only part of it; note that these match the cases where both are *not*
        // filled with the value specified in the first arm.
        (3.141, _) => println!("Got pi only."),
        (_, 2.718) => println!("Got e only."),
        // You can also destructure the elements of the tuple into values to be
        // used in the body of the expression, not just ignore them with `_`.
        (a, b) => println!("Got weird other stuff: {:}, {:}", a, b),
    }
}


/// Shows how this is used in a more meaningful context, with a standard type.
pub fn get_an_option(get_it: bool) -> Option<f64> {
    if get_it {
        // Returns an `Option` enum in the `Some` type.
        Some(3.14159)
    } else {
        // Returns an `Option` enum in the `None` type. This might look like the
        // `null` type you'd see elsewhere; it isn't. See below.
        None
    }
}


/// Shows how an option type works in practice.
pub fn demonstrate_option() {
    // Just demonstrate how it gets printed.
    let some = get_an_option(true);
    let none = get_an_option(false);
    print!("{:?}\n{:?}\n", some, none);

    // You can see, from printing the values above, that the `some` is (as you
    // would expect from seeing how enums work in general) not a plain value;
    // it's wrapped in `Some`! However, we can unwrap it to get at the actual
    // value:
    let some_value = some.unwrap();
    println!("{:?}", some_value);
}


/// Shows how to return either a meaningful result or an error as an enum.
///
/// The `Result` type is a standard pattern for handling cases where the result
/// of a given function may be an error. No more of the C/C++ pattern of passing
/// in/out arguments so that you can get a normal/error-indicating return code.
/// Instead, you just return a `Result`, and then `match` on that value to
/// handle it.
pub fn get_a_result(succeeds: bool) -> Result<f64, String> {
    if succeeds {
        Ok(2.718_281_828)
    } else {
        Err("Huh. This didn't go as planned.".to_string())
    }
}


/// Shows how a `Result` type works in practice.
///
/// A `Result` is an `enum` (which can be converted to the `Option` type), which
/// lets us hand back, and then handle, the results of a given function easily.
pub fn demonstrate_result() {

    // First of all, what is the result of this `match`? What would it be if we
    // changed the call to `get_a_result(false)` instead?
    match get_a_result(true) {
        Ok(value) => println!("The value was: {:}", value),
        Err(message) => println!("ERROR: {:}", message),
    }

    // Note that you can readily convert the `Ok` and `Err` terms in a `Result`
    // to `Option`s!
    let an_ok = get_a_result(true);
    let an_err = get_a_result(false);
    print!("{:?}\n{:?}\n", an_ok.ok(), an_err.err());

    // Let's demonstrate the flip-side (i.e. the `None` and `Some` bits). Note
    // how the methods behave: `Result::ok()` -> `Some` for the `Ok` enum and
    // `None` for the `Err` enum; and `Result::err()` does the inverse..
    let another_ok = get_a_result(true);
    let another_err = get_a_result(false);
    print!("{:?}\n{:?}\n", another_ok.err(), another_err.ok());
}
