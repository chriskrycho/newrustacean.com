# Rust 1.24

Hello, I’m Chris Krycho and this is New Rustacean: a show about the Rust programming language and the people who use it. This is a news episode for Rust 1.24.

## Rust Release

With apologies for this being over a week late—I’ve been first sick and then swamped by work!—let’s dive right in!

It’s mostly the usual with this release: lots of small changes and wins. There’s one _really_ big win landing, but first, a few highlights from the smaller pieces:

* The Rust compiler now defaults to doing builds with 16 parallel codegen units for release builds. This means that it splits up the LLVM code generation process into 16 pieces and runs them in parallel to whatever extent it can. The upside of this is that build times can be significantly faster, because the LLVM code generation is something like _half_ of the work that the Rust compiler does. The downside is that the compiled executable can be slightly slower, because LLVM doesn’t have quite as much insight into the whole codebase and therefore into some of the optimizations that might be available.

  Since there’s a tradeoff here, this is also a configurable option: you can set the number of codegen units to 1 when doing a build and wait the extra time for the build to finish in exchange for the absolute fastest executable on the other end… or you can trade a little bit of runtime performance for faster rebuild cycles.

* Speaking of performance: the `str::find::<char>` function got an improvement under the hood that should speed up any use of it by a factor of 10 in most scenarios. Yes, 10x faster!

Now, about that _big win_: as of this release, Cargo defaults to using incremental compilation by default for all non-release builds. _Incremental compilation_ means that Rust no longer needs to rebuild every part of your crate just because you added a `println!()` in one module: it only recompiles the things it actually _needs_ to. It does this by storing the results of its previous builds in a _cache_, and then leaning on that wherever possible. Work on this is far from done, but this is a huge step in improving the experience of working with Rust on a day-to-day basis.

In terms of the real-world impact: when you’re using incremental compilation, your _first_ build will as often as not be slower, because it has to create those cache artifacts for use in later builds. However, _after_ that, you can expect to see fairly dramatic decreases in compile times compared to a non-incremental rebuild (a.k.a. what we’ve all had for years now)—compiles might take anywhere from half as long all the way down to a tenth or a fifth as long. And this remains a big focus, so you can expect to see this continue to improve!

Note, again, that this is only enabled for _development_, i.e. non-release builds. For release builds, you want to throw everything at it in the smartest way possible, and that means turning off incremental compilation to give LLVM all those passes it needs with the whole compile artifact together.

The net of all of these changes but _especially_ incremental compilation is that stable Rust should just be dramatically nicer to work with in terms of compile times now.

## 2018 Roadmap RFC

The weeks since the last Rust release have also seen a concerted effort to sort out the game plan for this year’s development cycle. As I noted in the previous news episode, the core team took a different tack this year than previous years, soliciting input not only via the normal RFC process but through blog posts. That proved to be a _smashing_ success: there were something like a hundred blog posts that were part of the conversation! Aaron Turon did the work to read through _all_ of them and synthesized them into a draft of the RFC for the 2018 Roadmap, and then got further input on the text of that. The result is a nice, tight story that particularly picks up on a theme from a bunch of the different blog posts that circulated: basically, making 2018 a continuation of the late-2017 `impl` period, and _finishing_ a lot of the things we already have in flight.

The major _goals_ for 2018 are going to be:

* Ship an epoch release: Rust 2018
* Build resources for intermediate Rustaceans
* Connect and empower Rust’s global community
* Grow Rust’s teams and new leaders within them

Those are pretty high-level, so at the more on-the-ground level, the Rust community is going after these by targeting specific domains:

* Web services
* WebAssembly
* CLI apps
* Embedded devices

These are not the _only_ things going, of course, but they represent the _core_ of the work for this year. Others, including further work on language features that are already in nightly, tooling improvements like continuing to push _hard_ on build times, and so on, are also part of this overall roadmap. So much so that compile times are explicitly called out in the full text of the RFC, for example, even though they’re not one of those areas-of-focus!

## “Rust 2018”

