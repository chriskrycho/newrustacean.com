# Rayon

## Intro

Hello, my name is Chris Krycho, and this is the New Rustacean podcast – a show about learning the Rust programming language. This is Crates You Should Know: Rayon.

Before I jump into the show, one quick thing to call out in the community which I would normally put in the News episode that'll be coming out later this week, but which are happening before it comes out!

There's a NYC Rust meetup happening on August 31. I don't have a long-term schedule for that meetup, but you can follow up with the organizer, Robert Balicki for more details. Info about time and place is on meetup.com, and I'll have those in the show notes. If you'd like me to call out local meetups, I'd be happy to (though usually in News episodes, once every six weeks, just like Rust releases) – just send me a note on Twitter @newrustacean or email me at hello@newrustacean.com!

Now, into the show!

## Overview

One of the major promises of Rust is that it can do "threads without data races." For anyone who's ever done multithreaded work in most conventional languages, that sounds almost too good to be true. The only approaches which have proven capable of solving this problem are green threads, as you see most prominently these days in Go but which are also closely related to the approach used in Erlang, and trying really, really hard to be really, really careful.

As usual, that "be really, really careful" approach tends not to work out all that well in practice: as hard as you try, inevitably you end up with two different threads trying to access shared memory at the same time and things go badly. One solution to this problem in other contexts has been pure functional programming, leaning on persistent data structure implementations to make the approach ergonomic and workable. But it's difficult to apply those solutions in the high-performance, deterministic contexts many Rust users need.

As usual, Rust's approach to ownership gives us superpowers, and Rayon makes those superpowers easy to use for everyone.

## Using Rayon

Rayon's API is actually two distinct APIs, which you'll choose between depending on what exactly you're doing. Following the crate itself, I'll describe these as the "parallel iterators" API and the "" quite straightforward -- not least because it largely builds on the same core iterator APIs you're already familiar with if you've been using Rust for any time.

### The parallel iterators API

The parallel iterators API is incredibly straightforward to use if you're used to working with Rust's iterator APIs at all, because, well... it's *basically* the same API, with just one important modification! You use one of the new trait methods, `par_iter` and `par_iter_mut`, to take an existing type which can be converted to an iterator and convert it to a *parallel* iterator instead. As is conventional in Rust, the `_mut` suffix on the default name gets you a *mutable*, parallel iterator instead of the default immutable parallel iterator.

Otherwise, the trait provides most of the same functions as you'd find on `std::iter::Iterator`. There are a few that aren't there in Rayon's `ParallelIterator` trait, because they don't map across correctly -- for example, the `next` method is pretty weird to think about in the context of a parallel vs. sequential iterator. In a sequential iterator, it's always clear what the "next" item is; in the parallel case, by definition, the "next" item depends on which thread is asking, and has no definite meaning in the context of defining, say, something you've `map`-ed over in a parallel way.

#### Prelude

There are a bunch of traits which need to be in scope for this. (In a future episode, I'll circle back around and talk a bit about the way different items and scopes interact -- there's a lot to say there!) Rayon uses a pattern you'll see in a number of libraries: a *prelude*. Importing this prelude automatically brings all the traits you'll need into scope. This is one of the only times you *should* use a "glob import", i.e. pulling in everything in a given namespace without qualification. You usually want to pull in a specific name, or in my style more often a given *module*, to avoid clobbering other names in your own local scope. (You can imagine having your own `read` function, for example, and not wanting Rust's `std::io::Read::read` to conflict with it.) But in the case of a prelude -- or at least, a well-designed one -- the names which come in are just things which you'll have problems with if you reuse anyway. And that's the case here. So to most effectively use Rayon, you'll just include this declaration at the top of any module which uses it:

```
use rayon::prelude::*;
```

Once you've done that, everything will *just work*.

### The `join` API

The other API Rayon provides uses a function called `join`, which actually underpins the parallel iterator API. `join` takes two closures and treats them as targets to run in parallel with each other.

### Bugs are still possible

Now, Rust and Rayon save you from *data races*. But data races are only one specific kind of bug you face in dealing with parallel computing. They're an incredibly important class not to have to deal with, and in many ways getting rid of these make it easier to deal with the other ones. However, there are some things you have to watch out for:

- Using 

## Outro

So that's a look at Rayon! I hope you find some neat places to parallelize your code safely—this is, after all, one of the great promises of Rust!

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
- Raph Levien
- and Vesa Khailavirta

As always, thanks as well to everyone who helps other people find the show---whether by just telling a friend, or by rating or reviewing it in iTunes or recommending it in another podcast directory, or by sharing it on social media!

You can find show notes for this episode, including links to docs and more for Rayon, at `newrustacean.com/show_notes/cysk/rayon/`. The show is on Twitter @newrustacean, and I am @chriskrycho. Tweet me with news, topic ideas, etc! You can also respond in the threads on the Rust user forums, Reddit, or Hacker News, or – and this is my favorite – just send me an email at hello@newrustacean.com.

Until next time, happy coding.
