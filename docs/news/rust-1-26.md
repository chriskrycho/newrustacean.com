# Rust 1.26

Hello, I’m Chris Krycho and this is New Rustacean, a show about the Rust Programming Language and the people who use it. This is a news episode for Rust 1.26.

This is, it turns out, one of the biggest single point releases in Rust’s history—a lot dropped in this release! So let’s dig in.

## Language stabilizations

The biggest set of changes is in the language itself. There are, by my count, *five* major stabilizations in the core language this release. (And there’d have been one more, but it ended up being rolled back because the beta release found some bugs in it!)

### `impl Trait`

First up is the biggest, and what will I think prove to be one of the most important changes to Rust since 1.0: the `impl Trait` feature landing. I’m going to keep my summary of this brief, because a *large* chunk of the next teaching episode – which should be out next weekend – will focus on all the nitty gritty details. The *very* short version is that where historically if you wanted the return type from a function to be a trait instead of a concrete type, you had to wrap it in a pointer. Most typically, you’d see something like `-> Box<Iterator<Item = u32>>`, for the case where what you cared about was not the concrete iterator type but that it was an iterator. This meant two important things:

1. You always had heap allocations, whether or not you actually needed them for any reason *other* than that the type being returned was a trait. Thus the `Box` (or other similar heap-allocated pointer).
2. You always had dynamic dispatch rather than static dispatch (as you have in most other cases, including generics).

`impl Trait` eliminates *both* of those. You can now write – both in function argument position and in return position – `impl Iterator<Item = u32>`. Note that there *are* times when you need a pointer and dynamic dispatch still: whenever you’re returning *different* iterator types from the same function. But for the simple cases, you can now get static dispatch with monomorphization, get rid of the heap pointer, and get much nicer type mismatch errors to boot!

Again, we’ll talk about this a bunch more in a week or so when I get the second part of our traits deep dive episode out.

### `match` improvements

#### dereferences

The next major change to the language – which is *also* a contender for biggest-change-since-1.0! – is new match behavior around dereferences. Historically, when writing a match against a reference, you had to write something like this (assuming a reference to an `Option<String>` type):

```rust,ignore
match optional_name {
    &Some(ref name) => println!("Hello, {}!", name),
    &None => println!("Hello, somebody!"),
}
```

If you missed any part of that – the `&` reference operator before `Some` or `None` or the `ref` before `name` inside the `Some` pattern match – the Rust compiler would tell you *exactly* what to write to make it type-check... but you still had to fill it in yourself. This is more than a papercut; and it shows up constantly in idiomatic Rust, where references are all over the place.

Rust 1.26 changes all of that. Precisely because the compiler *can* figure out exactly what you need here, it now does. It knows you’re matching against references to the enum variants, and it knows that you’re getting a reference to the contents – so now it does that automatically. From 1.26 forward, you’ll write it the way you probably expected to in the first place:

```rust,ignore
match optional_name {
    Some(name) => println!("Hello, {}!", name),
    None => println!("Hello, somebody!"),
}
```

The types (including lifetimes and borrow analysis) are all still exactly what they were before; there’s no lost safety here. It’s just less work for you to do when the compiler already knew exactly what to do!

#### Fixed-entry slice patterns

The other match-related feature that landed in 1.26 is “fixed-entry slice patterns”. This is for the case where you want to pattern-match against a slice, and there are specific scenarios you want to handle. For example, let’s say you wanted to match on an array of numbers and do one thing if it started with 1 and something else otherwise, you can do that now.

### `main` can return `Result`s now

Next up: one of the pain points most people run into early in Rust is with error handling. Nearly anywhere in Rust, you can return a `Result` type, and use the `?` operator to essentially “short-circuit” and just return an error case early. This lets you basically write your local code in a function on the happy path, while still explicitly opting into returning early for error codes.

But not in `main`. Until 1.26, `main` always had to return the unit type. As of 1.26, though, `main` can return a `Result`. The only requirements are:

- The `Ok` type must be `()`
- The `Err` type must implement `Debug` – so that it can be printed.

This eliminates the speed bump for new programmers, of course, but it also means that the rest of us don’t need to have our `main` just call a `run` function which happens to return a `Result` that we ourselves debug print and quit.

### Other new syntax

There are a couple other nice syntax stabilizations as well.

One is inclusive range syntax. We’ve had *exclusive* range syntax for a while: `0..10` would give you 0 through 9 and exclude 10. If you want to include the range “0 to 10, including *both* 0 and 10”, you can now write `0..=10`.