Now, one other major point discussed by the RFC, and which we should talk about in some detail, is the concept of a Rust “epoch.” This is part of Rust’s still-developing strategy for _stability without stagnation._ We’ve gone through almost 3 years of completely backwards-compatible releases now – 25 releases since 1.0! – and the language has evolved an astounding amount over that time.

Rust 2018 marks not a departure but a thoughtful continuation of this theme. It’s an _epoch_ – ‘e’ ‘p’ ‘o’ ‘c’ ‘h’, an era in history – and marks the start of the next chapter for Rust. And Rust 2018 really represents two distinct (though related) things: a _technical_ step, and a _marketing_ step (and the two go together). Let’s talk through what the epoch means in that order: technical, then marketing.

### Technical

Technically-speaking, a Rust epoch has two major things going for it:

1.  The possibility for a backwards-compatible breaking change. Yes, you heard that right, and I’ll explain in a moment.
2.  The ability to bring together the whole ecosystem into a kind of “technical coherence.”

#### Backwards-compatible breaking changes

So: backwards-compatible breaking changes. That sounds… impossible. Let’s start with the motivation, though. One thing that everyone generally recognizes is desirable is the ability to use the lessons learned as you develop a language to continue to improve language design.

There are significant problems with permanent backwards-compatibility: ask C or JavaScript! There are parts of the language that (a) effectively no one uses and (b) effectively everyone wishes no longer existed. However, neither language (and especially JavaScript) wants to introduce breaking changes, and for good reason. Code written decades ago still needs to work! On the web, especially, that’s one of the fundamental commitments that all the standards bodies and browser vendors _mostly_ share (at least as concerns JavaScript): _don’t break backwards compatibility._ Over time, though, this leaves you painted into corners by language design decisions made many years ago that you cannot undo—even when a decade of using those features leaves you convinced they were a bad idea!

But there are also _really_ serious potential problems for making breaking changes to a language. Just ask Python: it’s been a decade since Python 3 came out and the community-wide transition from Python 2 _still_ isn’t over. The transition path, for a huge amount of that time, simply wasn’t smooth enough. It was enormously costly for anyone to convert their library to Python 3, and it was very difficult (especially early on) to ship python libraries that worked with both versions.

So we’re caught on the horns of a dilemma, it seems. Maintain backwards compatibility, even when you see opportunities for really significant improvement, so as not to break your users or split the community—or make a breaking change, which most people agree _does_ improve the language but which imposes major costs for transition and effectively splits the community.

We _think_, as a Rust community, that we have a third way. We can introduce an _epoch_, which is a marker for a set of _parsing-level_ breaking changes, and set a flag on a per-project basis that tells the Rust compiler “Parse and compile this crate with _these_ rules.” At the same time, the Rust compiler will maintain support for all _previous_ epoch’s parse modes. And the compiled output between all epochs will remain compatible with each other.

In practical terms, that means that we can introduce new keywords, stop treating certain old keywords as keywords, and even make tweaks to make certain kinds of other declarations valid which weren’t before and vice versa. But the compiled output of whatever tweaks we make for Rust 2018 will work alongside compiled output from what we might retroactively call the Rust 2015 epoch. So Rust you wrote in a crate that you felt was “finished” back in 2015, on Rust 1.2 or something, can be a dependency of Rust 2018 code. Even more interestingly, if you wanted to go back and update that—perhaps because it was using a long pre-1.0 version of the Serde serialization and deserialization crate—you could continue to write Rust 2015-style code, while updating to use a version of Serde that was written in Rust 2018.

In other words, we get to make small, carefully considered, breaking changes to the surface syntax of the language _without breaking backwards compatibility_. This is Rust’s “stability without stagnation” mantra taken to a whole new level.

This doesn’t cover _every_ kind of backwards-incompatible change you ever might want to make to a language, of course. Deep changes to semantics still aren’t viable. If Rust (for some unfathomable reason) wanted to throw away its core notion of ownership and the borrow-checker and everything else, well… that would be the kind of breakage this kind of thing would not and could not solve. But for the kinds of ergonomic improvements and tweaks that are now fairly obviously desirable based on the community-wide developments that have happened since Rust 1.0 came out… we can solve those!

#### Synchronizing the ecosystem

