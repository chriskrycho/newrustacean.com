# CYSK: `bindgen` and `cbindgen`

## Intro

Hello, I’m Chris Krycho and this is New Rustacean: a show about the Rust programming language and the people who use it. This is Crates You Should Know: bindgen and cbindgen.

## [Sponsor: Parity](https://parity.io/jobs)

Before we dive in, Parity is back sponsoring the show—because they want to hire you to come work in Rust with them!

Parity is advancing the state of the art in decentralized technology, and they’re using Rust to do it, leaning hard on its trifecta of performance, reliability, and productivity. They're building cutting-edge tech in areas like WebAssembly and peer-to-peer networking. Two of the larger projects they're working on are: Substrate, a framework for building blockchains, and Polkadot, a platform leveraging blockchain tech for scaling and interoperability in decentralized systems.

If that sounds interesting, check out their jobs at <parity.io/jobs>!

## Motivation

Over the last few episodes we talked a great deal about <i>foreign function interfaces</i>, and I walked through how you can explicitly define and build those out yourself. However, as you can imagine, doing that for every single function and type you deal with in a large project with lots of FFI is… painful and error-prone. Happily, the work of mapping C interfaces to Rust interfaces can, by and large, be automated—with [the `bindgen` crate][bindgen source]. And, similarly, we can automate the work mapping Rust interfaces to C interfaces with [the `cbindgen` crate][cbindgen source]. Today, we’re going to dive into how each of them works!

[bindgen source]: https://rust-lang.github.io/rust-bindgen/
[cbindgen source]: https://github.com/eqrion/cbindgen/

## [`bindgen`][bindgen source]

`bindgen` is an *official* Rust tool, but it is not part of the standard library. That’s because it’s not needed for *most* projects in Rust, and it’s really a library for your *build* process rather than for your crate proper. It’s a tool for *generating bindings* from Rust to C and *some* C++ libraries. If the thought of writing out bindings by hand and hoping you get them exactly right every time seemed a bit intimidating… well, you’re not alone in that; I feel much the same!

bindgen only generates Rust bindings for C code. It’s designed to cover the half of the story that’s usually most important for integrating Rust into an existing codebase, where you want to be able to add Rust alongside C or C++. For example: this is exactly what Mozilla needed when they started pulling Rust into the Firefox codebase, which is pretty much *entirely* C++.

`bindgen` supports two ways of using it: as a command line tool, and as a part of a `build.rs` script. The latter is the recommended path, and it gives you a great deal of control over exactly how you use it. In particular, it lets you account for architecture-specific bits dynamically—which is often important for cross-platform Rust code which is interacting with cross-platform C or C++ code! If you follow that recommendation, you don’t need separate binding files for every target you build against. However, your consumers *will* need a copy of the clang compiler to build your code in that case. Happily, it’s fairly easy to get an up-to-date version of clang on every major platform, including for multiple different Linux distros.

As with build-file dependencies in general, you’ll put `bindgen` in your `[build-dependencies]` section of your `Cargo.toml` file. Then you need to create an *entry point* for the library to do its work: a single `.h` header file which uses the `#include` syntax to pull in every item declaration you need bindings for, by including *other* `.h` header files from the project you’re working with. This file has one other purpose, but we’ll come back to it in a minute. Once you have included all your requisite dependencies, you’ll go into your `build.rs` file and add a `use bindgen;` statement. Then the simplest thing you can do with its exported `Bindgen` type is create a builder and call its `header` method with the path to that `.h` file you created as its argument, and then call its `.generate` method. This will do the work of attempting to parse that header file and all the files *it* includes, and to generate a data structure which can be written as Rust type definitions of the sort we talked through in detail in episodes 029 and 031. You can write that out using the `.write_to_file` method on the generated bindings item, also just passing it a standard file path.

### Customizing bindgen

There are a number of ways we can customize the behavior beyond the defaults, though, which is helpful for situations where the defaults aren’t quite what you need.

#### Customizing bindgen: enums

You can map C/C++ enums into Rust in a variety of different ways:

-   The first is as a set of constants—optionally namespaced into a module—with each Rust constant having the integer value corresponding to the enum literal value on the C side

-   The second is as Rust enums, with guarantees that the discriminants match across the languages. This sounds like what you always want, right? But it’s only what you always want if you’re in control of the C side, because otherwise you can cause undefined behavior with it! The docs link to an explanation of *why*; the point is that, as always with FFI, things are more complicated than they might seem at first.

