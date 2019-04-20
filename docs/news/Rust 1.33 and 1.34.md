# News: Rust 1.33 and 1.34

Hello, I’m Chris Krycho and this is New Rustacean: a show about the Rust programming language and the people who use it. This is a news episode for Rust 1.33 and 1.34.

As I mentioned in a recent bonus episode, March was *extremely* full with my prep for a workshop at EmberConf—this episode covers both 1.33 and 1.34 as a result.

## Sponsor: Parity

Before we dive in, Parity is back sponsoring the show—because they want to hire *you* to come work in Rust with them!

Parity is advancing the state of the art in decentralized technology, and they’re using Rust to do it, leaning hard on its trifecta of performance, reliability, and productivity. They're building cutting-edge tech in areas like WebAssembly and peer-to-peer networking. Two of the larger projects they're working on are: Substrate, a framework for building blockchains, and Polkadot, a platform leveraging blockchain tech for scaling and interoperability in decentralized systems.

If that sounds interesting, check out their jobs at <parity.io/jobs>!

## Rust 1.33 Release

Okay, so let’s talk Rust 1.33.

### Moar `const fn` fun!

1.33 landed yet more features around `const fn`, allowing you to do yet more compile-time evaluation. As you may recall from my discussion of these in the Rust 2018 Edition episodes, there’s a tradeoff between these: you spend more time compiling, but with the benefit of effectively no runtime cost at *all* for getting the results of these computations. Deployed effectively, these can lead to significant improvements in the speed of your program or library. The 1.33 features are:

- Assignment! Yes, you literally couldn’t use assignment in `const fn`s before.
	- That includes things like the `+=` and `-=` operators.
	- It also works down into structs or when indexing into arrays, so you can do things `point.x += some_offset`..
- `let` bindings: as long as *they* in turn evaluate to constant results, they’re good.
- The same goes for `let mut`.
- You can use destructuring patterns if they *always* work. So if you have a struct `Point` with fields `x`, `y`, and `z`, you can write `let Point { x, y, z } = some_point;`, or you can do the same with tuples, and so on.
- You can use `const unsafe fn` from `const fn` now. (I was actually kind of surprised this wasn’t the case already!)

The net of that is that a *lot* of stuff can now be `const fn` which couldn’t before, and the Rust standard library now takes advantage of that.

#### Standard library changes for `const fn`

- The `overflowing_` and `wrapping_` math operations are now `const fn` for all numeric types.
- So are `is_positive` and `is_negative`.
- A *bunch* of handlers around binaries numbers and endianness—things like `leading_zeros` and `to_le` and `from_le`, for example.

I’m really curious to see if anyone has benchmarked representative programs and seen if or how this has improved run times.

### `Pin`

One of the most important items in 1.33 was the stabilization of the `Pin` type. `Pin` is a type representing an object which is *guaranteed* to keep the same location in memory throughout its life. Normally, objects in Rust do *not* have this property: they can *move*. This applies to when we talking “moving” in the context ownership, and in that context the specific dynamics of how objects move around in memory vary—they’re compiler-defined, in general, and that lets the compiler make a lot of interesting optimizations. It also applies to pointers: you can move the objects that a mutable reference or a `Box<T>` point to. There are, in short, a *lot* of ways that things can move in Rust.

But there are times when you need to be able to guarantee that an object does *not* move, and for that we have `Pin`.

When specifically might you want that? The API docs (and the announcement blog post) give the example of a self-referential struct type. If a type has a pointer to itself, and it moves, the pointer would become invalid—and this is undefined behavior. We don’t want that!

