# News: Rust 1.23

Hello, I’m Chris Krycho and this is New Rustacean: a show about the Rust programming language and the people who use it. This is a news episode for the Rust 1.23 release.

## Rust

### Rust 1.23

As with most Rust releases, there aren’t any huge features landing today! The highlights of the features that _are_ landing, though, are nice improvements. (I say almost the exact same thing every time I talk about upgrades—and that’s the way it should be. It’s _good_ that it’s rare for big features like MIR to land; that’s part of the stability story.)

The most notable feature change here is to do with rustdoc: for most of 2017, the docs team has been working to move rustdoc over to use the `pulldown-cmark` crate, which tracks the CommonMark spec, instead of the prior hoedown C bindings crate. For the last many months, rustdoc on nightly has built both the `pulldown-cmark` version and the Hoedown-based version, compared them, and spit out the differences. Now, stable does the same—in preparation for the final switchover in some future release.

A couple other little niceties in this release:

* Two places where things unexpectedly didn’t work, or didn’t work the way you’d expect – `auto` traits in trait objects, and type-checking for some binary operations not playing out the way you might expect – now _do_ work.

* The compiler now suggests you rename an import if it clashes with other imports. So if you import `MyModule::MyItem` and also import `SomeOtherModule::MyItem`, Rust will now helpfully suggest for the second that you do `import SomeOtherModule::MyItem as OtherMyItem` so that not only do you know what went wrong, you have some idea how to fix it. This is an especially nice change for _new_ Rustaceans: it’s compiler-as-teacher instead of compiler-as-antagonist.

* Rust also displays errors correctly when there are either wide characters or zero-width characters in them. Yes, you heard that right: if you’re not familiar with either, suffice it to say there are printing characters which have _no_ width—modifiers for other characters, usually—and also printing characters with _extra_ width. In either of those circumstances, the beautiful error messages that get printed used to end up misaligned. Now they don’t! (Typography on computers is _really_ hard to consistently get right, it turns out.)