-   The third is as bitfields, a densely packed layout designed to save space and deal with byte alignment issues—the kinds of concerns that come up when you’re dealing with registers on embedded systems, or sometimes when dealing with message protocols

#### Customizing bindgen: regular types

You can also extensively customize how bindgen handles different types.

For one thing, you can set up a whitelist or a blacklist to determine which types get exposed in the first place. It may well be that you only want to pull in a small subset of the items defined across a bunch of different header files you’re including, but the C side doesn’t really give you that kind of control. When you `#include` a file, it’s literally just dumping the contents of that file in as a string. (I have… stories… about this behavior; if you ever happen to see me at a conference or a meetup ask me and I will happily regale you with the horrors I have seen.) You can use the `bindgen::Builder` whitelist family of methods—`whitelist_type`, `whitelist_function`, and `whitelist_var`—to tell bindgen to allow *only* a subset of items into the generated Rust definitions.

You can similarly blacklist *types* from appearing with the `blacklist_type` function. However, this is only for when you need to provide your own handwritten definition of the type, because it’ll still show up in *other* functions and types, and so you have to provide a definition for the type or your crate won’t compile! In these cases, your best bet is telling bindgen to generate a type that is opaque to Rust—the inverse of the opaque type scenario I talked it great detail in e031. Here, you use the `bindgen::Builder`’s `opaque_type` helper to make it generate a type for which Rust only knows the size and alignment.

You can also replace one C or C++ type with a different C or C++ type entirely entirely—one that bindgen can do the right thing with, when the original is too complicated for bindgen to get right. Servo has to do this at times with C++ types, but I’m hard-pressed to imagine it ever coming up in plain-old C code. (Of course, I could be failing to think of a weird edge case: C has a few of those!)

Finally, for our purposes, you can customize what traits do and don’t get derived in *great* detail. That means you can opt out of `Copy` and `Clone`—which bindgen *tries* to do by default—or into other traits like `Debug` or `Default` or `Hash` or `Eq`, all using other methods off of `bindgen::Builder`.

Now, I’ve covered this at a high level, but there are *many* more settings you can tweak and customize along the way. There are two further resources you’ll find very helpful in digging further into `bindgen`: [the official guide][bindgen guide], and the [API docs][bindgen API docs]. Give them a read (and check out the very simple example in the show notes!).

[bindgen guide]: https://rust-lang.github.io/rust-bindgen/
[bindgen API docs]: https://docs.rs/bindgen

## [`cbindgen`][cbindgen source]

So that does it for `bindgen`, but what about going the other direction—generating C types from Rust, say, to be consumed by Swift? For that, we’ll use `cbindgen`.

`cbindgen` is *not* an official Rust tool, but that doesn’t mean you shouldn’t use it. Its design mirrors that of `bindgen`, so at a high level the *workflow* is the same as with `bindgen`, except that we’re going the other direction. `cbindgen` intentionally mirrors the API of `bindgen`. You can use it either from the command line or from a `build.rs` build script, and its API mirrors `bindgen`’s quite closely. You pull in the library, you create a `Builder`, you pass it a path to a crate directory, you call `generate` on it, and you write the fill out—but this time, instead of writing a Rust file, you’re writing out a `.h` header file.