Finally, you can explicitly write `'_` in places where lifetime annotations would normally be elided. I’ll refer you to the [original RFC, #2115](https://github.com/rust-lang/rfcs/blob/master/text/2115-argument-lifetimes.md), for the details here as it’s a pretty nuanced discussion (and a really great RFC!). The new syntax here is just letting us use `_` in lifetimes the same way we can in other contexts in the language already: “we don’t need to name this particular type or value, but we want a placeholder for it.” The kinds of spots you’ll drop this in are where you want to make it explicit that a borrow is happening – for example in a return type where a borrow *could* be elided, but it’s helpful to *show* that it’s happening, but you don’t care about what the “name” of that borrow scope is.

## Standard library stabilizations

There are some really nice additions to the standard library. I’m only touching on a couple of them – specifically, the ones that caught my eye.

First up, the `fs` modules now has a number of really nice convenience methods, each of which let you do fewer module imports *and* skip intermediate allocations:

- `fs::read` reads the whole contents of a file to a buffer of bytes
- `fs::read_to_string` does the same but as a string instead of a buffer
- `fs::write` writes a buffer of bytes directly to a file

Previously, you had to open both the `fs::File` and the `std::io::Read` or `std::io::Write` modules, and then open or create the file prior to writing it. That wasn’t *terrible*, but it’s a lot nicer to just be able to do `let contents = fs::read_to_string(path_to_a_file);` instead.

There’s also a nice new method on the `hash_map::Entry` type: `and_modify`. This lets you update an existing entry if it exists before inserting a new element if it doesn’t. Again: you *could* do this before, but you usually need intermediate terms and/or longer ways of writing it out.

Last of the ones I want to call out is one of those little things that is super surprising we didn’t *already* have it, actually: `process::id` is a simple function get the OS-assigned process ID associated with the process where you call the function.

The [release notes](https://github.com/rust-lang/rust/blob/master/RELEASES.md#version-1260-2018-05-10) have all of the rest, so take a look!

## Community

On the community front, there are a *bunch* of neat things in motion, and I certainly can’t keep up with them all. In fact, your *best* bet… is to listen to a different podcast! I’ve mentioned it before, but wanted to call your attention again to the Rusty Spike, which is a *weekly* update of all the major things going in the Rust ecosystem, from Jonathan Turner, who I interviewed last year.

There were a trio of big, related releases this week: the `regex`, `csv`, and `docopt` crates all hit 1.0! These are, to varying degrees, pretty core to the Rust ecosystem. `regex` in particular is used *everywhere*. The `csv` and `docopt` crates are both quite a bit more specialized, but even so it’s really neat to see them hit 1.0 with the associated commitment to stability. Each of them has been pretty stable for quite some time *anyway*, but pushing things across the finish line like this is an important part of the our story of a *stable* language and ecosystem. And that’s extremely important as we aim to drive increased Rust adoption.

In the WebAssembly world – that beautiful world where I’m actually getting to write Rust a tiny bit for some open source work I’m doing! – there has been a *lot* going on since Rust 1.25 came out. One of the biggest, neatest bits is that there’s an online IDE targeting WebAssembly specifically: [WebAssembly Studio](https://webassembly.studio/). It’s still pretty early, but it’s also pretty neat.

There’s also a really neat tool called `wasm-pack` which lets you take wasm code generated via the `wasm-bindgen` tool and ship it directly to npm for straightforward consumption from other JavaScript. It’s hard to overstate how big a deal this is: the barrier for shipping wasm in the Node world – and, given how much the front end world uses npm, *also* to browsers! – just got way, way, way lower. As someone who is both a massive Rust fanboy *and* a front-end developer, I could not be giddier about this. And in a happy turn, the open source work I’ve just started will get to lean *hard* on this directly.

There are also a few new Rust subteams that have spun up or kicked into high gear since the start of the year, but which I haven’t mentioned previously. There are now working groups for compiler performance, networking tools, command line tools, wasm, and codegen – and that’s just the ones I can dig up or remember!

Also, with apologies for announcing this much later than I meant to, I wanted to note a neat Visual Studio Code plugin from long-time listener Marcin: “Search crates.io”. Once you’ve installed it, if you’re in your `Cargo.toml` file, you can start typing crate names and it’ll do a lookup against the crates.io registry and give you a list of crates and versions. It’s still a bit rough, and doesn’t always do exactly what I want, but it’s a great step in the right direction and I recommend you test it out and give Marcin feedback on improving it. Ultimately, I’d like to see this kind of functionality make its way upstream into the RLS so we can have this functionality everywhere.

## Closing

That’s definitely not everything that has been going on, but it’s as much as we can cover today! It somehow seems fitting to me that this release, pretty much at the 3-year mark since Rust 1.0, was such a huge deal. And there’s still more coming this year. So, as I said last time: buckle up!

Thanks to everyone who sponsors the show. This month’s $10-or-more sponsors included:

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
- Marshall Clyburn
- Nathan Sculli
- Nick Stevens
- Peter Tillemans
- Paul Naranja
- Olaf Leidinger
- Oluseyi Sonaiya
- Ramon Buckland
- Raph Levien
- Vesa Khailavirta
- and Zachary Snyder

If you’d like to sponsor the show, you set up ongoing support at patreon.com/newrustacean, or send a one-off at any of a number of other services listed at newrustacean.com. The website also has scripts and code samples for most of the teaching episodes as well as transcripts for many of the interviews, along with full show notes for every episode. You can find the notes for *this* episode at <https://newrustacean.com/show_notes/news/rust_1_26>.

If you're enjoying New Rustacean, please help others find it – by telling them about it in person, by sharing it on social media, or by rating and reviewing the show in your favorite podcast directory.

The show is on Twitter @newrustacean, or you can follow me there @chriskrycho. Tweet me with news, topic ideas, etc! You can also respond in the threads on the Rust user forums, Reddit, or Hacker News, or—and this will always be my favorite—just send me an email at hello@newrustacean.com.

Until next time, happy coding!