Full script for the episode.

# e022: `Send` and `Sync`

**Note:** In the original (recorded) version of the episode, I said "deadlock" everywhere I meant "data race." Throughout the transcript, I've made the substitution accordingly, but if you're listening to the episode, you'll have to do it mentally. Sorry about that!

## Intro

Hello, I'm Chris Krycho and this is New Rustacean: a show about the Rust Programming Language and the people who use it. This is episode 22: TODO

There are a few major parts of Rust we haven't covered yet, including one of the most important _parts_ of Rust's promise: safe concurrency. So today we're going to dive in and talk about the two traits that are the most important part of Rust's story for enabling safe concurrency: `Send` and `Sync`.

## Background

Multithreaded processing has become increasingly important over the last couple decades, as our ability to speed up single-threaded processing has effectively hit a wall. We _can_ make things go faster, but it takes a lot of power and it generates a lot of heat, and that makes it hard to speed up our programs just by running the processor faster. Starting in the early 2000s, the computer industry began adapting to that by allowing for multi-threaded and multi-processor approaches, which at least in principle lets us parallelize parts of whatever we're working on. But there's a problem: parallelizing things effectively is _hard_. And it's hard in at least two ways—one of which Rust really _can’t_ help us with, but one of which it really can.

* Rust _cannot_ help us with designing algorithms well for parallelization, or even with identifying which specific problems are amenable to parallelization. We still need good old computer science reasoning for that side of things.

* On the other hand, Rust _can_ hep us make sure the design we implement is safe once we _have_ identified candidates for parallelization. In fact, it can do more than help. Rust can actually just eliminate whole classes of parallelization bugs common in other languages, and do so at compile time.

This is basically true of _all_ the ways Rust helps us, to be fair: it doesn’t tell you how to write an algorithm that performs linearly instead of quadratically. It makes it easier to not shoot yourself in the foot with memory errors while you write your algorithm. The same thing goes for parallelism.

So now we know _why_ we care about `Send` and `Sync`—what are they and how do they work?

### Marker traits

The first thing we need to understand is that both `Send` and `Sync` are _marker traits._ This is one of several things I passed over in my previous discussion of traits back in episode 8; we’ll be coming back to many of them in the next few months.

