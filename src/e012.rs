//! I'm not familiar with that expression
//!
//!   - **Date:** March 21, 2016
//!   - **Subject:** What it means to be an expression-oriented language, and
//!     how that works out in Rust.
//!   - **Audio:**
//!       + [M4A][m4a]
//!       + [MP3][mp3]
//!       + [Ogg][ogg]
//!
//! [m4a]: http://www.podtrac.com/pts/redirect.m4a/cdn.newrustacean.com/e012.m4a
//! [mp3]: http://www.podtrac.com/pts/redirect.mp3/cdn.newrustacean.com/e012.mp3
//! [ogg]: http://www.podtrac.com/pts/redirect.ogg/cdn.newrustacean.com/e012.ogg
//!
//! <audio title="e012: I'm not familiar with that expression" controls preload=metadata>
//!   <source src="http://www.podtrac.com/pts/redirect.m4a/cdn.newrustacean.com/e012.m4a">
//!   <source src="http://www.podtrac.com/pts/redirect.mp3/cdn.newrustacean.com/e012.mp3">
//!   <source src="http://www.podtrac.com/pts/redirect.ogg/cdn.newrustacean.com/e012.ogg">
//! </audio>
//!
//!
//! Notes
//! -----
//!
//! Rust is an *expression-oriented* language. What does that mean, and how does
//! it play out in Rust? We look at `if` and `match` blocks, discuss looping
//! constructs, and examine functions, and then widen out to discuss how having
//! an expression-oriented language can change the way we think about
//! programming.
//!
//!
//! Links
//! -----
//!
//!   - [Redox][l1]
//!   - [Servo alpha announcement][l2]
//!   - [WebRender][l3]
//!   - [Wired article on Dropbox][l4]
//!   - Rust documentation on expression-oriented-ness:
//!       + [Rust Book][l5]
//!       + [Rust by Example][l6]
//!       + [Rust Reference][l7]
//!   - [Removing Rust ternary][l8]
//!   - [Digits of pi necessary for astronavigation][l9]
//!
//! [l1]: http://www.redox-os.org
//! [l2]: https://groups.google.com/forum/#!topic/mozilla.dev.servo/dcrNW6389g4
//! [l3]: https://air.mozilla.org/bay-area-rust-meetup-february-2016/#@25m50s
//! [l4]: http://www.wired.com/2016/03/epic-story-dropboxs-exodus-amazon-cloud-empire/
//! [l5]: https://doc.rust-lang.org/book/functions.html#expressions-vs-statements
//! [l6]: http://rustbyexample.com/expression.html
//! [l7]: https://doc.rust-lang.org/reference.html#statements-and-expressions
//! [l8]: https://github.com/rust-lang/rust/issues/1698
//! [l9]: http://www.jpl.nasa.gov/edu/news/2016/3/16/how-many-decimals-of-pi-do-we-really-need/
//!
//! Sponsors
//! --------
//!
//!   - Aleksey Pirogov
//!   - Chris Palmer
//!   - [Derek Morr][s3]
//!   - Hamza Sheikh
//!   - Lachlan Collins
//!   - Leif Arne Storset
//!   - Luca Schmid
//!   - Micael Bergeron
//!   - Pascal
//!   - Ralph Giles ("rillian")
//!   - Ralph "FriarTech" Loizzo
//!   - reddraggone9
//!   - Ryan Ollos
//!   - [William Roe][s11]
//!
//! [s3]: https://twitter.com/derekmorr
//! [s11]: http://willroe.me
//!
//! ### Become a sponsor
//!
//!   - [Patreon](https://www.patreon.com/newrustacean)
//!   - [Venmo](https://venmo.com/chriskrycho)
//!   - [Dwolla](https://www.dwolla.com/hub/chriskrycho)
//!   - [Cash.me](https://cash.me/$chriskrycho)
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

use std::f64;

/// Functions are expressions, and so are their contents.
///
/// This is the foundation of the other items in this modules, because it is in
/// using functions as expressions that we'll be able to demonstrate clearly
/// that if blocks and match blocks are *also* expressions.
///
/// Note that we can see several things going on in the body of this function:
///
/// 1. The items within the function are expressions.
/// 2. The function *returns* the value of the final expression in the function
///    body. But a better way to think about it is to see the function as having
///    that value *itself*, because...
/// 3. The function itself *is* an expression, which is evaluated to have the
///    value of that final expression!
pub fn functions_are_expressions() -> i32 {
    // We can do as many things inside as statements as we like. Note that these
    // statements are actually still expressions, whose value is the empty tuple
    // `()`, sometimes called the "unit type".
    let a = 14;
    let b = 12;

    // Now that we're done, we can just return the desired value by making the
    // final line of the function *just* an expression, not a statement, so that
    // the evaluated value is not `()` but that of the expression (so: 26)!
    a + b
}


/// Like functions, if blocks are expressions.
///
/// This function simple doubly demonstrates this, by showing an assignment from
/// one evaluated expression, and by returning the result of another directly.
pub fn if_blocks_are_expressions() -> f64 {
    // note the terminal boolean expressions, unterminated!
    let a_bool = if "quuxy".len() > 3 {
        print!("{:}", "Totally a long word");
        true
    } else {
        print!("{:}", "Not at all long");
        false
    };

    // This wouldn't work: the compiler would report mismatched types as the
    // values of the `if` and `else` expressions.
    // let mismatched_types = if true {
    //     42
    // } else {
    //     "quuxy"
    // };

    // Here, we are returning the result of this expression, not only from the
    // if/else block, but from the whole function, because *this* expression is
    // the final, unterminated expression of a function, as above!
    if a_bool {
        42.0
    } else {
        f64::consts::PI
    }
}


#[cfg_attr(feature = "clippy", allow(dead_code))]
enum ThreeTypes {
    First,
    Second,
    Third,
}

/// Match blocks are also expressions.
///
/// This particular example demonstrates both local binding and returning a
/// match expression's value as the return value from the entire function.
pub fn match_blocks_are_expressions() -> String {
    let a_three_type = ThreeTypes::Second;
    let x = match a_three_type {
        ThreeTypes::First => 42.0,
        ThreeTypes::Second => f64::consts::PI,
        ThreeTypes::Third => f64::INFINITY
    };

    match x {
        0.0...100.0 => "Less than 100".to_string(),
        _ => "Less than 0, or more than 100".to_string(),
    }
}


/// Block blocks are also expressions!
///
/// In the trivial example below, we have a situation that doesn't come up
/// especially often (and certainly wouldn't in this trivial way), but is a real
/// option and is handy when you need it. You can create arbitrary blocks
pub fn ordinary_blocks_are_expressions() -> i32 {
    let a_block_result = {
        println!("You can evaluate *any* block, see?");
        println!("And the final expression is still returned.");
        // a_block_result will be 14
        14
    };

    // And that's what we'll return from the function, from this utterly
    // pointless block! (These kinds of things aren't always pointless, though.)
    {
        a_block_result
    }
}
