# News: Rust 1.31 – The 2018 Edition, Part II

Hello, I’m Chris Krycho, and this is New Rustacean: a show about the Rust Programming Language and the people who use it. This is a news episode for Rust 1.31 and the 2018 Edition!

This second part of my discussion of the 1.31 release covers the largest of the changes specific to the 2018 Edition. Note: only the largest changes! There are a handful of other small ones that you’re less likely to run into in practice but which are nice improvements—you should certainly read [the Edition Guide](https://doc.rust-lang.org/stable/edition-guide/) in full!

## Sponsor: [Parity Technologies](https://paritytech.io/jobs)

First though, the show’s current sponsor: Parity Technologies!

Parity is advancing the state of the art in decentralized technology, and they’re using Rust to do it, leaning hard on its trifecta of safety, speed, and correctness. They're building cutting-edge tech in areas like WebAssembly and peer-to-peer networking. Two of the larger projects they're working on are: Substrate, a framework for building blockchains, and Polkadot, a platform leveraging blockchain tech for scaling and interoperability in decentralized systems.

And they’re not just *using* Rust, they’re hiring Rust developers! So if you’d like to work on any of these projects, check out their jobs at [paritytech.io/jobs](https://paritytech.io/jobs).

## More 1.31 Both-Edition Features!

Okay, now let’s dig into the remaining changes in the 1.31 release which are in both editions, as well as all the features specific to the 2018 edition!

### Tooling stabilizations

Up first, there are a bunch of tooling-related stabilizations: Clippy, rustfmt, and support for lint attributes in your code.

Up first is Clippy: named for the old paperclip “helper” in Microsoft Office, which would prompt you with little snippets like “It looks like you’re trying to write a letter…,” Clippy provides a bunch of extra lints to make for more maintainable, more idiomatic Rust code. It’s now shipped standard, and it should help you make sure that what you’re writing isn’t just strictly safe in the Rust-makes-sure-it’s-memory-safe category, but also *good* Rust and much more likely to do what you want. Clippy has been around for a long time; but it’s now part of the default tooling that ships with the language. You should turn on the Clippy integration in your editor—you can use it in anything from Vim to Visual Studio Code – and get writing better Rust!

Next up is rustfmt, the automatic code formatting tool. rustfmt has been in development for close on three years now: the repository reports the first commit was in April 2016! Since then, it has gone through a lengthy process of developing both the tool and the official formatting standards—the formatting standards actually got their own RFC process! The result is a really fast, really nice tool that in my experience works really well and produces *great* results 99% of the time and *results I can live with* the last 1% of the time: even when they’re not what I would choose, they’re *fine*—and having an auto-formatter on your code is genuinely great. I’ve used Prettier for auto-formatted JavaScript and TypeScript for the last few years, and even with its quirks, I can’t imagine ever going back to *not* having an auto-formatter. One of my favorite things about it is that I just write stuff out, not worrying about formatting at *all*, because I know when I save the file it’ll get formatted into something nice.

As of Rust 1.31 and the 2018 edition, rustfmt is stable and at 1.0! This means that the tool guarantees backwards compatibility: if you run it on your code, it will consistently give the same results going forward. (During the development period pre-1.0 this was not true.) You can now integrate it into your commit or continuous integration flow and be confident it won’t give you spurious results or a lot of churn and noise!

Finally, there are some nice new language features in the form of tool-level attributes for lints. For both Clippy and rustfmt, there may be specific places where you want to explicitly opt out of the tool’s behavior—you may have specific reasons to format things in a specific way (for example in macros), or you have a specific reason to ignore Clippy’s advice in some spot. Historically, you had to write tool rules with the `#[cfg_attr(...)]` attribute, which was *long*: you had to write `#[cfg_attr(feature = "cargo-clippy", allow(iter_nth)]` to disable the `iter_nth` lint. Now you can just write `#[allow(clippy::iter_nth)]`: so much better!

### `const fn`

The last big feature to stabilize on both the 2015 and 2018 editions is `const fn`. A `const fn` is a function which can has a *constant*-valued expression as its result, and which can therefore be computed at compile time instead of at runtime. (There’s a specific subset of Rust which is currently allowed in `const fn`s to guarantee that you will indeed always get a constant value for the result of the function; I’ll refer you to the [reference](https://doc.rust-lang.org/reference/items/functions.html#const-functions) for details.) The compiler is *allowed* to compute the result of a `const fn` in any context, but is only *required* to do so in [“const contexts”](https://doc.rust-lang.org/reference/const_eval.html#const-context)—that is, contexts where the expression in question must be evaluated at compiler time. The const contexts in the language today are:

- array length expressions, like the `3` after `i32` in `let array: [i32; 3] = [1, 2, 3];`
- repeat length expressions, like the `5` in `[0; 5]`, which produces an array of five elements, all set to the value `0`
- and initializers for constants, statics, and enum discriminants.

That’s a fairly short list at present, so the number of places where the compiler is guaranteed to run a `const fn`—but remember: the compiler is *allowed* to evaluate `const fn`s in lots of places. That list is likely to expand in the future, as is the list of allowed kinds of operations in a `const fn`, but we now have a starting point for this kind of optimization.

And “optimization" is the right description: `const fn` can be pretty big deal for runtime performance! If you have an operation which takes a while to run, but can be reused throughout the life of the program, you can save your users a *lot* of overhead with `const fn` by doing that compilation once on the source machine instead of repeatedly on every user’s machine, and even repeatedly in the runtime of the program. It’s like function inlining but with superpowers.

However, as is nearly always the case for the triangle of runtime performance, safety, and compile time performance, we’re paying for it somewhere: in this case, since the computation happens at compile time, you’re paying for it… at compile time. That’s sometimes exactly the right tradeoff, but as with all tradeoffs, it’s very important you understand that you’r making it so you can make it explicitly! In many cases, the benefit of doing a `const fn` is so small that it’s *not* worth it! However, especially in some of high-performance numeric computing contexts, this is a game-changer, and it has long been one of the reasons to continue preferring C++ over Rust.

## 2018 Edition Features

Finally, let’s talk about the Edition-only features:

- changes to the language syntax,
- improvements to the module system,
- and non-lexical lifetimes!

### Syntax changes

The syntax changes are *very* small. It’s no exaggeration to say that this is one of the smallest sets of “breaking” changes a language has seen. The only *removals* are removing a few new keywords from the set of previously-valid identifiers for item bindings: meaning, since they’re keywords now, you can’t use them to name a function or a variable or what-have-you anymore. Also, there are only a few additions: the new keywords `dyn`, `async`, `await`, and `try`.

<i>But wait</i>, you say! <i>Wasn’t `dyn` already a keyword in the 2015 edition? Didn’t you talk about it back in [the Rust 1.27 episode](https://newrustacean.com/show_notes/news/rust_1_27/index.html), when `dyn Trait` was stabilized?</i> The answer is: *kind of*. I did indeed talk about it then, but `dyn` was *not* a full keyword in the 2015 edition: it was what the Rust reference calls a “weak” keyword—what you’ll sometimes hear referred to as a *contextual* keyword: that is, a keyword that’s not a keyword everywhere, but only in specific locations in the language’s grammar.

Contextual keywords are essentially workarounds for times when you *reaaaally* wish you’d made a keyword before, but you didn’t, but you can *kind of* make it work by saying “it’s a keyword in these specific locations.” The downside to them is that they’re more work for the lexing phase of the compiler, where it turns all the various tokens into their language-level semantics. Is `dyn` a keyword, or just a regular identifier? Well, in the 2015 edition, we don’t know until we see the *next* item! The 2018 Edition promotes it to being a full keyword, so it’s now invalid to use as any other kind of identifier (whereas it was still legal to use in most positions in the 2015 edition).

`async`, `await`, and `try` are now all *reserved* keywords: none of them mean anything on stable Rust today, but the language design team expects they may in the future. `async` and `await` are *definitely* doing to be used as part of the story around asynchronous programming with the `Future` type, and we expect that to stabilize sometime here in the first half of 2019. `try` is a bit murkier: there have been a lot of discussions about ways we might improve error-handling ergonomics, and it’s not yet clear that any of them are winners—`try` is reserved for the 2018 edition to make a number of them *possible*, but it’s an open question whether any changes will happen there.

Now, what if you have a codebase where you heavily used one of these keywords—say, if you were using `async` in a bunch of places as a shorthand for something? Rust has your back here: you don’t have to do a global find-and-replace operation. The `cargo fix --edition` invocation we covered in previous news episodes will rewrite any uses of these new keywords using another new feature in the language: raw identifiers. You can now write an identifier prefixed by `r#`. (This doesn’t come out of nowhere: the format is similar to [the raw string syntax](https://doc.rust-lang.org/stable/reference/tokens.html#string-literals)—which you may never have run into; I’ve linked it in the show notes to look at if you haven’t.) Thus, `cargo fix --edition` will rewrite every use of `async` as an *identifier* as `r#async`. It’s not the prettiest thing in the world, but it solves the problem nicely—and it means you can now use *any* keyword as an identifier, as long as you give it that leading `r#`.

### Module system improvements

Up next: the module system improvements. We covered the first major part of this when it was stabilized with Rust 1.30—so it’s obviously compatible with Rust 2015! The remaining piece ([“uniform paths”](https://doc.rust-lang.org/edition-guide/rust-2018/module-system/path-clarity.html#uniform-paths)) will be part of the 2018 edition, but remains nightly-only. As such, I’ll refer you to [the news episode for 1.30](https://newrustacean.com/show_notes/news/rust_1_29_1_30/index.html) for a full walkthrough on the changes which have already stabilized, and to the *future* for the uniform paths component—but I didn’t want to push out two episodes on the Edition release without at least *mentioning* it! It’s a really big improvement and an important change, so please check out the earlier episode and read [the Edition guide section on the changes](https://doc.rust-lang.org/edition-guide/rust-2018/module-system/path-clarity.html)!

### Non-lexical lifetimes

There is *one* major feature that is landing as part of the 2018 Edition that wasn’t part of previous releases: *non-lexical lifetimes*. This is something the Rust compiler and language team have been working toward for *years*: a massive improvement to the way that lifetime tracking works in the compiler. (As an aside that I think is kind of fun: a lot of this work was unlocked by something I talked about in depth in [the very first dedicated news episode, years ago, when I covered MIR](https://newrustacean.com/show_notes/news/_1/): the mid-level intermediate representation in the compiler.)

It’s worth note, before I dig into the details, that this is *presently* landing in the 2018 Edition only, but the team expects to back-port it to work with 2015 Edition Rust as well.

The name of this feature, <i>non-lexical lifetimes</i>, gives us the pieces we need to understand what’s changing. At some point, we’ll all just thinking of these as “lifetimes” plain and simple, but for right now, they’re named in contrast with how lifetimes have worked historically. To this point, lifetimes were “lexical”: that is, they were the same as lexical scopes—that is, *blocks*. If you borrowed something anywhere in a block—a function, a for loop, a match block, etc.—it was borrowed until the end of the curly braces for that block. This is why one of the workarounds for a fair number of situations you’d run into with the borrow-checker has been to introduce a local block: then the lifetime and borrow analysis the compiler did could understand that something was no longer borrowed at the end of that block, and the rest of your function could do what you wanted.
  
This is super frustrating, though: you as a user can look at it and say, “No, Rust, I’m *done* with this borrow up here! Why can’t you see that I’m no longer borrowing it and let me borrow it again later! Agh!” And the block workaround is ugly and even that doesn’t always get you where you want.

So, for the last several years—this has been an in-progress effort for most of the life of the 2015 Edition!—the team has been working on a new approach to the borrow checker’s understanding of lifetimes. This is tricky because the new version has to get three things right:

1. It has to let through everything that the old borrow checker let through.
2. It has to properly let through as many new things as it can.
3. It has to be sound—that is, it has to rightly uphold all of Rust’s normal memory guarantees.

And point (3) actually trumps points (1) and (2): correctness is the most important thing for the borrow checker. And this isn’t just hypothetical! The new engine identified potential soundness bugs that got through with the old borrow checker—the bugs which triggered the 1.27.1 and 1.27.2 releases earlier this year.

Non-lexical lifetimes work by tracking a much more sophisticated graph of where and how *values* and the *references to them* interact. For the extremely nitty-gritty details, I’ll direct you to a series of blog posts by Niko Matsakis (dating back to April 2016!), which show the evolution of this in detail. At a high level, it’s enough to understand that non-lexical lifetimes use the control-flow graph from the mid-level intermediate representation in the compiler. That control-flow graph makes it possible for the compiler to determine when a given value is live (and thus valid to have a reference to) and when a given reference is live (and thus valid or invalid for there to be *other* references to that data). This is one of the advantages of something like MIR: while you’d never want to *write* something in the extremely long-form style that MIR uses, it makes all these kinds of relationships extremely clear in a way that the rest of the compiler can take advantage of.

There are also a couple of other kinds of “smarter” lifetimes that landed as part of this feature. One of my favorite examples of this that makes it easy to see the improvement is [a simple example](https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=f497716877668a5f0dc904e24ffc207a) I’ve linked from the show notes, where you call `vec.push(vec.len())`. On Rust 2015, this just doesn’t work: `vec.push()` mutably borrows `vec` and then `vec.len` tries to borrow it as well, and the compiler just says *Nope!* In Rust 2018, it works fine, because the compiler can tell that the `vec.len()` effectively happens *before* `vec.push()` does: the temporary value created from `vec.len()` by the compiler is the same as the workaround you’d do yourself by actually writing out an intermediate value before calling `vec.push()`.

There are tons of these kinds of wins, and the net of it is that a lot of things where you historically would have “fought with the borrow checker” you won’t have to anymore!

## Closing

And that’s it! This is a huge release, and I think it sets Rust up really well for the future. Since the 1.0 release, we’ve seen a ton of clean-up and improvement, and this is a great time to introduce Rust to colleagues or friends who haven’t heard about it or who looked at it in the past and decided to wait before diving in.

Thanks to everyone who sponsors the show! This month’s $10-or-more sponsors included:

- Daniel Collin
- Scott Moeller
- Ramon Buckland
- Michael Mc Donnell
- Johan Andersson
- Oluseyi Sonaiya
- Steffen Loen Sunde
- Anthony Deschamps
- Rob Tsuk
- Behnam Esfahbod
- Matt Rudder
- Ryan Osial
- Embark Studios
- Nick Stevens
- Paul Naranja
- John Rudnick
- Daniel Mason
- Nicolas Pochet
- Andrew Dirksen
- Alexander Payne
- Graham Wihlidal
- Jerome Froelich
- beaorn
- Dan Abrams
- Joseph Marhee
- Chip
- Nathan Sculli
- Adam Green
- Chris Palmer
- James Hagans II
- Jonathan Knapp
- Raph Levien
- Peter Tillemans
- Nick Gideo
- Bryan Stitt
- Jako Danar
- Brian McCallister
- Martin Heuschober

If you’d like to sponsor the show, you set up ongoing support at patreon.com/newrustacean, or send a one-off at any of a number of other services listed at newrustacean.com. The website also has scripts and code samples for most of the teaching episodes as well as transcripts for many of the interviews, along with full show notes for every episode. You can find the notes for *this* episode at <newrustacean.com/show_notes/news/rust_1_31/part_2/>.

If you like the show, please help others find it—by telling them about it in person, by sharing it on social media, or by rating and reviewing the show in your favorite podcast directory.

The show is on Twitter @newrustacean, or you can follow me there @chriskrycho. Tweet me with news, topic ideas, etc! You can also respond in the threads on the Rust user forums, Reddit, or Hacker News, or—and this will always be my favorite—just send me an email at hello@newrustacean.com.

Until next time, happy coding!
