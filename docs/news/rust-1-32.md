# News: Rust 1.32

Hello! I’m Chris Krycho and this is New Rustacean: a show about the Rust Programming Language and the people who use it. This is a news episode for Rust 1.32.

## Sponsor: Parity

First up, Parity is back sponsoring the show, because they want to hire Rust developers. A.k.a. *you*!

Parity is advancing the state of the art in decentralized technology, and they’re using Rust to do it, leaning hard on its trifecta of performance, reliability, and productivity. They're building cutting-edge tech in areas like WebAssembly and peer-to-peer networking. Two of the larger projects they're working on are: Substrate, a framework for building blockchains, and Polkadot, a platform leveraging blockchain tech for scaling and interoperability in decentralized systems.

If that sounds interesting, check out their jobs at <parity.io/jobs>!

## Rust 1.32

Okay, so let’s dig into what will be the *last* news episode for a while! Because I got the Rust 1.31 episodes out late, this is a quirky month with three news episodes in it! Gladly, there won’t be another one for about six weeks and you should get *at least* one bonus episode and one teaching episode between then and now!

Okay, so let's jump in, because Rust did *not* slow down with this release.

### `dbg!`

First up is the `dbg!` macros. If you're like me, you *sometimes* use a debugger like lldb, but just as often you're using the `println!` or `eprintln!` macros to dump the output and state of some part of your program while you're working out why some data structure isn't in *quite* the shape you wanted. This is *fine*, but it's definitely not *amazing*, and it's incredibly repetitive, and you always have to remember the slightly odd debug format string invocation: `{:?}` or, to pretty-print the debug output, `{:#?}`. (Confession: I always forget the latter even exists, but it makes the output way more useful!) Rust 1.32 to the rescue with `dbg!`. You still have to `#[derive(Debug)]` for types which you want to be able to print this way (though in truth I do this for nearly every type in my programs anyway). But once you do, you can just do `dbg!(some_value);`: no format string required. This is a quality-of-life win, not some kind of huge change to the language, but I *love* it.

### Breaking up with jemalloc

Up next is a *really important* but also somewhat *in the weeds* kind of change. As of Rust 1.32, all platforms now use the system allocator by default, instead of jemalloc. I talked a bit about this in the episode for Rust 1.28, when Rust got the *ability* to switch allocators—that was important foundational work for this change, so that people who *want* to keep jemalloc can. Also, this is not actually a change for *all* platforms; Windows already used the system allocator.

As a quick refresher: the *allocator* is the piece of your program environment which gives you memory when you need it. A memory allocator has a couple responsibilities:

- it has to give you *enough* memory for the amount you need, e.g. if you need to double the size of a `Vec`
- it has to reliably handle conditions when your system is out of memory (which can happen if, say, you have too many instances of Slack up… sorry, too soon)
- it has to try to keep the allocations it makes well-structured, so that it can *keep* providing memory as needed – if things get chunked up weirdly, there may be enough *total* memory available for a given request, but none of it in big enough individual *blocks*

And there are, no doubt, more that I'm unaware of; I've never actually needed to deal with allocators in detail. But like a lot of things: when you *are* dealing with them, it's super important!

jemalloc does all of those things, but it has its own set of quirks and behaviors, and the reason Rust was using it had to do with the time in Rust's history where it had a green-threading model—more like Erlang or Go! That time passed *ages* ago and it makes the most sense to *default* to the system allocator and let people opt in other directions if they have good reason to. So: now that's what we do! And one big and immediate upside of this is that your binaries should be smaller. I saw [one user on Twitter][smaller] report that they saw "hello world" drop 576kb to 280kb: almost 50% smaller. Obviously you're not going to see 50% reduction in the size of your app by doing this in general—it's probably more like a constant size decrease, and on the order of hundreds of kilobytes, and the specific amount will depend on the specifics of your system. It won't change at all on Windows, for example, since jemalloc was *never* the default on Windows! A nice little win in any case.

[smaller]: https://twitter.com/softprops/status/1086723200095059972

### Unified paths

