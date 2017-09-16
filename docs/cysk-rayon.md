# Rayon
#podcasts/new rustacean/crates you should know#

## Intro

Hello, my name is Chris Krycho, and this is the New Rustacean podcast – a show about learning the Rust programming language. This is Crates You Should Know: Rayon.

## Overview

One of the major promises of Rust is that it can do "threads without data races." For anyone who's ever done multithreaded work in most conventional languages, that sounds almost too good to be true. The only approaches which have proven capable of solving this is Erlang's functiona, fail-fast model; and trying really, really hard to be really, really careful.

As usual, that "be really, really careful" approach tends not to work out all that well in practice: as hard as you try, inevitably you end up with two different threads trying to access shared memory at the same time and things go badly. One solution to this problem in other contexts has been pure functional programming, leaning on persistent data structure implementations to make the approach ergonomic and workable. But it's difficult to apply those solutions in the high-performance, deterministic contexts many Rust users need.

As usual, Rust's approach to ownership gives us superpowers, and Rayon makes those superpowers easy to use for everyone.

## Using Rayon

Rayon's API is actually two distinct APIs, which you'll choose between depending on what exactly you're doing. Following the crate itself, I'll describe these as the "parallel iterators" API and the `join` API. Actually using Rayon is, as we’ll see, quite straightforward.

### The parallel iterators API

The parallel iterators API is incredibly straightforward to use if you're used to working with Rust's iterator APIs at all, because, well... it's *basically* the same API, with just one important modification! You use one of the new trait methods, `par_iter` and `par_iter_mut`, to take an existing type which can be converted to an iterator and convert it to a *parallel* iterator instead. As is conventional in Rust, the `_mut` suffix on the default name gets you a *mutable*, parallel iterator instead of the default immutable parallel iterator.

Given my constant use of the words "parallel iterator" in the last few sentences there, it's probably no surprise that Rayon's main trait is called (wait for it) `ParallelIterator`. It provides most of the same functions as you'd find on `std::iter::Iterator`. There are a few that aren't there in Rayon's `ParallelIterator` trait, because they don't map across correctly -- for example, the `next` method is pretty weird to think about in the context of a parallel vs. sequential iterator. In a sequential iterator, it's always clear what the "next" item is; in the parallel case, by definition, the "next" item depends on which thread is asking, and has no definite meaning in the context of defining, say, something you've `map`-ed over in a parallel way. `ParallelIterator` also provides a few things that the native iterators *don't* provide—most notably, `for_each` and `for_each_with`, which are basically methods designed to do side-effecting operations (unlike `map` or `map_with`, which are designed to be pure, i.e. to have no side effects).

### Prelude

