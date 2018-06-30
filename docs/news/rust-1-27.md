# Rust 1.27

Hello, I’m Chris Krycho and this is New Rustacean, a show about the Rust Programming Language and the people who use it. This is a news episode for Rust 1.27.

## Rust 1.27

Rust 1.27 is another big release – if not quite as big as 1.26 was, still very significant.

### Language stabilizations

Let’s start by digging into the stabilizations for the language itself.

#### SIMD

The first big new stable feature is one I already discussed briefly in [the news episode for Rust 1.25](https://newrustacean.com/show_notes/news/rust_1_25/): SIMD coming to stable Rust. Back in Rust 1.25, the old feature flags were deprecated, in preparation for stable SIMD landing – and the first step of that happens *now* with Rust 1.27. For a basic intro to SIMD itself, you can go back to the 1.25 episode, and for the *full* stabilization plan, you can look at [RFC #2325](https://github.com/rust-lang/rfcs/blob/master/text/2325-stable-simd.md) for the master plan. Here I’m focusing on what actually landed in Rust 1.27, which is a *subset* of the full available set of features SIMD exposes.

The Rust compiler, via LLVM, already does a lot of the ‘vectorization’ that SIMD gives you – the parallelization of the process across all available processing units, because everything in the *vector* in question can safely be processed in parallel. (Note that “vector” is used here in the fundamental sense, *not* in the Rust data type sense, though Rust’s `Vec` type is certainly a candidate for this kind of parallelization.) However, rustc and LLVM cannot *always* tell that something is safe to vectorize, so some functions, loops, etc. which *could* be vectorized *aren’t*.

The newly stabilized `std::arch` standard library module exposes primitives which let you access the “SIMD intrinsics” for a number of platforms. An “intrinsic” is a function that is *intrinsic*, or built into, to the language or compiler. In this case, the intrinsics are actually built into the *architecture*, all the way down at the processor level; they’re *intrinsic* to the CPU.

You can use the currently-stabilized set of SIMD intrinsics with the `#[cfg(...)]` attribute applied to `use` statements to get specific versions of a given function at compile time, or on the declaration of a function to choose at runtime for when a given architecture may or may not support the intrinsics in question depending on the age of the machine in question (like the x86 line). The module also exposes a number of convenience helpers, like the `is_<arch>_feature_detected!` macro, which lets you generate code with fallback for when you want to use a feature if it’s available on your architecture.

The upshot of all of this, for *most* Rust users, is that a bunch of the libraries you use are going to get faster. You can use these yourself where it makes sense, of course, but it’s not a most-programs-most-of-the-time concern. At the same time, there are plans to expose higher-level APIs on top of the low-level primitives now available in the `std::arch` module in the future, which should make it that much easier to use these for average developers. And those kinds of changes have the possibility to make it relatively easy for Rust to go even faster than it does today.

#### `dyn Trait`

The other big feature, `dyn Trait`, is a complement to the `impl Trait` feature released in 1.26. `dyn Trait` is the official replacement for the bare `Trait` syntax. Wherever you might have written something like `Box<Iterator>` or `&Iterator` in the past, now you’d write `Box<dyn Iterator>` or `&dyn Iterator` instead.

The motivation for this is two-fold: one is that, in combination with `impl Trait` it makes static vs. dynamic dispatch both *explicit* and *symmetric*. That is, you can always tell when looking at a given reference to a trait whether it’ll be *dynamically* dispatched (that is, at runtime), or *statically* dispatched (that is, “monomorphized” into distinct functions at compile time). `dyn Trait` will be *dynamically* dispatched – thus the name! – and `impl Trait` will be statically dispatched.

The fact that it’s symmetric is nice – both forms have a short keyword in front of the trait – but the fact that it’s *explicit* takes us to the second reason: the bare trait version we had historically was something of a footgun for people – a place where it was easy to shoot yourself in the foot because you didn’t realize the consequences of what you might normally write. `Box<Something>` or `&Something` are ambiguous, and their performance characteristics are ambiguous as well: they could refer to a pointer to a normal type (a `struct` or `enum`), or they could refer to a *trait object*.

We’ll talk more about trait objects and the related concept of object safety in the upcoming Traits Deep Dive, Part 3 episode. Here, it’s enough to note that that ambiguity – is this a normal reference or a trait object? And should I write `impl SomeTrait for SomeOtherTrait`, or should I be writing `impl<T> SomeTrait for T where T: SomeOtherTrait`? Because those are two different things! One is implementing things for a trait object, the other for a constrained generic, but the *shorter* one that you’re more likely to write is usually *not* what you want (because you more often want the generic, not the trait object).

There’s a lot more to say here, which is why there’s another episode coming up to cover it in more detail. The takeaway for today on this particular change is that anywhere you had a bare trait name in argument or return position before, you now need to change it to be `dyn Trait` instead – at least, assuming you want to keep the same dynamic-dispatch dynamics. Because you now have both `dyn Trait` and `impl Trait` at your disposal, though, it’s much easier to switch between the two as makes sense in the specific context of your program.

#### Attributes updates

The last couple language changes of interest that landed in 1.27 were all to do with *attributes*. The first of these is that you can now put the `#[must_use]` attribute on any function – not just any type. With this change, you can require-via-lint that the result of your function be checked even when you might not want to require that for every use of the type (or, if it’s a third-party type, might not be *able* to). The other is that you can now put attributes on generic parameters like lifetimes and types. This is a fairly obscure feature for the moment, but it’s likely to be useful for procedural macros, which are the main place you see heavy use of attributes anyway.

### Standard library stabilizations

Now let’s turn our attention to standard library stabilizations. The biggest of these in 1.27 is `std:arch`, which we already talked about: it’s the module which exposes the SIMD tooling! Others were more the normal gamut of small-niceties. A few that caught my eye – you should see the [full release notes](https://github.com/rust-lang/rust/blob/master/RELEASES.md#version-1270-2018-06-21) for the rest –

- The new `Iterator::try_fold` and `Iterator::try_for_each` methods are both *short-circuiting* versions of their non-`try_` equivalents. So where `fold` requires that you have a non-fallible function and are responsible to manage the case where you failed on an earlier iteration yourself, `try_fold` will stop and return *immediately* if there’s a failure. The same thing goes for `try_for_each`: `for_each` keeps applying the closure no matter what, so you have to do some extra work yourself if you don’t want to operate if you’ve encountered a failure; `try_for_each` just returns immediately if you return an error. These are super handy for fairly common real-world scenarios with iterator operations!
- `String::replace_range` is exactly what it sounds like: it lets you replace a specified range within a string. Note that you’re responsible for making sure you’re on a character boundary, though; it’ll panic if your range starts or ends in the middle of a character. UTF problems!
- `Option::filter` is just like `Iterator::filter` (which is implemented for `Option`!), but you don’t have to call `.iter` or `.into_iter` and then `.collect` to get the result. So it’s at least potentially lower overhead *and* it’s definitely more convenient.

Again, for the others, see the [release notes](https://github.com/rust-lang/rust/blob/master/RELEASES.md#version-1270-2018-06-21)!

### Documentation updates

There are also some nice updates to the Rust documentation. First, and something I’ve desperately wanted for a couple of years, is the ability to search *all* the Rust documentation. You’ve been able to search the API docs for a long time, but *not* the Rust books. This has been a constant source of low-level pain for everyone – and it has driven me nuts whenever I’m trying to find the right page to link in show notes! – so this change is most welcome.

Second is that there’s now a guide for invoking `rustc` directly: it has its own dedicated book. Most of us use Cargo for all of our build interactions most of the time, but there are lots of times when you want to use `rustc` directly – particularly for including it in other build pipelines where the ongoing work to make Cargo integrate easily isn’t ready yet. This guide should be very helpful for those purposes!

## Rust 2018 Preview

Talking about documentation makes for a nice segue into talking about our *next* topic: the Rust 2018 Edition preview! We’re getting close to the target release for the Edition! As such, there’s an early alpha release out for testing. If you want to make sure the Rust 2018 Edition release is as solid as it possibly can be, you should set up the release preview using the `rust_2018_preview` crate-level feature flag on nightly and try out the various tweaks and improvements landing in the next few months. (To do that, you just add `#[feature(rust_2018_preview)]` to the top of your `lib.rs` or `main.rs` file.)

There’s an [announcement post](https://internals.rust-lang.org/t/rust-2018-an-early-preview/7776) in the internals forum and a [Rust 2018 Edition guide](#) book in the Rust docs, both of which are linked in the show notes, which give you the current status of the preview and guide you through the changes. You’ll also want to install the `rustfix` tool for some nice automatic code-modification changes: you can just `cargo install cargo-fix` and then run `cargo +nightly fix --prepare-for-2018` on your crate and it’ll correctly and losslessly update your codebase in place. This process will likely be familiar to people on the cutting edge of the JavaScript ecosystem, where codemods have become fairly common for migrating between versions of packages and even language-level changes, but it’s new for Rust, so you should definitely give it a whirl and report any errors or problems. It’s also directly relevant to the `dyn Trait` change I mentioned a couple minutes ago – it will make that fix *for* you!

The current set of Rust 2018 features which *work* include the stable changes we talked about earlier in this episode and in [the Rust 1.26 news episode](https://newrustacean.com/show_notes/news/rust_1_26/): `impl Trait`, `dyn Trait`, SIMD, improved match ergonomics, and inclusive ranges and slice patterns. They also include some done-but-not-yet-stable features, which I’ll cover when they stabilize: module system improvements, simplified lifetime declarations, and the new rustfix tool I mentioned above. The non-lexical lifetimes and inference of struct lifetimes from the struct fields are still works-in-progress – more on those soon as well!


## Outro

I did not have time to cover any community updates; there’s too much happening! So once again I’ll just commend [This Week in Rust](https://this-week-in-rust.org) and [the Rusty Spike podcast](https://rusty-spike.blubrry.net) for weekly updates covering a much broader swath! Also check out Matthias Endler’s [Hello Rust](https://m.youtube.com/channel/UCZ_EWaQZCZuGGfnuqUoHujw) for awesome video learning materials!

Thanks so much to everyone who sponsors the show. This month’s $10-or-more sponsors included:

- Daniel Collin
- Nick Stevens
- Peter Tillemans
- Chip
- Zachary Snyder
- Alexander Payne
- Paul Naranja
- Sascha Grunert
- Hans Fjällemark
- Daniel Mason
- Martin Heuschober
- David W. Allen
- Dan Abrams
- Behnam Esfahbod
- Aaron Turon
- Matt Rudder
- Derek Buckley
- Vesa Khailavirta
- John Rudnick
- Olaf Leidinger
- Anthony Deschamps
- Raph Levien
- Marshall Clyburn
- Damien Stanton
- Nathan Sculli
- Ramon Buckland
- Ryan Osial
- Oluseyi Sonaiya
- Chris Palmer

You can sponsor the show at <patreon.com/newrustacean>, or send a one-off at any of a number of other services listed at the website. Even more importantly, please let others know about the show – by telling them about it at a meetup, sharing it around in whatever media you use, or reviewing or recommending it in your podcast directory of choice.

You can find the notes for *this* episode at <newrustacean.com/show_notes/news/rust_1_27/>. The website also has scripts and code samples for most of the teaching episodes and transcripts for many of the interviews. 

The show is on Twitter @newrustacean, or you can follow me there @chriskrycho. Tweet me with news, topic ideas, etc! You can also respond in the threads on the Rust user forums, Reddit, or Hacker News, or—and this will always be my favorite—just send me an email at hello@newrustacean.com.

Until next time, happy coding!