Probably the biggest language feature landing in this release is uniform paths: the final part of the module system improvements I've talked about over the last few months. "Uniform paths" here means that the path system now works exactly the same way for imports—that is, `use` statements—as it does for other references to paths. Historically, a `use` path always started from the root of the crate, while other paths always started from their current location in the crate. This meant that in your root module, they *appeared* to work the same way, because *by coincidence*, they did—but as soon as you had a submodule, the two kinds of paths stopped working the same way. The confusion that sprung from this difference was one of the major motivators for reforming how module paths work in Rust.

Paths got a lot clearer already, and I talked about that a few episodes back when the "path clarity" changes landed. Uniform paths are the final touch, though, and they make for a much cleaner and much easier-to-learn system: you can always use the *same* kind of path from anywhere in your crate, in any kind of path statement, and it works the same way. So if you write `some_module::some_item` in a use statement, that's a relative path, just like it would be if you were writing a path to an item for resolving it in a `where` clause or a type definition or something.

### `Self`-absorption

There is one other language level feature out in this release: a bunch more places where you can use the `Self` type. One is that you can use `Self` as the name of the type when constructing unit or tuple structs, like when you are using a tuple struct for the newtype pattern to give yourself a type-safe wrapper for another type (e.g. `struct Email(String)`, to give yourself a tool for differentiating between strings which have been validated as emails and strings which are just, well, strings): in an `impl` block for `Email`, you could write `Self(some_validated_string)` instead of `Email(some_validated_string)` for constructing the type. Similarly, you can use the `Self` shorthand in type definitions. The release notes give the example of a constrained `List<T>` type, where you want it to be an *ordered* list: you might write it with a `where` clause whose body was `Self: PartialOrd<Self>` instead of `List<T>: PartialOrder<List<T>>` and so on. It's a nice short-hand. Nothing world-changing, but in each of these cases it means you can write the type *just once* and refer to it as `Self` after that, as you already can in many other places.

### Library stabilizations!

This release also saw a number of neat library stabilizations beyond the fancy new `dbg!` macro.

First up, the first major batch of standard library use of `const fn` landed! This should lead to a small but meaningful increase in the performance of these at runtime! A handful of illustrative examples: `char::is_ascii`, `str::as_ptr`, `Ipv6Addr::new`, and `Duration::as_secs`. Any time you're in a const context, these will now be evaluated at compile time, so they never have to be calculated by the running program.

Second, there are a bunch of new handlers to convert little and big endian bytes to numbers in Rust's native (i.e. the local) representation. If you've never encountered the idea of endianness before, it's the order in which information is stored in the binary value that makes up a byte: if you're reading them as a set of numbers, is the first bit on the left side of a byte, or the right side of a byte? And the same for bytes: is the smallest/least-significant byte first or last for a whole data structure? Here, "least significant" means *smallest*, e.g. in binary the bit representing 1 is smaller, or less significant than the bit representing 2, which is smaller—less significant—than the bit representing 4, and so on. Little-endian puts the least significant bits and bytes first; big-endian puts the least significant bits and bytes *last*. This doesn't often come up in most high-level programming, but when you're dealing with network protocols or low-level hardware concerns, it can be *very* important. So now Rust has convenient handlers to take data from either representation and convert them into numbers, where you no longer have to concern yourself with endianness.

## Community

Finally, a few things that recently came to my attention in the last few weeks—which may not be *new*, but which were at least new to *me*!

### [Amethyst]

