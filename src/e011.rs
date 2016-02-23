//! Once upon a type
//!
//!   - **Date:** March 6, 2016
//!   - **Subject:** Type systems: strong vs. weak, dynamic vs. static, and
//!     degrees of expressivity.
//!   - **Audio:**
//!       + [M4A][m4a]
//!       + [MP3][mp3]
//!       + [Ogg][ogg]
//!
//! [m4a]: http://www.podtrac.com/pts/redirect.m4a/cdn.newrustacean.com/e011.m4a
//! [mp3]: http://www.podtrac.com/pts/redirect.mp3/cdn.newrustacean.com/e011.mp3
//! [ogg]: http://www.podtrac.com/pts/redirect.ogg/cdn.newrustacean.com/e011.ogg
//!
//! <audio title="e011: Once Upon a Type" controls preload=metadata>
//!   <source src="http://www.podtrac.com/pts/redirect.m4a/cdn.newrustacean.com/e011.m4a">
//!   <source src="http://www.podtrac.com/pts/redirect.mp3/cdn.newrustacean.com/e011.mp3">
//!   <source src="http://www.podtrac.com/pts/redirect.ogg/cdn.newrustacean.com/e011.ogg">
//! </audio>
//!
//!
//! Notes
//! -----
//!
//! Talking about type systems! A broad and wide-ranging discussion about type
//! systems in general, with specific examples from languages like PHP,
//! JavaScript, Python, C, C++, Java, C♯, Haskell, and Rust!
//!
//!   - What is a type system?
//!   - What are the kinds of things we get out of type systems?
//!   - What are the tradeoffs with different type systems?
//!   - What is Rust's type system like?
//!   - What is especially attractive about Rust's type system?
//!
//! A comment on the C integer/character string addition example: what's
//! actually happening there is that the character string is an array "under the
//! covers," and as such has an address. C silently switches to using the memory
//! address, which is of course just an integer, when you try to add the two
//! together. As I said on the show: the result is nonsense (unless you're using
//! this as a way of operating on memory addresses), but it's compileable
//! nonsense. In a stricter and stronger type system, memory addresses and
//! normal numbers shouldn't be addable!
//!
//!
//! Links
//! -----
//!
//!   - [Rust 1.7 released][l1]
//!       + [`HashMap` changes][l2]
//!   - [Introduction to Type Theory][l3]
//!   - [Visualizing Rust's type-system][l4]
//!   - [The Many Kinds of Code Reuse in Rust][l5]
//!
//! [l1]: http://blog.rust-lang.org/2016/03/02/Rust-1.7.html
//! [l2]: http://blog.rust-lang.org/2016/03/02/Rust-1.7.html#library-stabilizations
//! [l3]: http://www.cs.ru.nl/~herman/PUBS/IntroTT-improved.pdf
//! [l4]: https://jadpole.github.io/rust/type-system
//! [l5]: http://cglab.ca/~abeinges/blah/rust-reuse-and-recycle/
//!
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

use std::fmt;


/// Is the thing "expressive", whatever that means?
pub enum Expressive {
    Ridiculously,
    PrettyDarn,
    Fairly,
    SortOf,
    Barely,
    NotEvenALittle,
}

impl fmt::Display for Expressive {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let description = match *self {
            Expressive::Ridiculously => "ridiculously",
            Expressive::PrettyDarn => "pretty darn",
            Expressive::Fairly => "fairly",
            Expressive::SortOf => "sort of",
            Expressive::Barely => "barely",
            Expressive::NotEvenALittle => "not even a little",
        };

        write!(f, "{} expressive", description)
    }
}

/// Is the thing *strong*?
pub enum Strong {
    Indeed,
    ABit,
    NotEspecially,
    NopeNopeNope,
}

impl fmt::Display for Strong {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let description = match *self {
            Strong::Indeed => "strong indeed",
            Strong::ABit => "a bit strong",
            Strong::NotEspecially => "not especially strong",
            Strong::NopeNopeNope => "strong? NOPE NOPE NOPE",
        };

        write!(f, "{}", description)
    }
}

/// Is the thing statically known?
pub enum StaticallyKnown {
    Yeah,
    Nope,
}

impl fmt::Display for StaticallyKnown {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let description = match *self {
            StaticallyKnown::Yeah => "it's totally known at compile time",
            StaticallyKnown::Nope => "we don't know anything about it until run time",
        };

        write!(f, "{}", description)
    }
}


/// Look, we composed those enums into another type. How... droll.
pub struct TypeSystem {
    name: String,
    expressive: Expressive,
    strong: Strong,
    statically_known: StaticallyKnown,
}


impl TypeSystem {
    pub fn builder(name: &str,
                   expressive: Expressive,
                   strong: Strong,
                   statically_known: StaticallyKnown)
                   -> TypeSystem {

        TypeSystem {
            name: name.to_string(),
            expressive: expressive,
            strong: strong,
            statically_known: statically_known,
        }
    }
}


/// An incredibly contrived function which just shows enums at work.
///
/// I wanted there to be *some* sample code this episode, you know? This one
/// just assembles the types into a vector and prints them all. It does,
/// however, let us refresh ourselves on a bunch of other concepts we've covered
/// on the show---pretty much everything but generics makes an appearance in
/// some way in this module.
pub fn describe_type_systems() {
    let js = TypeSystem::builder("ECMAScript",
                                 Expressive::Fairly,
                                 Strong::NopeNopeNope,
                                 StaticallyKnown::Nope);

    let php = TypeSystem::builder("PHP",
                                  Expressive::Barely,
                                  Strong::NopeNopeNope,
                                  StaticallyKnown::Nope);

    let c = TypeSystem::builder("C",
                                Expressive::Barely,
                                Strong::NotEspecially,
                                StaticallyKnown::Yeah);

    let cpp = TypeSystem::builder("C++",
                                  Expressive::SortOf,
                                  Strong::ABit,
                                  StaticallyKnown::Yeah);

    let java = TypeSystem::builder("Java",
                                   Expressive::SortOf,
                                   Strong::ABit,
                                   StaticallyKnown::Yeah);

    let csharp = TypeSystem::builder("C♯",
                                     Expressive::Fairly,
                                     Strong::Indeed,
                                     StaticallyKnown::Yeah);

    let swift = TypeSystem::builder("Swift",
                                    Expressive::PrettyDarn,
                                    Strong::Indeed,
                                    StaticallyKnown::Yeah);

    let rust = TypeSystem::builder("Rust",
                                   Expressive::PrettyDarn,
                                   Strong::Indeed,
                                   StaticallyKnown::Yeah);

    let haskell = TypeSystem::builder("Haskell",
                                      Expressive::Ridiculously,
                                      Strong::Indeed,
                                      StaticallyKnown::Yeah);

    let langs = vec![js, php, c, cpp, java, csharp, swift, rust, haskell];
    for lang in langs {
        println!("{language} is {expressive}, {strong}, and {statically_known}",
                 language = lang.name,
                 expressive = lang.expressive,
                 strong = lang.strong,
                 statically_known = lang.statically_known);
    }
}
