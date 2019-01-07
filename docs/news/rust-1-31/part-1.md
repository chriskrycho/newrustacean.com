# News: Rust 1.31 – The 2018 Edition, Part I

Hello, I’m Chris Krycho, and this is New Rustacean: a show about the Rust Programming Language and the people who use it. This is a news episode for Rust 1.31 and the 2018 Edition!

My discussion of the 2018 edition is going to be split over two episodes, because the changes here are large and they’re a big deal! These episodes are unfortunately about a month late, but that’s because (a) I was busy helping with the final polish and push for the new Rust website, (b) I ended up dipping back into the burnout a bit from overcommitting in November and December, (c) December is *always* incredibly busy with holidays, (d) *wow* is there a lot to cover here, and (e) I was working on the interview you just heard. But here’s to more consistency (and less burnout) in 2019!

## Sponsor: [Parity Technologies](https://paritytech.io/jobs)

First, thanks to the show’s current sponsor: Parity Technologies!

Parity is advancing the state of the art in decentralized technology, and they’re using Rust to do it, leaning hard on its trifecta of safety, speed, and correctness. They're building cutting-edge tech in areas like WebAssembly and peer-to-peer networking. Two of the larger projects they're working on are: Substrate, a framework for building blockchains, and Polkadot, a platform leveraging blockchain tech for scaling and interoperability in decentralized systems.