The first of those is [Amethyst]: a game engine [written in Rust]. As an aside, this one came in  via listener Alexander Lozada, who did what I'm always asking people to do and emailed me about it! Amethyst is aiming to be both incredibly fast—it leans hard on Rust's ability to do safe multi-threaded computation—and also idiomatic and easy to use for game developers. It uses the Entity-Component-System architecture that is, as I understand, a standard pattern in game development (the [keynote at RustConf 2018][keynote] dug into why that's the case *and* into how Rust is a really great fit for it). Amethyst has [a whole book] devoted to using it, a bunch of [examples] to help you see how to get started with it, lots of [docs], integration *today* with OpenGL and work in progress for Vulkan and Metal… basically, this is an attempt to make game-development in Rust *easy and good*, and I think that's fantastic.

[Amethyst]: https://www.amethyst.rs/
[written in Rust]: https://github.com/amethyst/amethyst
[keynote]: https://m.youtube.com/watch?v=aKLntZcp27M
[a whole book]: https://www.amethyst.rs/book/latest/
[examples]: https://github.com/amethyst/amethyst/tree/v0.9.0/examples
[docs]: https://www.amethyst.rs/book/latest/

### [`insta`][insta]

Another neat crate I saw fly across my radar is [`insta`][insta]—a brand new tool for *snapshot testing* in Rust. Snapshot testing is a style of testing that has gained a ton of steam in front-end web development in the last few years, especially in the React community. The basic idea is: you generate an expected value—often for something a bit more complicated than just some small piece of data; in the front end web world it's often something like some rendered DOM—and store it in a separate file from the test. When you run the tests after that, they simply diff their output against the contents of that file. If you make an intentional change that alters the output you expect, you update the snapshot. This can make your test code a *lot* easier to read and maintain: instead of having to basically include all of that output directly in the body of your test, you can store it *beside* the tests and let your test code simply do the work of describing what you expect to happen.

[insta]: https://crates.io/crates/insta

The `insta` crate applies this exact same model to items in Rust: it works with anything which supports either the [`Debug`][debug] trait or [Serde]'s [`Deserialize`][de] trait. In your tests, you use one of the macros it supplies:

[debug]: https://doc.rust-lang.org/std/fmt/trait.Debug.html
[Serde]: https://docs.serde.rs/serde/
[de]: https://docs.serde.rs/serde/trait.Deserialize.html

- `assert_snapshot_matches!` compares basic strings.
- `assert_debug_snapshot_matches!` compares the output of debug printing a data structure.
- if you turn on the `serialize` feature in your crate, you can use `assert_serialized_snapshot_matches!` to compare any type which supports the Serde `Deserialize` trait: it'll generate YAML for the snapshot and compare that output.

When you run your tests the first time, you make sure they fail the way you *expect* them to if there is no snapshot, and then generate the snapshot by setting the `INSTA_UPDATE` environment variable to `1` with your `cargo test` invocation: that will *generate* the snapshots for future use.

As soon as I looked at this I realized I *want* it very much, for an easier way to test a lot of the various pieces of my (still moving, just very slowly) static site generator project: I have a number of tests in a working branch which just make sure that the tool serializes and deserializes various complex data structures in exactly the way I want, and snapshots would help a *ton* with them.

## Closing

As a bit of personal news here at the end of the episode: I’m starting a new job on the 28th (a week from when this episode releases)—I'm going to be working on front-end infrastructure at LinkedIn. I'm excited about this role for a *lot* of reasons, but one near the top of the list is that there's a very good chance I will actually get to write some Rust in that role over the next year! Thinking on that as I wrote up this script also made me reflect a bit  on the longevity of this show! It’s kind of amazing to me that New Rustacean has run as long as it has and has had the success it has: I have been making this show longer than I’ll have had *any job*. That’s thanks to all of you who listen, and especially to the folks who’ve seen fit to sponsor the show. Thank you! And more to come!

On that note, this month’s $10-or-more-sponsors included:

- Bryan Stitt
- Oluseyi Sonaiya
- Daniel Collin
- Scott Moeller
- Behnam Esfahbod
- Rob Tsuk
- Paul Naranja
- Johan Andersson
- Raph Levien
- Chip
- Andrew Dirksen
- Matt Rudder
- Joseph Marhee
- Nicolas Pochet
- Nick Gideo
- beaorn
- Jonathan Knapp
- Alexander Payne
- Dan Abrams
- Nathan Sculli
- Chris Palmer
- Jerome Froelich
- Brian McCallister
- Ramon Buckland
- Michael Mc Donnell
- Peter Tillemans
- Daniel Mason
- Anthony Deschamps
- James Hagans II
- John Rudnick
- Nick Stevens
- Jako Danar
- Embark Studios
- Graham Wihlidal
- Ryan Osial
- Martin Heuschober
- Evan Stoll

You can sponsor the show at patreon.com/newrustacean or via other services listed on the show website, <newrustacean.com>. There, you’ll also find show notes, including links to things I talk about, scripts, code samples, and interview transcripts. The notes for *this* episode are at <newrustacean.com/show_notes/news/rust_1_32/>.

Please recommend the show to others if you like it, whether in person, via your podcast directory, or otherwise! You can contact me at @chriskrycho or @newrustacean on Twitter, or by sending me an email at hello@newrustacean.com.

Until next time, happy coding!