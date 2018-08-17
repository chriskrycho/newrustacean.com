# Rust 1.28

Hello, I’m Chris Krycho and this is New Rustacean: a show about the Rust programming language and the people who use it. This is a news episode for Rust 1.28.

## Sponsor: Parity Technologies

Parity Technologies is sponsoring this episode because they need Rust engineers! They’re advancing the state of the art in decentralized technology, from their flagship software, the Parity Ethereum client, to building cutting-edge tech in areas like WebAssembly and peer-to-peer networking. Their next big project is Polkadot, a platform leveraging blockchain tech for scaling and interop in decentralized systems. Parity loves Rust for its trifecta of safety, speed, and correctness! If that sounds interesting, check out their jobs at paritytech.io/jobs.

Thanks again to Parity!

## Rust 1.28

Rust 1.28 was a slightly smaller release than the last few have been, but that’s okay: as we’ll talk about in a few minutes, a lot of what is happening is the build-up to the 2018 Edition release. Before we get into some of the things going on with the Edition release, though, let’s dig into what *did*land with 1.28.

### Language features

#### Global allocators

The major feature that stabilized this release is the `#[global_allocator]` attribute and some supporting functionality in the standard library. This is a really important stabilization – not for most people’s everyday use of Rust, but for a lot of “specialized” domains.

Let’s take a step back and talk about what an allocator is. An allocator is the underlying library that your programming language uses when it needs to get more memory from the system. This happens all the time in normal Rust code – every time a `Vec` needs to grow because it’s going to be full otherwise, for example – but it usually happens “under the hood” and you don’t have to think about it. Different allocators have different behaviors and characteristics. Some are better at avoiding fragmentation of the allocated heap memory, for example, or accounting for multiple threads requesting new heap memory.