The other piece of this is the opportunity to bring the ecosystem back into a degree of synchrony. The last three years have seen enormous exploration and growth in both the broader Rust ecosystem and in Rust itself. There has been nearly constant change in the language – and while that’s amazing, it also means that it’s easy for different pieces of the ecosystem to be moving at different speeds. Keeping all of the documentation in sync with the language and standard library, for example, is the kind of thing that’s hard to manage just in terms of technical accuracy. It’s even harder to make sure that everything stays up to date in terms of what is idiomatic.

An epoch gives us a chance to _focus_ on that kind of whole-ecosystem coherence. The push up to Rust 1.0 had a lot of that kind of focus: it was important to present a unified and interesting story about what Rust had to offer now that it had reached a point of _stability_, and to make it as easy as possible for someone coming in to check it out now that it had hit 1.0 to say “Oh, here are how the pieces fit together.”

The goal is something of a repeating process of _intentional_ fragmentation and experimentation followed by coming back together with the shared solutions and results of that fragmentation and experimentation into a coherent whole. That dynamic can be incredibly powerful. We’ve already seen some of the fruit of that in the 2017 `impl` period, and 2018 is shaping up to be a whole year of focus that way. I think it’s going to yield huge dividends.

Among other things, it means landing all those efforts the Rust 2018 Roadmap RFC outlined. It also means polishing up a new version of Rust’s documentation tooling, and landing solid 1.0 versions of rustfmt, improving the reliability and usefulness of the Rust Language Server, and bringing all the documentation up to date (hopefully including the reference!).

### Marketing

The idea of an “epoch” also lets us _use_ all that technical-and-ecosystem coherence to help “sell” Rust. There are lots of people out there who’ve looked at Rust and decided it was too immature for—and in many cases that was a perfectly reasonable decision! But Rust has changed a _lot_ in the last few years—from the availability of stable libraries in the ecosystem at large, to the ease of using learning and using the language itself, to the quality and usability of tooling in editors, and so on. It’s useful in driving adoption to _let people know about that_!

So the other thing Rust 2018 does is give us a place to showcase all those improvements, across a bunch of different fronts. That goes for all sorts of things. One of those I’ve been looking forward to the longest is bringing a unified and coherent design language to all the Rust web properties, and using that design language for a revamped and more useful Rust landing page with better explanations of how to use Rust, how to get involved in the community, and so on. And as we land the Rust 2018 epoch—probably with Rust 1.29, in early fall—we will have a coherent story to tell people about the value proposition of Rust and why they might look at it again if they looked at it once before.

## Outro

So that’s a look at what’s coming in Rust in 2018! Thanks as always to this month’s sponsors—you’re all awesome, and I appreciate you. The sponsors who contributed at least $10 were:

### Sponsors

Thanks, as always, to all of this month's sponsors. Contributors who gave at least $10 included:

* Aaron Turon
* Alexander Payne
* Anthony Deschamps
* Chris Palmer
* Behnam Esfahbod
* Dan Abrams
* Daniel Collin
* David W. Allen
* Hans Fjällemark
* John Rudnick
* Matt Rudder
* Nathan Sculli
* Nick Stevens
* Peter Tillemans
* Olaf Leidinger
* Oluseyi Sonaiya
* Raph Levien
* Vesa Khailavirta
* and Zachary Snyder

If you’d like to sponsor the show, you set up ongoing support at patreon.com/newrustacean, or send a one-off at any of a number of other services listed at newrustacean.com. The website also has scripts and code samples for most of the teaching episodes as well as transcripts for many of the interviews, along with full show notes for every episode. You can find the notes for _this_ episode at \<newrustacean.com/show_notes/cysk/serde/\>.

If you're enjoying New Rustacean, please help others find it – by telling them about it in person, by sharing it on social media, or by rating and reviewing the show in your favorite podcast directory.

The show is on Twitter @newrustacean, or you can follow me there @chriskrycho. Tweet me with news, topic ideas, etc! You can also respond in the threads on the Rust user forums, Reddit, or Hacker News, or—and this will always be my favorite—just send me an email at hello@newrustacean.com.

Until next time, happy coding!