A _marker trait_ is a trait which does not have behavior of its own; it simply specifies that certain kinds of invariants are enforced, which the compiler can then require types to implement before it will allow certain kinds of behavior. These are not _special_ in that they’re not restricted to language built-ins or anything like that. You can implement your own and define what makes that trait hold or not in your own code, and get the same kinds of guarantees. (For an example tied to a recent episode, Diesel actually uses marker traits internally – for example, to [segregate certain kinds of SQL expressions](http://docs.diesel.rs/diesel/expression/trait.NonAggregate.html 'NonAggregate') from each other.)

In the standard library, though, there are four of these market traits: `Copy`, `Send`, `Sized`, and `Sync`. There’s also the `PhantomData` struct type. We’ve already seen `Copy`, in episode 14’s discussion of strings, and we’ll come back to `Sized` and `PhantomData` in the future. For today, we’re just sticking with `Send` and `Sync`.

There are two fundamental things you need to understanding about `Send` and `Sync`.

1. They are automatically implemented for every type they can be by the compiler—they have default implementations, and you have to explicitly opt _out_ of those if you’re somehow doing unsafe behavior yourself. For example, the `Rc` type expressly indicates in its own type definition that it is _not_ `Send` or `Sync`.
2. Closely related to that first point: `Send` and `Sync` are _unsafe traits_. This means that you cannot implement them “safely” – and that’s fine because your safe types will basically get them for free. You need to write the `unsafe` implementations of any `unsafe` type machinery you intend to be `Send` and `Sync` _very_ carefully, to guarantee that the relevant invariants hold (i.e. that you can’t get data races etc.).

We’re not going to dig further into the details of _how_ you set up those invariants on your types today—it really only matters when you’re building your own implementations which are necessarily in the “trust me, I know what I’m doing category” of `unsafe` code, because they’re _unsafe traits_. Instead, we’re just going to talk about what the two traits are and then look at how we can put them to use together. However, you should note that if you _do_ need to implement a new, low-level type which is `Send` or `Sync`, the responsibility is all on you to get the implementation right, as it is with `unsafe` code in general. (We’ll cover `unsafe` probably in early April.)

### `Send` and `Sync`

`Send` is the trait that indicates it’s safe to _move_ data across threads.

`Sync` is the trait that indicates it’s safe to _share_ data across threads.

Put another way: `Send` is about _cross-thread ownership_, and `Sync` is about _cross-thread borrows_. A type `T` needs to be `Send` to be handed to a cross-thread function with a signature like `fn do_something(with_a_t: T)`. A type `T` needs to be `Sync` to be handed to a cross-thread function with a signature like `fn do_something(with_a_t_ref: &T)`.

Once that distinction is in place, the normal rules about mutability and access apply, just as they would in a single-threaded context.

So what makes a type `Send`-able and `Sync`-able?

#### `Send` rules

For a thread to be `Send`, it has to be

For more complex types – structs, enums, and smart pointer wrappers around them – as long as all the pieces that make up the type are _also_ `Send`, the type is automatically `Send`. If any of the struct members or enum variants are _not_ send, the type as a whole not `Send` either.

#### `Sync` rules

The briefest definition of `Sync` – and here I’m stealing straight from the standard library docs, because they’re really good! is that a type `T` is `Sync` when the type of a _reference_ to `T`, `&T`, is `Send`. (This is why we’re talking about the two in the same episode: not only are they closely related conceptually, they’re closely related in terms of their implementation.)

To expand that out a bit, let’s thing about `String`. Remember that a `String` is a smart pointer, which is ultimately just a `Vec` of bytes which has some guarantees in construction so you can be sure that it’s always valid UTF-8. (If you need to brush up on strings, you can go back and listen to [episode 14](http://www.newrustacean.com/show_notes/e014/index.html 'e014: Stringing things along'), where I talked about them in some detail.) When you take a reference to a `String`, you get an `&String`, but most of the time you actually get a reference to an `&str` slice type. If `&str` or the under-the-hood `&Vec` which `&String` is a special case of were _not_ safe for `Send`, then `String` would not be `Sync`. But since string slices and references to `Vec`s are carefully implemented by the standard library to make sure they _are_ thread-safe, you can use them this way.

As a result, the exact same rule applies for `Sync` types as for `Send` types: as long as all components of the type are `Sync`-safe, so is the type itself, no matter how complex; but if any of the parts of the type are _not_ `Sync`-safe, neither is the type itself.

#### Other data structures

Of course, this isn’t all there is to say. We can also combine these with other data structures and traits we’ve talked about in the past, when we need other kinds of behavior. The strategy is to build up our abstractions from the specific, lower-level combinations of types we need. For example, we might need a reference-counted piece of data that’s shared across threads, and which needs to be possibly owned by more than one thread. With today’s context in mind, we now know that we can take a piece of data that’s both `Send` and `Sync` and put it in an `Arc`—an atomically reference-counted smart pointer type, which we talked about briefly in an earlier episode.

Part of what makes `Arc` especially useful is the guarantee we talked about above: any type whose contents are all `Send` or `Sync` are themselves automatically `Send` or `Sync`. That means that any `Arc` which wraps a `Send` and `Sync` type is automatically safe to use across thread boundaries. And the fact that that’s _not_ true for `Rc`—because it’s not an atomic type and doesn’t maintain the right invariants for thread safety itself—is what makes `Arc` appropriate for cross-thread work and `Rc` inappropriate for it. Put in the terms I used earlier in the episode: `Arc`’s (internal, `unsafe`) implementation maintains all the invariants we need; `Rc`’s implementation _doesn’t_.

### Using `Send` and `Sync` data

Once you have a piece of data that is `Send` (and `Sync`) you can use them in contexts where you’re doing multi-threaded work. For a fairly easy-to-understand example, let’s imagine processing hundreds or thousands of Markdown files. Since each file is independent of the others, we decide this problem is ripe for parallelization.

Now, I want to say up front: what I’m about to desscribe is definitely a naïve approach, and in anything remotely approaching a real-world scenario we’d want something like a thread pool to avoid totally hammering our machine. There’s a great discussion of this in ch. 20 of the second edition of the Rust book, so if you want to see how to do this _right_! Props to Steve Klabnik and Carol Nichols-or-Goulding for their write-up there. Listeners who have followed along with previous episodes will note that we could also sort of cheat and just use Rayon for multi-threading parallelism, but that wouldn’t help us see how `Send` and `Sync` come into play more manually.

In this case, we might have a function `markdown_to_html`, which just takes a string buffer – an `&str` – as its argument. We could just iterate over a `Vec` of `String`s representing the Markdown files and pass them to `markdown_to_html` in sequence. But we want to parallelize them, so instead we’d chunk up the vector into something like the number of processor cores on our machine, and then iterate over those chunks. With each of them, we’d then use the `std::thread::spawn` function to spin up a new thread, and pass it a closure to execute. That would read _something_ like this, if we had a current Markdown item named `md`:

```rust,ignore
let first = std::thread::spawn(|| markdown_to_html(&md));
```

We’d do the same with the others, and push them into some other container to look them up later, and once we finished iterating through everything, we’d call `join` on the thread references we saved, and every time we did we’d get back

What’s interesting here isn’t so much the `std::thread` APIs – though it’s nice to have at least mentioned them explicitly on the show now! – as it is the fact that all of this can be _guaranteed_ to be thread-safe by the compiler. If we tried to hand over a mutable reference to, say, the `Vec` itself on each pass through, we’d get the usual Rust complaints about there being more than one mutable reference at the same time – no different than any other context!

But if for some reason (and there are lots of times this might come up!) you were wrapping each Markdown buffer with a type which was thread-unsafe – say, a `Cell` or `RefCell`, with their _non-atomic interior mutability_ – the Rust compiler would say, “NOPE. You can and will end up surprising yourself in a painful way with those via data races or overwriting mutable data or something like that, so you’re not allowed to do it.” When that circumstance comes up, you just have to switch to another strategy – if you need interior mutability, to something like `Mutex` or `RWLock` or one of the `Atomic` types if you’re getting really down into the nitty-gritty.

## Outro

So now we have a pretty good starting idea how to deal with multi-threading and parallelism in Rust. Again, most of the time unless you’re implementing your own abstractions you’re actually better using one of the existing community libraries for parallelism – Rayon, which we’ve talked about in the past, or Futures and Tokio for async I/O, which we’ll talk about in the future – being the big ones. But now you know the machinery those use “under the hood,” and that’s an important part of understanding how they work and what their performance characteristics will be, and therefore when each is appropriate!

### Sponsors

Thanks, as always, to all of this month's sponsors. Contributors who gave at least $10 included:

* Aaron Turon
* Alexander Payne
* Anthony Deschamps
* Chris Palmer
* Christopher Giffard
* Behnam Esfahbod
* Dan Abrams
* Daniel Collin
* David W. Allen
* Guido Hoermann
* Hans Fjällemark
* Hendrik Sollich
* John Rudnick
* Matt Rudder
* Nathan Sculli
* Nick Stevens
* Peter Tillemans
* Olaf Leidinger
* Oluseyi Sonaiya
* Raph Levien
* Shane Utt
* and Vesa Khailavirta

If you're enjoying the show, please let others know about it in person or on social media, rate and review in your favorite podcast directory, or, if you're feeling extra generous, by sending some financial support for the show my way at Patreon.com/newrustacean or as a one-off via any of a number of other services I've listed on the show website: newrustacean.com.

NewRustacean.com also has scripts and code samples for most of the teaching episodes as well as transcripts for many of the interviews, along with full show notes for every episode.

The show is on Twitter @newrustacean, or you can follow me there @chriskrycho. Tweet me with news, topic ideas, etc! You can also respond in the threads on the Rust user forums, Reddit, or Hacker News, or—and this will always be my favorite—just send me an email at hello@newrustacean.com.

Until next time, happy coding!
