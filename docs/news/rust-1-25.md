# News: Rust 1.25

Hello, I’m Chris Krycho and this is New Rustacean: a show about the Rust programming language and the people who use it. This is a news episode for Rust 1.25. Already!

## Follow-up

First things first: some follow-up from the *last* news episode. Between when I recorded that and now, the language we’re using for the “backwards-compatible breaking changes” I discussed then has changed. Instead of “epoch” (or “epoch”) the Rust core team decided to go with “edition.” For one thing, there’s a consistent pronunciation of it! For another, and more importantly, “edition” is a word that far more people understand, and it communicates the right things to people for whom English isn’t their native language. That’s a big deal, and it’s an *especially* big deal when one of the goals for Rust in 2018 is broadening Rust’s international and multicultural reach! “Edition” also gets the idea across more to other communities: it’s more reminiscent in the *right* ways of things like the C++2011 standard.

## Rust 1.25

With that important bit of follow-up out of the way, we can dive in and talk about what’s going on with Rust 1.25!

### Rust proper

One major stabilization for this release is [RFC 1358](https://github.com/rust-lang/rfcs/pull/1358), for `#repr[align(x)]` – which lets you specify the specific number of bytes to use for aligning a given item. This is an important tweak for people writing code that needs to meet specific hardware requirements, or that needs to interoperate with specific compiler output from the GCC or MSVC C and C++ compilers. Most of the time you don’t need this *at all*: the compiler just figures out a good way to lay out the data for you. When you *do* need it, you’ve been able to do this manually for a long time. This feature stabilizing lets you get it for a lot less manual work, though: you can just use an attribute to tell the compiler “Hey, lay this out with this byte alignment instead of whatever you’d normally use,” and trust that the compiler will get it right – no more manual work required for those scenarios. That’s pretty neat! It’s also been a long time coming; the RFC was opened in November 2015, and approved in May 2016. That’s over two years! Slow and steady wins the race!

Two other language features I’ve wanted for a *long time* also landed in this release. Both of them are just little syntactical niceties, but *boy* are they nice.

The first one is that you can now do *nested* path imports, using the curly brace syntax. (I’ve thrown a small example of this in the show notes, so you can see what I mean.) This makes for a much nicer experience when you’re importing a bunch of things from different sub-modules of a given crate. And I believe the rustfmt formatting tool already has smarts in place to handle this so that you can get nicely laid out, easy-to-read nested imports as well.

Another syntactical nicety is that Rust now lets you write leading `|` characters for *each* of the items listed in a given pattern match. I have mixed feelings on this one: in other languages where that’s the case, the leading `|` is what delineates the pattern match. Here, it’s just the separator between multiple terms that all have the same result. However, the option is there so that when you have enough, or complicated enough, items that you have to span multiple lines, you can visually line them up. I’ve dropped a code sample in the show notes for this episode to show why this actually ends up being *less* clear – again, it’s a function of how Rust matches are *different* from the other languages that support this. But it’s there if you want it!

Finally, an old, long-deprecated (and always unstable) attribute for using SIMD has been removed. If you’re a big fan of SIMD, though, don’t worry, because this got removed because we’re getting full SIMD support on stable pretty soon – hopefully in time for the Rust 2018 edition! Some of you are giddy about this; others of you are saying “I know that acronym, I think?”; and others are saying “What in the world is SIMD?” So, a quick overview: the acronym stands for “single instruction, multiple data,” and it’s a way of taking a bunch of different data and running the same instruction on all of them. In the concurrency-vs.-parallelism discussion, this is a kind of *parallelism*: there’s only one *instruction* happening, but’s happening on multiple pieces of data at the same time. SIMD in practice is a set of instructions available on modern compilers that are powerful when you need to apply the same transformation to a *lot* of data. One canonical example is applying the same volume adjustment transform to a bunch of audio samples – because you want to turn this podcast up, for example. You can check out [RFC #2325](https://github.com/rust-lang/rfcs/pull/2325) for discussion of how that stabilization process will go – starting with x86 SIMD instructions, but with the foundation in place to add other platforms and indeed to be portable. This could be a huge win for Rust in performance-critical spaces.

One other sort of “behind-the-scenes” note – this update also included getting Rust using LLVM 6.0.0 – LLVM, if you’re unfamiliar, is the compiler backend for Rust (along with tons of other languages including C, C++, Objective-C, Swift, and many more). This is more than just a nice “keeping up with changes upstream” thing, though that in and of itself is important. It’s also important for fixing bugs around the same SIMD instructions we just talked about, and for using the LLVM WebAssembly back end!

### Cargo

Cargo also got a bit of love this release. One of the more notable changes you might see day-to-day is that when you run `cargo new`, it defaults to generating a binary for you instead of a library. (You can of course still run `cargo new --lib` to get a library.) The thinking here is that we want the first run experience for new users to be as smooth as possible. Experienced users know how to type those couple extra characters to generate a stub for a library, but it’s extra mental work for someone just trying out the language for the first time. After all: what’s the first thing *most* people do when they go to use a new language? Build a small program! Whether that’s hello world or a small command-line tool or something like that, people don’t generally *start* by writing a library, but with small programs. This optimizes for that case, just to make the on-ramp a little smoother.

## Community Goings-On

Of course, there have been a bunch of things happening in the community since the last news episode as well.

- Matthias Endler (who you may remember I interviewed at Rust Belt Rust last year) launched [Hello Rust](https://hello-rust.show)—a video series Matthias describes as “a lighthearted live-programming channel about my journey to become a fearless, more effective Rust programmer.” I’m *extremely* excited to see this out there, as video content is a really helpful way to help people learn the language that we don’t have much of yet!

- Speaking of learning materials: there’s a new Rust book… in Portuguese! The title (translated) is [“Functional and Concurrent Programming in Rust”](https://www.casadocodigo.com.br/pages/sumario-rust-funcional-concorrente) – and I admit to being a little jealous that our Portuguese Rustaceans have a book focused on that topic, as it sounds awesome!

- There’s a neat new load-testing tool you can use with your web applications (Rust or otherwise!) called [drill](https://github.com/fcsonline/drill) – it’s designed to be lighter weight (and maybe faster, too) than many of the existing load-testing solutions out there. If you have a web application in *any* stack and need to see how it behaves under load, you might give drill a look.

- There was also a really fun bit of news announced at EmberConf a couple weeks ago: Rust is also being used to reimplement the VM that runs the Glimmer rendering engine used in Ember.js and Glimmer.js. Yehuda Katz and Alex Crichton have been hammering away at this since last December or so and I’ve mentioned it on the show before, but the fun bit is that *it’s working*. You can actually see the app running with it at schedule-wasm.emberconf.com in any non-Safari browser. (Safari accidentally, and temporarily, broke wasm with its fix for the Spectre and Meltdown vulnerabilities; but it’ll be back soon.)

- Next up – and this one is perhaps my favorite of all of these (though it’s a close competition with the Glimmer bits): I heard last week from a listener in Russia who was inspired by the show, and particularly by the bonus episode with my “Becoming a Contributor” talk, to start collaborating with other Russian Rust users to translate the second edition of the Rust book! How great is that?

- Finally, the RustConf [call for proposals](https://cfp.rustconf.com) is open, and will be until April 13! And I’m on the program committee, so I’ll be reviewing your proposals and coordinating with the rest of the team to decide which talks we’ll be hearing – and I’ll see you there if you’re going in August!

## Closing

That’s a wrap on Rust 1.25. There was a fair bit going on, but just wait till 1.26! The amount of stuff stabilizing this year is going to be pretty crazy, and pretty great. I also expect to have two deep dive episodes on Traits over the next three weeks! 1.25 just… snuck up on me. March was crazy!

Thanks to this month’s $10-or-more sponsors:



- Aaron Turon
- Alexander Payne
- Anthony Deschamps
- Chris Palmer
- Behnam Esfahbod
- Dan Abrams
- Daniel Collin
- David W. Allen
- Derek Buckley
- Hans Fjällemark
- John Rudnick
- Matt Rudder
- Nathan Sculli
- Nick Stevens
- Peter Tillemans
- Paul Naranja
- Olaf Leidinger
- Oluseyi Sonaiya
- Ramon Buckley
- Raph Levien
- Vesa Khailavirta
- and Zachary Snyder

If you’d like to sponsor the show, you set up ongoing support at patreon.com/newrustacean, or send a one-off at any of a number of other services listed at newrustacean.com. The website also has scripts and code samples for most of the teaching episodes as well as transcripts for many of the interviews, along with full show notes for every episode. You can find the notes for _this_ episode at <newrustacean.com/show_notes/news/rust_1_25/>.

If you're enjoying New Rustacean, please help others find it – by telling them about it in person, by sharing it on social media, or by rating and reviewing the show in your favorite podcast directory.

The show is on Twitter @newrustacean, or you can follow me there @chriskrycho. Tweet me with news, topic ideas, etc! You can also respond in the threads on the Rust user forums, Reddit, or Hacker News, or—and this will always be my favorite—just send me an email at hello@newrustacean.com.

Until next time, happy coding!