It’s worth noting, however, that `cbindgen` has some important limitations and gotachs. Specifically (and here I’m cribbing from [a blog post by one of the main authors of the crate][future-directions]:

[future-directions]: https://blog.eqrion.net/future-directions-for-cbindgen/

- It has a hard time dealing with path resolution, so if you have multiple item which would resolve to the same external name, it’ll fall over

- It doesn’t understand privacy modifiers, and between that and its not understanding paths, it doesn’t actually know whether something should be exported from your crate or not. That’s a big deal! It *guesses*: if an item is `pub`, it thinks it’s exported. As we talked about in detail in e030, though, that’s not correct.

- Because of where it interacts with Rust code—i.e. at the level of the uncompiled syntax—it doesn’t understand macros, which are a form of code generation. There’s an option to work around this using a Rust compiler flat… but it’s not guaranteed to work properly with macro hygiene, so it’s not stable.

- It similarly doesn’t always know what to do with `#[cfg]` attributes. It tries! …but it can’t always do the right thing, where by “the right thing” I mean “the thing you want” or “the thing you find unsurprising.”

The net of all of those is that basically the maintainers are confronted with the problem of trying to reimplement non-trivial subsets of `rustc`! So cbindgen *does* work today, and you can use it in exactly the ways I talked through above, but you should be aware of those caveats… and you should keep your eyes on a *follow-on* project, [`rust-ffi`], which aims to address those with a more robust architecture that splits apart some of the concerns and runs as a Rust compiler plugin for the bits that need to understand Rust itself correctly. (I’m not sure how alive that project is: it was a proof of concept, but it seems promising; hopefully it gets some forward motion soon!)

[`rust-ffi`]: https://github.com/eqrion/rust-ffi

## Binding libraries for other languages

The other thing worth calling out here is that there are dedicated libraries for providing Rust bindings for most of the major scripting languages. I’ll name them here, and link them in the show notes—in many cases, those are *the* place you should start if you’re wanting to augment code written one of those languages with a native extension in Rust.

- Python has [PyO3](https://github.com/PyO3/PyO3)
- Elixir and Erlang have [Rustler](https://github.com/rusterlium/rustler)
- JavaScript has [Neon](https://neon-bindings.com)
- Ruby has [Helix](https://github.com/tildeio/helix)

Those are in various degrees of maturity and stability, but all are at least *usable* and indeed all of them are actually being used in production. There was [a great write-up from the folks at Discord][discord-elixir-rust] just today (as I write and record) about how they’re using Rustler to speed up a critical part of their Elixir back end (linked in the show notes, of course!).

[discord-elixir-rust]: https://blog.discordapp.com/using-rust-to-scale-elixir-for-11-million-concurrent-users-c6f19fc029d3

## Testing

The last thing to say is that *you should test your bindings*. It’s one thing to look at code and be pretty sure you wrote the right things. It’s something else entirely to have tests in place that make *sure* you do. Use Rust’s built-in testing infrastructure to check on a couple things:

1.  Do a simple “is this just basically right?” test. Does invoking each function you’re importing via FFI give you the right results in the happy path? Testing *more* than the happy path is probably good, but at a *minimum* you want to check this part.

2.  Do you end up with any panics because you have invalid data flowing across the FFI boundary? To manage this you may need to write some unsafe code; but it’s a good way to make sure that your invariant-checking pieces are correct. Assert that things *should* panic when given bad data, for example, using the `#[should_panic]` attribute on a test function. Or get more fine-grained using the `std::panic::catch_unwind` function to get a `Result` with the error and dig into its details.

I expect this is a place where some kind of fuzz- or property-based-testing could also be useful… but I’ll freely admit that that’s just speculation, because I haven’t ever actually been able to give those a whirl myself. I’d love to hear if you’ve put those kinds of tools to use in FFI contexts in particular!

## Conclusion

Thanks as always to this month’s $10-or-more sponsors:

- Raph Levien
- Martin Heuschober
- Graham Wihlidal
- Jason Bowen
- Soren Bramer Schmidt
- Daniel Collin
- Peter Tillemans
- Oluseyi Sonaiya
- Behnam Esfahbod
- Nicolas Pochet
- Jonathan Knapp
- Dominic Cooney
- Cristian Paul
- Dan Abrams
- Nathan Sculli
- James Hagans II
- Chip
- Anthony Deschamps
- Andrew Dirksen
- Embark Studios
- Paul Naranja
- Ryan Osial
- Benjamin Manns
- Michael Mc Donnell
- Daniel Mason
- Bryan Stitt
- Alexander Payne
- Nick Gideo
- David Carroll
- Daniel Bornkessel (April only)
- John Rudnick
- Rob Tsuk
- Nick Stevens
- Arun Kulshreshtha
- Adam Green
- Jako Danar
- Jeff May
- Zach Peters
- Scott Moeller
- Evan Stoll
- Joseph Schrag
- Matt Rudder
- Ramon Buckland
- Johan Andersson
- Brian McCallister
- Jerome Froelich
- Ola Fadeyi

You can find show notes for this episode and every other at <newrustacean.com>—with scripts, code samples, and a number of interview transcripts.

Please recommend the show to others if you like it, whether in person, via your podcast directory, or in various media online! You can contact me at @chriskrycho or @newrustacean on Twitter, or by sending men an email at hello@newrustacean.com.

Until next time, happy coding!