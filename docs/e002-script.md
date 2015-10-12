e002: Something borrowed, something... moved?
=============================================

Intro (30s)
-----
Hello, I'm Chris Krycho, and this is the New Rustacean podcast---a 15--20 minute
show about learning the Rust programming language. This is episode 2: "Something
borrowed, something... moved?"

News (4m)
----

  - `rustfmt` -- a tool for formatting Rust code
      + [repo](https://github.com/nrc/rustfmt)
      + ["rustfmt-ing Rust`](http://www.ncameron.org/blog/rustfmt-ing-rust/)
      + [Reddit discussion](https://www.reddit.com/r/rust/comments/3nt2vm/rustfmting_rust_please_help_me_rustfmt_the_rust/)

  - Open RFC for incremental compilation -- valuable to decrease iteration time,
    especially on large projects (and available to other comparable languages).
    Right now, Rust compiles the entirety of a given "crate" (package)

      + [RFC issue](https://github.com/rust-lang/rfcs/pull/1298)
      + [RFC text](https://github.com/nikomatsakis/rfcs/blob/incremental-compilation/text/0000-incremental-compilation.md)

### About RFCs

Rust, like several other communities (EmberJS), discusses suggested changes to
the  language and tooling via "RFCs", i.e. "requests for comment". People submit
a PR with a detailed proposal, the tradeoffs, etc. and then the community
discusses it. Once accepted, implementation.

`struct` types (5m)
--------------

### Concepts

  - Data structures are fundamental
  - The kinds of structures you and have and what they can do empower/limit you.
      + True of language constructs in general: can you have standalone
        functions, or do they have to belong to a class (_a la_ Java, C♯, etc.)
      + Can you associate behavior with them (classes in classical languages,
        objects in e.g. JS)?
      + How do they relate to each other?
          * Classical inheritance
          * Prototypal inheritance
          * Mixins/traits
  - Two basic type constructors in Rust: `struct` and `enum`; today, focus on
    `structs` (the simpler of the two). Also tuples.
  - No inheritance (yet)

### Use

  - `struct`s have *members*
  - Members can be any type: primitives, other `struct`s, functions
  - Roughly like C++ or Java classes in many ways, but without inheritance, and
      + Rust has no header/source distinction as in C++
          * use documentation tools, etc. instead of reaidng header files. Cf.
            Python's `help` etc.
      + but implementation still separate from definition (in `impl` blocks)
  - Double colon notation for methods
  - Dot notation for member access

Borrowing and Moving (7m)
--------------------

### Concepts

  - A core problem in languages with memory management is knowing the state of
    a given piece of data
  - "Shared mutable state is the root of all evil"
  - Rust addresses via "ownership"
      + unlimited immutable references
      + max one mutable reference
  - When handing data to one function from another, the data can be *borrowed*
    or *moved*.
  - If you *borrow* data, you get a "reference".
      + Not the same as other languages---though some similarities under the
        covers
      + Means you don't *own* it, you just have access to it at some level. When
        done, control goes back to caller (owner)
  - If you *move* data, you get *ownership*.
      + You can do anything allowed by mutability rules of the type in the new
        owner.
      + On exiting the new owner's scope, the object is *dropped*---memory is
        de-allocated.
      + As a result, original owner can't access it anymore.

### Use

  - References use `&`.
  - The `*` operator dereferences, allowing you to assign new values to the
    referenced element (e.g. a new `struct` instance entirely, not just
    operating on the internals of a given `struct`).
      + But as always, only for mutable references.
      + It does a bit more than that, too; we'll come back to that in the future
        when we talk about traits.
  - Moves happen in function calls when using non-reference types
  - Can use `move` keyword to specify moves in other contexts as well -- more on
    that in a future episode (when I understand it better myself!)

It's both *like* and *unlike* C/C++/etc.

Closing (1m)
-------

### Next time

  - Another data structure type: `Enum`s!
  - The associated notion of pattern matching---one of the most powerful
    concepts in Rust.
  - Meaningful, safe return values from functions which might fail

### Follow/support

  - Follow: Tw, ADN (NR, then ck)
  - Support at Patreon
  - Show notes at NewRustacean.com, with links to content, Rust reference,
    social media, etc.

Until next time, happy coding!