Rust usually uses *either* an allocator called jemalloc or the system allocator, depending on the platform. However, there are times when you simply don’t *have* a built-in system allocator, and you need to supply your own. For example, some “bare metal” environments are like this. So is my favorite Rust environment: WebAssembly. Nick Fitzgerald had a great [writeup](http://fitzgeraldnick.com/2018/02/09/wee-alloc.html) a few months ago on the [`wee_alloc`](https://github.com/rustwasm/wee_alloc) allocator he built for WebAssembly; I’ve linked both the allocator and the blog post in the show notes. Until 1.28, that required Nightly… but now it doesn’t! This is one of a number of really important steps along the path to supporting *all* those environments where you have very different needs from the normal modern operating system context.

To opt into this (or, for that matter, to opt into using the system allocator on a platform where Rust normally uses jemalloc), you define a static variable, assign the relevant allocator to it (`System` in the case of the system allocator, `wee_alloc::WeeAlloc::INIT` in the case of `wee_alloc`, etc.), and then set the `#[global_allocator]` attribute on the static variable you declared. It’s actually a pretty simple way to be able to flip such an important switch!

To go with all of this, there’s also a new trait in the standard library, `GlobalAllocator`, which you can implement to supply your own custom allocator. As usual, the docs are really solid here – they tell you *super* important things like the fact that custom allocator methods are not allowed to panic (this is actually undefined behavior)!

As noted, of course, most of that will not matter to you *most* of the time as a Rust user. But it’s going to unlock a lot of great stuff on stable in the months ahead!

#### Format string error messages

Another language stabilization in Rust 1.28 is a small change in the big scheme of things, but a real quality of life improvement. The error messages you get for invalid format strings – the ones you pass to things like the `write!` and `println!` macros – are nicer now! They used to be pretty non-descriptive and at times could actually make you think the error was in the wrong place entirely. Now, they are much more in line with Rust’s nice-error-messages story in general: they tell you clearly why something is wrong and how to fix it. This remains one of my favorite things about Rust, and it’s nice to see it continuing to improve.

The last language stabilization I wanted to mention – see the [release notes](https://github.com/rust-lang/rust/blob/master/RELEASES.md#version-1280-2018-08-02) for *all* of them – is that unit tests marked with the `#[test]` attribute can now return `Result<(), E: Debug>`, following in the footsteps of `main` from a few releases ago. Like with the `main` change, this just eliminates a certain kind of boilerplate work we had to do when dealing with top-level `Result` instances.

### New standard library functionality

Besides the `GlobalAllocator` trait we talked about a minute ago, there’s one other nice little set of standard library stabilizations – one I’ve actually *already put to use* in some WebAssembly code I’ve been working on a bit. These are the `num::NonZero*` types. These let you define an instance of a numeric type, e.g. `u8`, which *cannot* be zero – in that specific case, a `num::NonZeroU8`. The docs note that this gets you some memory layout optimizations, but *I’ll* note that there are some times this just comes up in the domain you’re trying to capture in your software, too. I’m excited in general to see more happening in the numerics side of Rust; I think the language has a lot of potential here and it’ll be nice to see it continue to grow over the next few years.

And that’s pretty much it for Rust 1.28, though again there are more miscellaneous improvements in the release notes; you should always take a look to see all the details.

### A note on Rust 1.27.1. and 1.27.2

This seems like a good time to pause and note that there were two point releases for Rust 1.27. I think there was some trepidation in the ecosystem around this: did it reflect a lower degree of the commitment to stability, a higher degree of bugginess, or otherwise more problems with the process? In a word: *no*. It’s actually the opposite!

I’ll link to an official Rust blog post that traces this out in more detail, but in fact these two extra point releases are a reflection of the Rust team’s growing maturity and some real *improvements* in Rust. Team-wise, there’s now a dedicated release team which manages all the hard coordination work of getting a Rust release out the door. (If you think about all the components that have to go out to get that right, you can see how that would take a whole subteam! It’s a lot of work!) That team’s existence means that shipping point releases when bugs are found is *possible* in a way it hasn’t always been.

The other thing is that the specific bugs that were found were found because of the new, more robust system for borrow checking that has been developed. The borrow checker we all use today is pretty old in Rust terms, and like all software it has some bugs; the new borrow checker is much more robust and it has actually exposed these bugs. So when it comes to stable, not only will you have a much nicer time in a lot of cases (because it solves some major ergonomic problems), but it will also have many fewer such bugs over *its* lifetime.

## Rust 2018 Edition Update

One other big Rust core bit of news we should cover is the revised schedule for the Rust 2018 edition release. Originally, the *next* release (1.29) was the target for the edition. Unsurprisingly, software being what it is, that date [has slipped a bit](https://internals.rust-lang.org/t/rust-2018-release-schedule-and-extended-beta/8076 "Rust 2018 Release Schedule and Extended Beta (Discourse Thread)"). The 2018 edition release is now targeted for Rust 1.31, releasing on December 6. There’s also a slightly unusual release cadence to go with it, in the interest of getting every last possible bit of polish we can.

The nightly version corresponding to the Rust 1.28 release – the one out *right now*, in other words – is *Edition Preview 2*. The 1.29 release will have its *beta* channel be *Release Candidate 1* and the 1.30 release will have *its* beta be *Release Candidate 2*. That is: the Rust 2018 edition beta will be on the beta branch on the very next release: 1.29. It’ll stay there for 1.30, and then – assuming there are no catastrophic bugs or problems discovered late in the game – be promoted to stable in the 1.31 release.

It’s worth elaborating on that slightly. Normally, Rust’s train model means the code that goes in a beta in one release goes into stable on the next release. We’re *intentionally* not doing that this time around. The 2018 edition will be on the beta channel in 1.29 *and* 1.30, with only bug fixes going into that. The aim here is to make sure that the 2018 edition release is as tested, polished, and solid as it possibly can be. As I noted when I first explained the Edition process earlier this year, this is a huge opportunity for presenting Rust to the world again – almost as much so as Rust 1.0 was, and probably no less important in many ways – so it’s important that we get it right!

You can help with that. So please *do* help! Please go test the edition changes in your codebase – whether that’s just a small learning project, or something really substantial you’re using. If there are gaps in documentation, or weird things that show up in your testing, the Rust project *needs* to know about them. The edition guide, which I mentioned last time, continues to be the go-to source for up-to-date information about what is changing. As I noted in the last episode, you should actively try out both the new edition and the rustfix tool! We need those weird corners tested and bugs ironed out!

One particular tool you should help test, if you haven’t tested it previously, is Rustfmt! Rustfmt (spelled rust-f-m-t, sometimes pronounced rustfmt) is an auto-formatting tool, which I’ve mentioned on the show in the past. It’s the kind of tool that might seem a little frustrating at first – “That’s now how I format my code!” – but can end up being a true delight in the long term. I’ve been using not only Rustfmt but also Elm-fmt when I play with Elm and Prettier in the TypeScript ecosystem every day, and I honestly can’t imagine going back to formatting my own code. I just write the actual implementation, and let the auto-formatting handle laying it out in a reasonable way.

There’s [now a release candidate](https://www.ncameron.org/blog/rustfmt-1-rc/) for rustfmt out, so if you haven’t tried it, you should! If you’re using the RLS (for example in VS Code), it’s already available to you. If not, you can install it by doing `rustup install rustfmt-preview`. Once the 1.0 lands, the formatting it does will be frozen for the foreseeable future – so it behooves us to find any really odd corner cases that are left!

## Community

There are also, as always, lots of interesting things happening in the Rust community, and several of these came from listeners: when I say “email me news items or things of interest,” I mean it!

First, I’ve included a link in the show notes to [Nick Cameron’s LinuxConfAu 2018 tutorial](https://www.youtube.com/watch?v=vqavdUGKeb4), which is 90 minutes of goodness covering overall programming techniques in Rust. This is something I’m particularly happy to see because, as I noted in my Rust Belt Rust talk last year, video content has generally been a bit of a gap in the Rust learning story. Thanks to Ted Bedwell for sending me that link!

Another great thing to check out came from Daniel Sockwell, who pointed me to a static site generator called [Gutenberg](https://getgutenberg.io/). For those of you who’ve happened to check out my started-but-very-much-on-hiatus Lightning project, I’m happy to say that you should probably just go look at Gutenberg: it does 98% of what I wanted from Lightning (and the last 2% is all to do with my own very unusual needs around academic writing). If you’ve wanted something like Jekyll or Hugo but in Rust (and, frankly, with a better templating language), Gutenberg might be what you’re looking for.

Last, but *very* much not least: RustConf is this week! I’ll be there, wearing my New Rustacean t-shirt and bearing loads and loads of New Rustacean stickers, so find me and say hi! I am definitely planning on enjoying the talks (they’re going to be good, and I should know because I got to help pick them), but I’m equally looking forward to meeting many of you!

## Outro

Thanks, as always, to this month’s sponsors, including:

- Ramon Buckland
- Matt Rudder
- Marshall Clyburn
- Rob Tsuk
- Dan Abrams
- Nathan Sculli
- Derek Buckley
- Sascha Grunert
- Behnam Esfahbod
- Peter Tillemans
- David W. Allen
- Paul Naranja
- Anthony Deschamps
- Oluseyi Sonaiya
- Damien Stanton
- John Rudnick
- Chip
- Ryan Osial
- Zachary Snyder
- Vesa Khailavirta
- Alexander Payne
- Aaron Turon
- Chris Palmer
- Martin Heuschober
- Raph Levien
- Nick Stevens
- Daniel Collin
- Hans Fjällemark
- Daniel Mason

If you’d like to sponsor the show, you can do so at [patreon.com/newrustacean](https://patreon.com/newrustacean). You can also send a one-off my way; I’ve listed a bunch of options at the show homepage, [newrustacean.com](https://newrustacean.com), which also has show notes for every episode, scripts for most episodes, and transcripts for many of the interviews. Notes for this episode are in your podcast app or at [newrustacean.com/show\_notes/news/rust\_1\_28/](https://newrustacean.com/show_notes/news/rust_1_28/). Ratings, reviews, and recommendations in podcast directories are always appreciated, but even better I love it when you tell people about the show yourself. You can reach me on Twitter @newrustacean or @chriskrycho, leave a comment in the threads on the Rust user forums, Hacker News, Lobsters, or Reddit, or you can send me an email at [hello@newrustacean.com](mailto:hello@newrustacean.com "Rust 1.28").

Until next time, happy coding!