And they’re not just *using* Rust, they’re hiring Rust developers! So if you’d like to work on any of these projects, check out their jobs at [paritytech.io/jobs](https://paritytech.io/jobs).

Now: into the show!

## The 1.31 Release

This first episode on the 1.31 release will focus on two things: the mechanics of the release, and a couple of the features that appear in both editions.

### Setting the Stage

Rust 1.31 was the single most important release since Rust 1.0. In many ways, it’s likely to be just as determinative of the future of the language as the 1.0 release was. The 1.0 release put Rust on a stable footing with strong backwards compatibility guarantees: it gave people a foundation to start building on, sure that the foundation would still be there in six months. (Before 1.0, that wasn’t a given! Things changed a *lot*!) But it was unquestionably a *beginning*. The ecosystem had a few important pieces already, thanks to Rust’s long history before 1.0, but many of those pieces were still quite immature—and for many kinds of projects, you were going to be building everything yourself.

Over the last three and a half years, that has changed dramatically. The crates ecosystem is vibrant and growing steadily. So much so that we’ve started to have the kinds of bumps you expect with a bigger ecosystem—bad actors on crates.io or crate abandonment, for example—as well as the utility that comes from having a lot of high-quality libraries and applications now available in Rust. Unlike when 1.0 launched, in many cases it’s now quick and easy to build something in Rust. As just one small example: I now just default to using Rust if I want to quickly whip up a small command line tool, because I find it’s now just as quick and easy for me to do that as to use Node.js or Python—and with Rust’s compiler helping and far better performance than I get from JS or Python!

The Rust 2018 Edition has two aims:

1. To present all the improvements that have happened gradually over the last three and a half years of a release every six weeks in a coherent way.
2. To make some small improvements in the form of *backwards compatible breaking changes*, based on what we’ve learned and built over that time.

I talked about both of these ideas back when I first mentioned the Edition release – all the way back in the beginning of 2018, when we were still calling it an Epoch – but I think they’re worth digging into again and in more detail in light of the release.

### Presenting the gradual improvements

Rust 1.0 was a big deal not least for telling a story to the world. The story of Rust 1.0 was: <i>we have a stable foundation for you to build on; now, come build an awesome ecosystem with us!</i>

Since 1.0, if you’ve kept up, you’ve seen a ton of changes in the language and the ecosystem. However, that phrase, <i>if you’ve kept up</i>, is an important qualifier! Lots of people have *not* kept up. Plenty of people in the world undoubtedly took a look at Rust in the early 1.x days and decided *not* to dive in. Or maybe they started, but some of the gaps in the ecosystem and the rough edges in the language itself kept them from being able to progress. And while a lot of that has changed, it has changed on our 6-week, not-that-big-of-a-deal cycles. Most of these point releases don’t make a big splash, and that’s the way it should be! But it also means that most of these point releases have flown under the radar for many people who might be interested!

Well, the Rust 2018 Edition release is another opportunity to tell a story to anyone out there who hasn’t been following along with all of the point releases along the way. The story now is: <i>We have a really rich and increasingly mature ecosystem here; come build things on top of it!</i>

The standard library is far more mature now than it was in 2015, of course. So is the Crates ecosystem: many crates exist today that simply weren’t around three and a half years ago, and many crates are *stable* today that could only run on nightly three and a half years ago – especially important for core parts of the ecosystem like Serde. And the language itself is substantially easier to use, courtesy of the many improvements to the language that have landed over the last few years, from small-but-important things like better error messages, to major improvements to the module system. It’s important to communicate all those changes and improvements!

An Edition is also an opportunity to get the whole ecosystem in sync. As these changes have landed piecemeal over the last three and a half years, everything from what “modern” Rust code looks like to the state of the docs has gotten a bit scattered at times – and not through anyone’s fault at all! Keeping it all perfectly aligned is probably impossible and certainly not worth it. However, in line with the idea of presenting Rust well, an Edition gives us an opportunity to make sure the docs all present the latest and best thinking, that important parts of the ecosystem are aligned on the same ways of doing things, that our tools are working together as smoothly as they can be, and so on.

The net of all of this is that if you have been using Rust for the past three and a half years, and you go look at the Edition Guide, you’ll think, “Wait, that landed way before the 2018 Edition!” for a lot of the features. And that’s true! But much of the world *hasn’t* been using Rust for all of that time, and there was no single place to find all of that information previously. The Edition *Release* brings all of those changes together into one coherent announcement: here is our way-more-mature language and our way-more-mature ecosystem! And the Edition *Guide* captures all of the differences that have landed in the language over those three and a half years.

### Making some “backwards-compatible breaking changes”

The other major initiative here is to make some backwards-compatible breaking changes. That phrase seems like a contradiction in terms: how can a breaking change be backwards compatible? The trick is that the only “breaking” changes allowed are at the level of surface syntax—*not* the deep semantics of the language. And there are only a couple even of these!

Now, why do we even need these kinds of changes?

When you design a programming language, you often reserve some identifiers – strings of characters – as <i>keywords</i>. These are words with special meaning that the compiler knows to do special things with. Obvious examples in Rust are things like `fn` for function declarations, `match` for match blocks, `trait` for trait definitions, and so on. You can’t write `let fn = 42;` – you’ll get the error “expected pattern, found keyword `fn`”. So when you’re building the language and make a commitment to stability, as Rust did at 1.0, you’ve locked in those keywords: because if something is *not* a keyword, then it’s fair play to use it anywhere, but if it *is* a keyword, you *can’t* use it anywhere!

But of course, sometimes you change your mind on what things should be keywords further down the line. In Rust’s case, we *really* want `async` to be a keyword we can use as syntax sugar for functions which deal in `Future`s to smooth the process of writing high-performance asynchronous programs. I’ll be covering `Future`s and `async` and `await!` sometime in the first half of 2019 – it’s been on my radar for a long while, but I haven’t dug in because I’ve been waiting for a lot of it to stabilize, so that what I said wasn’t very quickly out of date and inaccurate. For now, it’s enough to know that `async` and `await` were not reserved keywords in Rust 2015, so people could (and presumably did!) use them in their codebases over the last three and a half years. They *are* reserved keywords in Rust 2018, so code that used them… would now break!

Since the breaking changes are only syntactical, though, it’s relatively straightforward for the compiler to operate in different modes for the 2015 edition and the 2018 edition. I say “relatively” because I don’t want to down-play the amount of work that went into making this happen! But it’s important to realize that while there are a couple new keywords and a couple removed keywords, and one new bit of syntax for strings in support of that, that really only affects the *parse* step of the compiler: everything past that is the same in both editions.

This is why the compiler can straightforwardly support 2015 and 2018 edition code side-by-side: after the parse step, *it’s the same thing*. And if things shake out as currently planned for a 2021 Edition, that will be true then as well. The compiler just has some different *parse* modes, and the underlying semantics are all still stable. This is *very* different from the breaking changes that happened in Python 3, where not only surface syntax but also important parts of the language semantics changed.

### Mechanics

Let’s talk for a minute about the *mechanics* of the 2018 edition. How exactly do you opt into using the new edition, and what changes when you do?

From your perspective as a user (that is, *not* as an author of the compiler itself), the only thing you have to care about is the edition flag: a value set in your `Cargo.toml` file (or passed explicitly as an argument to `rustc`) to specify the edition. It currently takes two values: `2015` or `2018` The flag was supported on stable Rust as of 1.30, and 1.31 stabilized the `2018` value for the flag. If you do *not* set the flag in your `Cargo.toml` file, the compiler will default to the 2015 edition (for backwards compatibility). From the release of 1.31 forward, any project created with `cargo new` will set the edition key to `2018` automatically.

Now, if you have 2015 Edition code, *some* of it may not compile if the 2018 edition flag is set. This is why the recommended migration path is not to simply add or change that value in your `Cargo.toml`, but instead to *start* by running `cargo fix --edition`, which will safely rewrite your code to compile in both editions – or, in a few cases, tell you what kind of change you need to make yourself if it can’t see how to do it safely. (I covered `cargo fix` in a bit more detail back in [the news episode for the 1.29 and 1.30 releases](https://newrustacean.com/show_notes/news/rust_1_29_1_30/ "News: Rust 1.29 and 1.30").) *Then* you set the 2018 Edition flag in your `Cargo.toml` and you’re off to the races.

`cargo fix` also has an `--edition-idioms` flag, which will further rewrite your code to be *idiomatic* in the new edition. If you want to use that flag, you should run it *after* the earlier steps. However, notice that this feature is still early in development: it’s able to do less, and what it can do doesn’t always leave your code in a compile-able state! So that’s not a *recommended* part of the edition upgrade, but 

## Some of the Both-Edition Features

There are a number of features which are present in the 1.31 release for *both* editions. In this episode, I’ll be covering the first handful of them:

- better lifetime elision rules
- library stabilizations
- Cargo features

### Better lifetime elision

One of the biggest win that’s on both editions is a set of improvements to lifetime elision. (The other is `const fn` types, which I’ll talk about next time!) Historically, you had to write lifetime annotations in a bunch of places that felt needless and obvious.

For one, when implementing a trait on a type which has a lifetime constraint, you had to write something like `impl<'a> TheTrait for TheType<'a>`. This duplication was annoying, and it usually wasn’t adding any actual information; it was literally just noise. So now you can write `impl TheTrait for TheType<'a>` – no more lifetime annotation on the `impl` keyword! Even better, if you don’t need to name the lifetime to track where it belongs, you can just give it the placeholder `_` lifetime: `impl TheTrait for TheType<'_>`. That signals to everyone reading it that “hey, there’s a lifetime here, but it’s not something the trait implementation interacts with!”

Lifetime annotations on structs also got better inference. If you were writing a struct with a lifetime `'l` and a generic `T` bound to that lifetime before, you had to write out explicitly `T: 'l` for that in the generic bounds on the struct, e.g. `struct Foo<'l, T: 'l>` – even if that relationship was obvious from how that lifetime was used in the struct. Now, if you have some property `bar: &'l T` in the struct, you can just write the top-level definition as `struct Foo<'l, T>`. A small improvement, but a welcome one.

### Library stabilizations

As is usual for a point release with Rust, there are a handful of nice little library stabilizations.

- The `NonZero` types (which [I talked about when they were stabilized back in Rust 1.28](https://newrustacean.com/show_notes/news/rust_1_28/ "News: Rust 1.28")) can now easily be converted into their corresponding regular (*not* non-zero) type: e.g. you can get a `u8` from a `NonZeroU8`. This is a pretty obvious stabilization: it’s always safe to go from a type which *cannot* be zero to one which *can*.

- There’s another convenient new `From` implementation: to go from an `&Option<T>` to an `Option<&T>` and from `&mut Option<T>` to `Option<&mut T>`. It’s not uncommon to have a reference to the container type and need a reference to the type it’s containing, and these are symmetric and safe!

- You can multiply time durations by unsigned 32-bit integers and 64-bit floating point numbers!

### Cargo features

Cargo got one new feature in support of the module improvements I covered a bit in the last news episode (and which I’ll cover more on in the next episode): in the `Cargo.toml` file, you can now specify an external crate rename. Historically, you’d have done that with `extern crate foo as bar;`, but since it’s no longer idiomatic to have *any* `extern crate` declarations, you can specify the key `package` with the desired name in `Cargo.toml`: `foo = { version: 1.0, package = "bar" }`. This doesn’t come up all that often in my experience, but it’s handy if you have a name conflict between a third-party crate and your own crate.

Cargo also got support for using HTTP/2 to download packages in parallel. While most of the time you spend doing an initial build is spent on the *build* part of it, some of it goes to downloads; parallel downloads can in principle speed this up!

## Closing

We’re out of time for *this* episode; I’ll be back next week with another covering the rest of the 1.31 and 2018 Edition features.

Thanks to everyone who sponsors the show! This month’s $10-or-more sponsors included:

- Alexander Payne
- Matt Rudder
- Joseph Marhee
- Rob Tsuk
- Ryan Osial
- James Hagans II
- Bryan Stitt
- Dan Abrams
- Brian McCallister
- Andrew Dirksen
- Graham Wihlidal
- Steffen Loen Sunde
- Nick Gideo
- Raph Levien
- Daniel Mason
- Behnam Esfahbod
- Nick Stevens
- Chris Palmer
- Paul Naranja
- Michael Mc Donnell
- Nathan Sculli
- Peter Tillemans
- John Rudnick
- Chip
- Daniel Collin
- Nicolas Pochet
- Jonathan Knapp
- Ramon Buckland
- Jerome Froelich
- beaorn
- Scott Moeller
- Adam Green
- Embark Studios
- Oluseyi Sonaiya
- Martin Heuschober
- Johan Andersson
- Jako Danar
- Anthony Deschamps

If you’d like to sponsor the show, you set up ongoing support at patreon.com/newrustacean, or send a one-off at any of a number of other services listed at newrustacean.com. The website also has scripts and code samples for most of the teaching episodes as well as transcripts for many of the interviews, along with full show notes for every episode. You can find the notes for *this* episode at <newrustacean.com/show_notes/news/rust_1_31/part_1>.

If you're enjoying New Rustacean, please help others find it – by telling them about it in person, by sharing it on social media, or by rating and reviewing the show in your favorite podcast directory.

The show is on Twitter @newrustacean, or you can follow me there @chriskrycho. Tweet me with news, topic ideas, etc! You can also respond in the threads on the Rust user forums, Reddit, or Hacker News, or—and this will always be my favorite—just send me an email at hello@newrustacean.com.

Until next time, happy coding!