There are a bunch of traits which need to be in scope for this. (In a future episode, I'll circle back around and talk a bit about the way different items and scopes interact -- there's a lot to say there!) Rayon uses a pattern you'll see in a number of libraries: a *prelude*. Importing this prelude automatically brings all the traits you'll need into scope. This is one of the only times you *should* use a "glob import", i.e. pulling in everything in a given namespace without qualification. You usually want to pull in a specific name, or in my style more often a given *module*, to avoid clobbering other names in your own local scope. (You can imagine having your own `read` function, for example, and not wanting Rust's `std::io::Read::read` to conflict with it.) But in the case of a prelude -- or at least, a well-designed one -- the names which come in are just things which you'll have problems with if you reuse anyway. And that's the case here. So to most effectively use Rayon, you'll just include this declaration at the top of any module which uses it:

```rust
use rayon::prelude::*;
```

Once you've done that, everything will *just work*.

If you’re curious about what exactly the prelude pulls in, docs.rs has you covered, of course. The most important thing it’s imports is the `ParallelIterator` trait, which has to be `use`d (with the actual Rust `use` keyword) before you can, well, *use* it (in the ordinary sense of the word). We’ll talk more later about `use` and scoping, not least because there’s an RFC looking to improve the ergonomics of that right now! If you’re curious about that RFC you should check out episode 10 if the Request for Explanation podcast, where Aaron Turon is on the show to talk about it! For our purposes right now, we just need to know that importing the prelude with that glob `use` does the trick.

### The `join` API

The other API Rayon provides uses a function called `join`, which actually underpins the parallel iterator API. `join` takes two closures and treats them as targets to run in parallel with each other. Notably, they're not *guaranteed* to run in parallel with each other, so it can actually be slightly slower to do things this way than doing them in sequence *if* all CPUs are already busy. The overhead is low. But it's there. This is a big reason to use the parallel iterators API: they use `join` under the covers, but they implement a sensible fall-back strategy for the times when you end up running in sequence instead of in parallel. That's something you have to manage by yourself if you're using the `join` API. Now, the flipside of that is: if you *need* every bit of control about how you will fall back to a sequential approach, `join` is what you want.

Using join is pretty simple from the API point of view. You simply define the two closures to use and call `rayon::join(the_first_closure, the_second_closure)`. Then Rayon takes care of dispatching them – again, in parallel if possible, or in sequence if not.

### Bugs are still possible

Now, Rust and Rayon save you from *data races* and *deadlocks*. You'll never end up in a spot where one thread wrote data right after the other but before a read happened leading to bad results; and you'll never have two threads ending up blocking each other because they're waiting on each other to release resources the other needs. The fact that we have explicit lifetime and ownership tracking as part of the type system makes it possible for Rayon to make that a firm guarantee. And hooray for that!

However, there are some important caveats here. One is that if the closures you use with the Rayon APIs *themselves* use lock-based approaches to concurrency, rather than lifetime- and ownership-based approaches, you can still run into deadlocks.

Another caveat is that data races and deadlocks are not the only kinds of bugs you face in dealing with parallel computing. They're an incredibly important class not to have to deal with, and in many ways getting rid of these make it easier to deal with the other ones. However, you can still shoot yourself in the foot if you use types with *internal mutability*. We talked about these a bit back in episode 16, but we didn't talk a lot about the mechanics of using them—that's *also* on my list of topics to get to in the future. For now, suffice it to say that types like `Cell` and `RefCell` can be *immutable containers* but have *mutable contents*—and yes, there *are* reasons you'd want this! But in the case that you're handing around an externally immutable but internally mutable object, you can *very* easily end up in a very, *very* bad situation. Shared mutability is the root of all evil.

There are ways around this, of course: you can switch to "atomic" types, which only allow access in "atomic", i.e. one-at-a-time, fashion. or find another type which implements Rust's `Send` thread-safety trait. (That's another future topic!) But those have costs and tradeoffs as well. For one, atomic types perform worse than non-atomic types, because there is runtime bookkeeping associated with them. For another, it's still pretty easy to end up with *bugs* here, even if they aren't data-race bugs. Atomic data types are *not* like promise or future types; they don't have any sense of "eventuality" or locked-sequence to them. Trying to keep data synchronized across threads is no joke. And if you add in locking data structures to solve those problems, well… then you're back at the point of having locking data structures and you can have data locks.

## Outro

Those caveats notwithstanding, Rayon is a really helpful tool for parallelization. For many cases, especially for those which are "embarrassingly parallel", you can essentially just swap Rayon's iterators in for the normal Rust iterators, and get parallelization essentially for free. If you haven't given it a spin, you should. Go find some places where your codebase might benefit from parallelism, and then use Rayon to parallelize your code *safely*—this is, after all, one of the great promises of Rust!

### Sponsorship

Thanks to this month's $10-or-more sponsors:

- Anthony Deschamps
- Chris Palmer
- Behnam Esfahbod
- Dan Abrams
- Daniel Collin
- David W. Allen
- Matt Rudder
- Nathan Sculli
- Nick Stevens
- Peter Tillemans
- Philipp Keller
- Olaf Leidinger
- Raph Levien
- and Vesa Khailavirta

As always, thanks as well to everyone who helps other people find the show---whether by just telling a friend, or by rating or reviewing it in iTunes or recommending it in another podcast directory, or by sharing it on social media! If you'd like to sponsor the show, you can do so by pledging monthly at Patreon.com/newrustacean, or by sending a one-off contribution my way at any of a number of services listed at newrustacean.com.

You can find show notes for this episode, including links to docs and more for Rayon, at `newrustacean.com/show_notes/cysk/rayon/`. The show is on Twitter @newrustacean, and I am @chriskrycho. Tweet me with news, topic ideas, etc! You can also respond in the threads on the Rust user forums, Reddit, or Hacker News, or – and this is my favorite – just send me an email at hello@newrustacean.com.

Until next time, happy coding.