There is also one nice optimization win in this release: the compiler [learned](https://github.com/rust-lang/rust/pull/45380) not to copy some function arguments when they’re rebound to local variables. This can decrease the memory usage in your code by 5–10%, which is no joke! And it turns out this optimization is one of the many wins that we’ve gotten out of the MIR feature—if you need a refresher on what that is, you can go back and take a look at [the first news episode](https://www.newrustacean.com/show_notes/news/_1/index.html), where I covered it in detail.

The standard library saw a bunch of new `impl`s stabilized, as well as a few performance bumps. The [release notes](https://github.com/rust-lang/rust/blob/50989cd98dbef60b0b6f5baa0ce4203ce778adaa/RELEASES.md#version-1230-2018-01-04) have the details; most are sufficiently niche that you’ll be really happy if they do affect you but many of them won’t directly affect the _average_ you, as it were.

### The end of the first impl period

That set of release notes was relatively quiet, which is unsurprising given the end-of-year holidays. But it also doesn’t capture just how much _was_ happening in that period, because a ton of it was features initially landing on nightly or even _starting_ the march toward stabilization, as the first impl period wrapped up!

If you didn’t hear about the `impl` period, it was a concerned effort over the last quarter of 2017 to focus on _implementing_ all the ideas raised throughout the rest of the year. And it was a big success! Contributors all over the ecosystem collaborated to make _huge_ progress on the RFCs opened throughout the rest of the year and to help push a bunch libraries all the way forward to their 1.0 releases.

(One of the most interesting things this included was the beginning of the marker for the next _epoch_ in Rust. If the idea of an “epoch” isn’t something you haven’t heard of, don’t worry: as we get closer to to 2019, when it will become much more important to understand, I’ll dedicate an episode to it.)

Some of the highlights from the work in the language and compiler which caught my eye include:

* the “non-lexical lifetimes” project, which will get rid of several pain points around the borrow-checker today, while also laying the foundation for more sophistication in the lifetime analysis in Rust in general

* incremental compilation, which went through a number of design interations but is now on nightly and tracking toward stable! I’ll be talking about in a _lot_ of detail in the Rust 1.24 episode next month.

* Getting support for the `?` short-circuit return operator in the `main` function (where you haven’t been able to use it historically because it didn’t match with the normal `main` return type).

Besides that, there was a massive amount of effort that went into improving both Rust itself and the surrounding ecosystem. The “libs blitz” involved helping a _ton_ of Rust libraries get to 1.0 quality and indeed to ship their 1.0 versions, as part of the 2017 goal to improve the stability (actual and perceived) the library ecosystem. One of those is the [Diesel ORM](http://diesel.rs/), and you can look forward to hearing an interview about that with Sean Griffin in the next two weeks!

In general, the first `impl` period seems to have been a rousing success, and I’m excited by all the progress the community made. I can’t wait to see how it goes in the next few years as we get better and better at it.

## Other goings-on in the community

Since the last news episode, there have also been some big happenings in the Rust community at large.

### Firefox Quantum

Perhaps the most interesting and “big” pieces of news since late November is that Firefox Quantum shipped. Quantum was a project to replace the entire style rendering engine in Firefox with the highly parallelized implementation built in Rust for Servo. This was a huge effort, but it seems to have paid off: Firefox’s performance got way better—and it really couldn’t have happened without Rust.

As we’ll talk about in the upcoming Episode 22 on `Send` and `Sync`, and as I alluded to when we talked about the `Arc` type back in [episode 15](https://www.newrustacean.com/show_notes/e015/index.html "Not dumb pointers") Rust affords the ability to _know_ that you’re managing multithreaded tasks safely, and that’s something that’s essentially impossible to do in C++: even if you get it all right on the first pass in C++—which you might be able to do if you’re very skilled and a little lucky—_maintaining_ it will be nearly impossible. Mozilla had made two previous attempts at parallelizing the style rendering process, both in C++. They abandoned both, because they couldn’t do it safely and reliably enough. With Rust, they managed it successfully, and pretty quickly. It’s a huge success story, and is one of the first places for Mozilla itself that its bet on Rust over the last decade is paying off. As we say on [my _other_ podcast](http://www.winningslowly.org "Winning Slowly"): doing good work takes time! So props to Mozilla for doing good work with Rust, making a decade-long bet here that is now yielding dividends both for Mozilla and for a host of other people and companies.

### WebAssembly

WebAssembly continues to gain traction in both small and big ways. On the small-and-fun side, I’ve seen several small but quite functional apps making their way around the internet written using Rust and WebAssembly—from an implementation of the [classic asteroids game](https://aochagavia.github.io/blog/rocket---a-rust-game-running-on-wasm/) to [a password generator](https://arkada38.github.io/2017/12/22/a-rust-password-generator-on-wasm/). At a slightly larger scale, there’s a _client-side_ web framework written in Rust, [Yew](https://github.com/DenisKolodin/yew), which leans heavily on ideas from Elm and React—you get the equivalent of [JSX](https://reactjs.org/docs/introducing-jsx.html), just by using Rust’s macro system—and compiles to WebAssembly, using the [stdweb](https://github.com/koute/stdweb) crate to do the small amount of DOM interaction required. It’s young, of course, but it’s quite impressive and I’m really excited for what it heralds.

There’s also—and this is _super_ exciting to me personally because of my day job—a [spike](https://github.com/glimmerjs/glimmer-vm/pull/752) of the Glimmer rendering engine used in Ember.js implemented in Rust and WebAssembly. There’s a very good chance that I’ll be shipping Rust to production in 2018 as a result: not because _I’ll_ be writing Rust myself for work (though boy do I want to!) but because our app will be shipping it as part of the framework code. And that’s really the perfect place for WebAssembly—though in a tech talk I gave at work a few weeks ago, I noted a few other places we might conceivably use it in the future. Here’s hoping, right?

Summing all of that up, there’s a great blog post by Michael Gatozzi called [Rust and the Case for WebAssembly in 2018](https://mgattozzi.com/rust-wasm), where he makes the case that Rust is well-positioned to become the primary language for targeting WebAssembly, because of the same combination of safety and approachability that makes it so attractive to people who were formerly put off by systems programming more generally. I’ve linked that post; you should definitely give it a read!

### 2018

In 2017, the Rust core team used the RFC process to establish goals for the year, and they paid off pretty handsomely. So for 2018, they’re repeating that same approach, but with a new wrinkle: in addition to the RFCs defining the roadmap for the year, they’re soliciting input via blog posts. You can [check out the announcement at blog.rust-lang.org](https://blog.rust-lang.org/2018/01/03/new-years-rust-a-call-for-community-blogposts.html "New Year’s Rust: A Call for Community Blogposts"), but the short version is: the core team recognizes that writing RFCs is _hard_; writing a blog post has a lower bar for entry, and may also be more accessible to other readers in some ways. To quote:

> These can take many forms:
>
> * A post on your personal or company blog
> * A Medium post
> * A GitHub gist
> * Or any other online writing platform you prefer.
>
> We’re looking for posts on many topics:
>
> * Ideas for community programs
> * Language features
> * Documentation improvements
> * Ecosystem needs
> * Tooling enhancements
> * Or anything else Rust related you hope for in 2018

They point to several example posts out there, one of which was Michael Gatozzi’s post on WebAssembly I mentioned just a minute ago! If you have ideas or vision, write it up and link to it on the relevant threads—you can help shape the direction Rust goes in the year ahead!

## Outro

And that’s a wrap—but that’s plenty!

* Aaron Turon
* Alexander Payne
* Anthony Deschamps
* Chris Palmer
* Christopher Giffard
* Behnam Esfahbod
* Dan Abrams
* Daniel Collin
* David W. Allen
* John Rudnick
* Matt Rudder
* Nathan Sculli
* Nick Stevens
* Peter Tillemans
* Olaf Leidinger
* Oluseyi Sonaiya
* Raph Levien
* and Vesa Khailavirta

If you're enjoying the show, please let others know about it in person or on social media, rate and review in your favorite podcast directory, or, if you're feeling extra generous, by sending some financial support for the show my way at Patreon.com/newrustacean or as a one-off via any of a number of other services I've listed on the show website: newrustacean.com.

NewRustacean.com also has scripts and code samples for most of the teaching episodes as well as transcripts for many of the interviews, along with full show notes for every episode.

The show is on Twitter @newrustacean, or you can follow me there @chriskrycho. Tweet me with news, topic ideas, etc! You can also respond in the threads on the Rust user forums, Reddit, or Hacker News, or—and this will always be my favorite—just send me an email at hello@newrustacean.com.

Until next time, happy coding!
