# News: Rust 1.35

## Intro

Hello, I’m Chris Krycho and this is New Rustacean: a show about the Rust programming language and the people who use it. This is a news episode for Rust 1.35.

## Sponsor: Parity

Before we dive in, Parity is back sponsoring the show—because they want to hire you to come work in Rust with them!

Parity is advancing the state of the art in decentralized technology, and they’re using Rust to do it, leaning hard on its trifecta of performance, reliability, and productivity. They're building cutting-edge tech in areas like WebAssembly and peer-to-peer networking. Two of the larger projects they're working on are: Substrate, a framework for building blockchains, and Polkadot, a platform leveraging blockchain tech for scaling and interoperability in decentralized systems.

If that sounds interesting, check out their jobs at <parity.io/jobs>!

## 4 years since 1.0

The first bit of news I want to touch on—and I’ll have a little more to say about this in the *next* episode—is that two weeks ago was the fourth anniversary of Rust’s hitting 1.0! It’s kind of amazing that it’s been that long, and an incredible amount has happened along the way. If you go back and look at what Rust 1.0 looked like—the state of the ecosystem, the state of the standard library, even the language itself—it’s an amazing change over that period of time.

## Rust 1.34 point releases

Next, I thought it important to mention two point releases in the 1.34 cycle:

- **1.34.1** fixed two issues in Clippy. As you may recall from my comments in my discussion of Rust 1.31 and the 2018 Edition, Clippy is now an official Rust tool with the same Semantic Versioning compatibility guarantees as Rust itself. Unfortunately, 1.34.0 saw a few bugs make it into Clippy—two false positives and one panic when checking macro behaviors. Not a *huge* deal, but they shipped a fix for it anyway, which I appreciate!