Now, it wasn’t immediately apparent to me when you’d want that, so I poked around a bit, and withoutboats’ [RFC for this API][RFC #2349] proved *extremely* helpful, so I strongly recommend you read it. One prime case where you need this is in a *generator*, which (to hand-wave a *lot*) a function which can suspend operation at one point in time and resume again later. (In JavaScript and Python, you may have seen this expressed with the `yield` keyword.) Making this work requires that you basically take the current stack frame and save it—turn it into an object that you can manipulate: storing it for reference, adding and removing things from it over time, etc.: at the end of the day, it’s a kind of state machine, and keeping track of and updating its internal state means keeping references to itself. This was `unsafe` in the formal Rust sense before the `Pin` type landed; `Pin` lets us express the guarantees we need to make this kind of thing safe. It’s similarly necessary to make the futures and async work viable.

[RFC #2349]: https://github.com/rust-lang/rfcs/blob/master/text/2349-pin.md

The stabilization adds the following new APIs:

- `Pin<P>` is a new type where the type parameter `P` is *any pointer type*—`&mut MyStruct`, `Box<YourStruct>`, `Rc<OurEnum>`, etc. This wraps those pointers in mechanics which forbid the use of *move* operations on the *value* behind the pointer. The *behavior* of something like `Pin<Box<T>>` is pretty much just like `Box<T>`… except that you cannot do things which cause moves, and that includes getting the actual `Box<T>` itself. The same goes for `Pin<&mut T>`—you can do basically all the normal things you can do with a referenced mutable value in Rust, *except move it*. No `mem::swap` calls here to exchange what is sitting in that memory for something else!

- `Unpin` is a matching trait, which effectively *undoes* the effect of `Pin`. That might sound strange, but it turns out that most types *don’t* have any need for this kind of memory-location stability. For example, an `i32` just doesn’t care whether it’s in the same spot: it’ll never be self-referencing anything… because it’s just an integer! As a result, `Unpin` is *automatically* implemented for most types. You may remember that this is true for a number of other traits in the standard library, most notably `Send` and `Sync`. Like `Send` and `Sync`, `Unpin` is a trait you can implement yourself, but also one you can explicitly opt *out* of for a given type by writing `impl !Unpin` for that type. Opting out means `Pin` has its defined effect: it cannot be moved.

In addition to these two types, there are some important rules about what it means to implement the `Drop` trait for a pinned type, and also about the relationship between a pinned struct and its fields. Rather than trying to explain those in this news episode, I will simply refer you to the really excellent and quite details [API docs][std::pin], which walk through these in detail and also provide some great examples of actual implementations with `Pin` and `Unpin`!

[std::pin]: https://doc.rust-lang.org/std/pin/index.html

### An Import Nicety

One last note for Rust 1.33: there’s a small feature I *really* appreciate in the module import syntax. You can now import an item as `_`: the same way you can use `_` to say “I don’t care about this *value name*” in a function or match expression, you can now use it to say “I don’t care about this *module name*” in a `use` statement. This is particularly handy for when you want to import a module’s trait implementations without importing (or aliasing) its name. I ran into exactly this myself not that long ago, so I was super happy to see this!

## Rust 1.34

Now, let’s dig into Rust 1.34. There are two big features I want to talk about here: alternative cargo registries, and the `TryFrom` trait stabilizing. Before I dig into those, a few library stabilizations I think are interesting (as always, you should take a look at the [full release notes][1.34 release notes] for the full list).

[1.34 release notes]: https://github.com/rust-lang/rust/blob/master/RELEASES.md#version-1340-2019-04-11

One API that stabilized is the `std::any::type_id` function. This lets you take a type and determine whether its compiler-generated, globally unique identifier is the same as that for a specific type. You can’t see into `TypeId`s, but you can use this for a certain amount of runtime reflection on otherwise opaque types. So, for example, you can define a function which determines if two types are the same:

```rust
use std::any::{Any, TypeId};

fn same_type<A: ?Sized + Any, B: ?Sized + Any>(a: &A, b: &B) -> bool {
    a.type_id() == b.type_id()
}
```

You can do a lot more with this when you combine it with the existing `TypeId` type and its implementations, which have been stable since Rust 1.0. This new `type_id()` helper just allows for some nice improvements to those runtime metaprogramming features!

As an aside: we haven’t talked about `Any` before, but it is roughly what it sounds like: a trait type which allows you to introduce a degree of runtime dynamicism into your types, by saying “This thing can be *anything*” – you can then do the kinds of dynamic programming you might be used to in other, more, uhh, *dynamically* typed languages. It’s not necessarily something you reach for often in Rust, but it’s nice that it’s here and nice that it continues to get some love!

### `TryFrom`

Up next, one of those two big items I mentioned: the `TryFrom` trait stabilized (and with it, the matching `TryInto`). This is a really, really nice win, and you probably know it if you’ve been programming Rust for any length of time. The existing `From` trait is great, but it *requires* that your `from` function *always succeeds*, and there are just an awful lot of times when you *cannot* write a function converting from one type to another in a way that always succeeds. Idiomatic Rust has usually worked around this by providing a `try_from` function which takes in the source type and returns a `Result` of the target type and the causes for failure… but because this has been done in an <i>ad hoc</i> way from codebase to codebase, you could easily end up in a spot where different implementations supplied their *own* `TryFrom` trait, or no such trait at all, and things were a bit of a patchwork as result.

Stabilizing `TryFrom`  solves this nicely: we now have a standard library trait that everyone can implement and use with their own types. `TryFrom` requires you to supply an associated type named `Error`, which is the error output, and a function `try_from` which returns a `Result` of target type or that associated `Error` type. `TryInto` is the inverse of `TryFrom`, just as `Into` is the inverse of `From`, so if you have an `impl TryFrom<T> for U` you automatically also get `TryInto<U> for T`. Finally, as a convenience for dealing with APIs which expect `Result`s, `TryFrom` is automatically implemented for `From` implementors, but which can never fail (you’ll never get the error case of the `Result`). This is one of those nice “fix a papercut and do it well” kinds of changes I love to see.

### Alternative registries

Okay, last but perhaps most importantly for many large organizations: support for alternative registries for Cargo is now stable! In general, many individuals and companies are happy to use the normal crates.io registry for finding and using packages. However, in a number of contexts—organizations with specific security needs, often including *large* organizations—crates.io (excellent though it is) doesn’t meet the specific requirements of the organization for vetting the packages it  uses. Additionally, many organizations of *all* sizes need *private* registries of packages which are internal-only: it’s easy to forget if you’re doing a lot of open-source stuff, but everything on crates.io is public! Organizations with either of those sets of needs will often set up their own private registries for packages they host—alternative npm registries, Nuget repositories, etc.

As an example of this: my own employer, LinkedIn, has very specific policies about how we use third-party dependencies from npm, for example, and we’re not pulling directly from npm but from our own registry for our vetted JavaScript dependencies. Similarly, my previous employer, Olo, had private registries for both npm and Nuget packages for internal code shared across multiple projects in the organization. This is *very* normal… and Cargo hasn’t had first-tier support for it until Rust 1.34. You’ve been able to work around it with Git URLs or by pulling down source code locally and using paths to map things in your `Cargo.toml`, but neither of these were great, and they definitely weren’t viable approaches for large engineering teams like my own!

Rust 1.34 ships with really *nice* support for alternative registries. You just add a `[registries]` section to your `Cargo.toml` which points to the URL for your alternative registry, and then when you add a dependency you specify both the version and the registry to use. You can now also use a `--registry` argument when publishing or logging into a registry, and there are some docs about how to run a registry.

This is one of those changes that has been a long time coming, in part because there wa so much to do last year for the 2018 Edition release… but also in part because it’s a thing the teams involved wanted to get *right* as much as possible, because it’s really important. And now that it’s out there, it’s that much easier for large or particularly security-conscious organizations to adopt Rust, and that’s a big win! I’m excited to see where it goes from here!

## Conclusion

Thanks as always to this month’s $10-or-more sponsors:

- Evan Stoll
- Martin Heuschober
- Soren Bramer Schmidt
- Ramon Buckland
- Ryan Osial
- Behnam Esfahbod
- Rob Tsuk
- Oladapo Fadeyi
- Andrew Dirksen
- Embark Studios
- Jason Bowen
- Chip
- John Rudnick
- David Carroll
- Anthony Deschamps
- Nathan Sculli
- Michael Mc Donnell
- Nick Gideo
- Raph Levien
- Bryan Stitt
- Zach Peters
- Matt Rudder
- Arun Kulshreshtha
- James Hagans II
- Adam Green
- Daniel Bornkessel (April only)
- Scott Moeller
- Jerome Froelich
- Jeff May
- Dan Abrams
- Jonathan Knapp
- Daniel Mason
- Nick Stevens
- Cristian Paul
- Brian McCallister
- Alexander Payne
- Peter Tillemans
- Daniel Collin
- Dominic Cooney
- Paul Naranja
- Oluseyi Sonaiya
- Graham Wihlidal
- Jako Danar
- Nicolas Pochet
- Johan Andersson
- Joseph Schrag
- Benjamin Manns

You can sponsor the show at patreon.com/newrustacean or via other services listed on the show website, <newrustacean.com>. There, you’ll also find show notes, including links to things I talk about, scripts, code samples, and interview transcripts. The notes for *this* episode are at <newrustacean.com/show_notes/cysk/bindgen/>.

Please recommend the show to others if you like it, whether in person, via your podcast directory, or in various media online! You can contact me at @chriskrycho or @newrustacean on Twitter, or by sending men an email at hello@newrustacean.com.

Until next time, happy coding!