- **1.34.2** was substantially more serious: it was a fix for a security issue. You may recall from my discussion of Rust 1.34.0 that there was a newly-stabilized `Error:type_id` method. A careful Rustacean, Sean MacArthur, identified that this exposed a security hole. The *default* implementation was safe… but it was possible to return the wrong type when implementing the method yourself. And if you did that, you’d have a situation where Rust would treat the item as the wrong type, which can lead to very serious memory unsafety. For example: if the type was actually a `u8` and your returned a `Vec<u8>` instead, Rust would think you had a valid `Vec`… and if you started performing operations on it, like calling `my_vec.push(42)`, well, now you’d be scribbling over memory that didn’t belong to you! 1.34.2 therefore *destabilized* the feature entirely. There’s [an ongoing discussion][Issue #60784] about what to do with the feature, since it has existed since 1.0… just on nightly… and has been unsafe that whole time. (It’s just that now is the first time somebody caught the unsafety!)

[Issue #60784]: https://github.com/rust-lang/rust/issues/60784 

## Rust 1.35

All right, so let’s dig into what’s new in Rust in 1.35! This is a fairly small release… which is actually *normal* for Rust, after all! It’s just that the past several months have been jam-packed with the lead-up to the 2018 Edition and a number of other major features landing. 1.35 isn’t one of those release… but it’s a good one nonetheless!

I talked in episode e031 about how you can expose Rust libraries as dynamically-linkable, C ABI-exposing libraries for other tools to use. Rust 1.35 adds a flag to Cargo, `rustc-dylib-link-arg`, to make it easier to get some of the nitty-gritty details of building a library like that right: specifically, things like setting the version for the shared library or the runtime path for where to find it. Not a thing you’re going to do most of the time, but if you’re deep in the kinds of weeds we were talking about and need to build a dynamic library on Linux or macOS… you may just be happy this exists!

There are a few new targets to build against… most interestingly, including a `wasm32-unknown-wasi` target. [WASI] is the recently proposed and rapidly developing WebAssembly System Interface. WASI is one of several recently-announced pieces of the WebAssembly story which start to capitalize on the promise for WebAssembly to be not just a way of targeting browsers for high-performance code, but a useful, safe (because it’s sandboxed!) cross-platform compilation target… for lots of languages, though of course at the moment Rust is really the language driving the effort forward. WASI supplies an API to do *systems programming*: sockets, file system operations, time, you name it… but with all the safety constraints (sandboxing, for example) that are inherent in the WebAssembly programming model.

It’s early, but it’s possible that this (along with a number of other efforts) will represent a sea change in the safety and reliability of systems programming… which we desperately need! If you’re interested in hearing more about WASI, I’ve linked to [an episode of the really excellent podcast The Bike Shed][bikeshed #195] in the show notes, where host Chris Toomey talked with Lin Clark and Till Schneidereit from Mozilla about a *bunch* of things, including WASI. And as of Rust 1.35, Rust can now target it natively (on nightly)! Now, there’s a lot missing in the standard library when targeting WASI, as you’d expect from something this early on—WASI itself is immature enough that everyone is figuring out the best way for the implementations to work! I find this incredibly cool nonetheless, and will be watching it very closely.

[WASI]: https://github.com/CraneStation/wasmtime/blob/master/docs/WASI-intro.md
[bikeshed #195]: http://bikeshed.fm/195

In terms of language features and library stabilizations, there are a handful of nice improvements:

- The `dbg!` macro can now be called with no args at all. “But what would that even print out,” you ask? “The file and line where you put it,” I say! A handy tool in the debug-printing toolbox. One place you might use this: watching how multithreaded or asynchronous operations interleave.

- The expanded `Debug` format—which I always forget about; you type it `{:#?}`, and it’s what the `dbg!` macro uses—now includes a comma after the final struct field… to make it match the idiomatic output in modern Rust. A hilariously tiny change, and I kind of love it.

- `Option` got a `copied` method, which copies an `Option<&T>` to an `Option<T>`. As you’d expect, `T` here has to implement `Copy` for this to work; it’s a nice little ergonomics win when it’s available, though, and there’s no particularly nice way to write it in Rust without this. You’d have to do something like:

    ```rust
    let copied = some_option.map(|val| {
        let new_val = val;
        new_val
    });
    ```

    Now you can just write:

    ```rust
    let copied = some_option.copied();
    ```

    Handy!

As always, of course, you should read [the full release notes][1.35], because there are other pieces that landed here, some of which may be particularly interesting to you!

[1.35]: https://github.com/rust-lang/rust/blob/master/RELEASES.md#version-1350-2019-05-23

## Final `async`/`await` syntax

Before we wrap up today, I also want to talk about the final `async`/`await` syntax, which *should* be stabilizing in Rust 1.37. The development of the primitives for ergonomic, asynchronous code in Rust has been a long, slow path—which is appropriate in a lot of ways; getting this right in a way that maps to Rust’s commitment to zero-cost abstractions *and* usability has been an incredibly hard task. If I’m honest, I still don’t have my head all the way around the features here, and I think it’ll take me some serious digging before I do!

The gist is this, though: there is a *library*, `futures`, which provides a number of types and functions in support of them for asynchronous programming—from the eponymous `Future` type, a single value which eventually resolves (or not!), very much like a `Promise` in JavaScript or a C# `Task`; to stream types, which represent a *series* of values over time; to tools for asynchronous I/O or asynchronous execution of operations. You can check out [the API docs][futures-rs] for details

[futures-rs]: https://rust-lang-nursery.github.io/futures-api-docs/0.3.0-alpha.16/futures/

All of this was developed against nightly Rust over the past several years—and I do mean *several years*; the earliest phases of this work began in 2016! For much of that time, the work has been done using *macros*: the `#[async]` attribute to specify when a given function has async behavior happening in it and to invoke an asynchronous function with the `await!` macro-rules style macro. However, now that the story around the library and the design of its integration into Rust as a core primitive is stabilizing, it’s time to land a dedicated syntax. There’s been an *enormous* amount of discussion about this—an amount I’ve found exhausting as a casual observer; I can’t imagine how taxing it must have been for everyone involved. It turns out that language syntax provokes strong feelings from people. Who’d have thought, right?

You may be wondering, even before we get into what the dedicated syntax *is*, why we need it, given that we’ve gotten by through the whole design period with macros thus far. The basic answer is: tooling! While macros can be made to work, and fairly well, it is difficult to get them to provide good diagnostic feedback during the compiler operations. This means that the experience of working with asynchronous code using the macros always feels second class. You’ve probably run into this yourself if you’ve used crates which are heavy on macros: even when the authors are doing a really good job, it can be much harder to figure out why something isn’t lining up—*which* types are wrong, and *how*, for example—than when you’re *not* using macros. For a feature as core to the future of the language as async and await, it’s important that make these things genuinely first class.

Let’s start with `async` functions: you write them `async fn` with a normal function signature and—and this is the important bit—the return type as if it were *not* a normal function signature. So if you have a function which will ultimately resolve as a type which implements the `std::future::Future` trait, with the associated `Output` type being a `Result<i32, String>`, you’d write:

```rust
async fn my_neat_thing() -> Result<i32, String> {
    // and then the body of the function
}
```

One reason for *this* choice is that it’s often *very* difficult—borderline impossible—to write out the fully resolved type of some complicated chain of futures. While futures are *like* JavaScript promises in some ways, Rust’s commitment to zero-cost abstractions means the language requires the type system to do a *lot* more work, rather than just throwing more memory allocation at it via allocations (which is basically how JavaScript handles it). Using the `async` keyword for this transformation means that you as an author can see that it’s a function which has this special asynchronous handling behavior, and see what it will resolve to, without needing to figure out how to write that complicated chain of types yourself.

The other half of this is the `await` keyword. It looks almost certain to end up in what looks like <i>field access position</i>. That is, if you have some method which returns a future, you’d indicate that you’re then awaiting its result (not immediately chaining it into a new future via some combinator or other) by writing `.await` after the invocation. So you might have a whole function which looks something like:

```rust
async fn neato() -> Result<Vec<i32>, String> {
    let doubled = get_some_i32s()
	    .await?
	    .iter()
	    .map(|i| i * 2)
	    .collect();
    Ok(values)
}
```

That invokes `some_other_async_fn_with_i32_in_a_result`, awaits the result with the `.await`, invokes the `?` operator to early-return if it’s an `Err`, then maps over the `Vec<i32>` which that function handed back, and returns a collected version of that as an `Ok` variant. Note that it *is* still returning a future in the end, and the compiler will be able to give us appropriate feedback about the types along the way… but it is a smooth experience.

I was a bit surprised initially by the choice of this position, instead of the `await some_other_async_fn_with_i32_in_a_result()` you’d expect coming from JavaScript or C♯. However, I came around fairly quickly when folks pointed out that other languages which use that position are *statement*-oriented. Rust builds everything out of expressions—it’s one of the things I love about it—and encourages chaining those expressions together. Idiomatic Rust also uses the `?` operator for short-circuiting when there’s an early error a *lot*. The combination of those two factors means you’d end up writing something like `(await some_fn())?` a *lot* unless there were a special case rule around the combination of `await` and `?`, and that would be both pretty surprising and also have some strange side effects. As much as it initially seemed weird, I’ve already gotten pretty used to it and it makes a lot of sense. (Also, it’s far from the only thing that happens in field access position which is *very* syntax sugary. Even method invocation does some pretty fancy things there to get `self` as the right kind of thing… for example, when you take `Box<&mut self>`, which you can do!)

There’s a *lot* more discussion around this, which I’ve [linked][so-long-omg] in the show notes, and it’s technically getting finalized in today’s language team meeting, so this *might* change… but it looks like this is how it’s going to land.

[so-long-omg]: https://internals.rust-lang.org/t/a-final-proposal-for-await-syntax/10021

I’m excited by this, in any case. In just a few weeks, we’ll be able to write async/await code on stable Rust, and that will, I think, be another big leap forward in the places where Rust is a viable and indeed *excellent* choice.

## Outro

And that’s a wrap on this news episode. The next episode will be out on Monday… and it’s a special one. Keep your eyes on your podcast feeds!

### Patreon Sponsors

Thanks as always to this month’s $10-or-more sponsors:

- Arun Kulshreshtha
- Matt Rudder
- Soren Bramer Schmidt
- Dan Abrams
- Oluseyi Sonaiya
- Anthony Deschamps
- Evan Stoll
- Nick Gideo
- Dominic Cooney
- Embark Studios
- Scott Moeller
- Benjamin Manns
- Daniel Mason
- Jonathan Knapp
- Nick Stevens
- Jeff May
- Behnam Esfahbod
- Johan Andersson
- Nathan Sculli
- James Hagans II
- John Rudnick
- Zach Peters
- Chip
- Jerome Froelich
- Andrew Dirksen
- Joseph Schrag
- Brian McCallister
- Bryan Stitt
- Raph Levien
- Nicolas Pochet
- Daniel Bornkessel (April only)
- Ryan Osial
- Jason Bowen
- Jako Danar
- Michael Mc Donnell
- Adam Green
- Alexander Payne
- Rob Tsuk
- David Carroll
- Ramon Buckland
- Martin Heuschober
- Peter Tillemans
- Paul Naranja
- Graham Wihlidal
- Ola Fadeyi
- Cristian Paul
- Daniel Collin

### Show info

You can sponsor the show at patreon.com/newrustacean or via other services listed on the show website, <newrustacean.com>. There, you’ll also find show notes, including links to things I talk about, scripts, code samples, and interview transcripts. The notes for *this* episode are at <newrustacean.com/show_notes/cysk/bindgen/>.

Please recommend the show to others if you like it, whether in person, via your podcast directory, or in various media online! You can contact me at @chriskrycho or @newrustacean on Twitter, or by sending men an email at hello@newrustacean.com.

Until next time, happy coding!